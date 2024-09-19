use crate::result_analyser::helpers::filter_valid_fahrt_weg_and_fahrt_speed;
use time::Duration;
use zusi_xml_lib::xml::zusi::result::{ResultValue, ZusiResult};

#[cfg(test)]
mod tests;
mod helpers;

#[derive(PartialEq, Debug)]
pub enum AnalyseError {
    NoEntries,
    ZeroDistance,
    ZeroDrivingTime,
}

#[derive(PartialEq, Debug)]
pub struct ResultAnalyser<R> {
    result: R,
    // TODO: implement cache
}

impl<R: AsRef<ZusiResult>> ResultAnalyser<R> {
    pub fn new(result: R) -> ResultAnalyser<R> {
        Self {
            result,
        }
    }

    /// Computes the distance for the whole route by using the `fahrt_weg` attribute.
    ///
    /// Throws [AnalyseError::NoEntries] if the [ZusiResult] does not contain any [FahrtEintrag](ResultValue::FahrtEintrag) entries.
    pub fn distance(&self) -> Result<f32, AnalyseError> {
        let result = self.result.as_ref();

        // also need to filter fahrt_speed because of usage in pure_average_speed_by_pure_driving_time
        let filtered_values = filter_valid_fahrt_weg_and_fahrt_speed(result);

        if filtered_values.len() > 0 {
            let ResultValue::FahrtEintrag(first) = filtered_values.first().unwrap();
            let ResultValue::FahrtEintrag(last) = filtered_values.last().unwrap();
            Ok(last.fahrt_weg - first.fahrt_weg)
        } else {
            Err(AnalyseError::NoEntries)
        }
    }

    /// Computes the average speed including idle times by using the overall driving time and distance.
    ///
    /// Throws [AnalyseError::ZeroDrivingTime] if the computed driving time is zero.
    pub fn average_speed(&self) -> Result<f32, AnalyseError> {
        let distance = self.distance()?;
        let driving_time = self.driving_time()?.as_seconds_f32();
        if driving_time == 0.0 {
            Err(AnalyseError::ZeroDrivingTime)
        } else {
            Ok(distance / driving_time)
        }
    }

    /// Computes the average speed excluding idle times.
    ///
    /// Throws [AnalyseError::NoEntries] if the [ZusiResult] does not contain any [FahrtEintrag](ResultValue::FahrtEintrag) entries.
    /// Throws [AnalyseError::ZeroDrivingTime] or [AnalyseError::ZeroDistance] depending on selected algorithm.
    pub fn pure_average_speed(&self, algorithm: PureAverageSpeedAlgorithm) -> Result<f32, AnalyseError> {
        match algorithm {
            PureAverageSpeedAlgorithm::PureDrivingTime => self.pure_average_speed_by_pure_driving_time(),
            PureAverageSpeedAlgorithm::WeightedLocalSpeeds => self.pure_average_speed_by_weighted_local_speeds(),
        }
    }

    /// Computes the average speed excluding idle times.
    ///
    /// Throws [AnalyseError::NoEntries] if the [ZusiResult] does not contain any [FahrtEintrag](ResultValue::FahrtEintrag) entries.
    /// Throws [AnalyseError::ZeroDrivingTime] if the time driven is zero.
    pub fn pure_average_speed_by_pure_driving_time(&self) -> Result<f32, AnalyseError> {
        let distance = self.distance()?;
        let pure_driving_time = self.pure_driving_time()?.as_seconds_f32();
        if pure_driving_time == 0.0 {
            Err(AnalyseError::ZeroDrivingTime)
        } else {
            Ok(distance / pure_driving_time)
        }
    }

    /// Computes the average speed excluding idle times.
    /// For each two [FahrtEintrag](ResultValue::FahrtEintrag) entries, the average speed between is computed.
    /// All these local average speeds will be averaged together weighted by their individual local distance.
    ///
    /// Throws [AnalyseError::NoEntries] if the [ZusiResult] does not contain any [FahrtEintrag](ResultValue::FahrtEintrag) entries.
    /// Throws [AnalyseError::ZeroDistance] if the distance driven is zero.
    pub fn pure_average_speed_by_weighted_local_speeds(&self) -> Result<f32, AnalyseError> {
        let result = self.result.as_ref();

        let filtered_values = filter_valid_fahrt_weg_and_fahrt_speed(result);

        if self.distance()? == 0. {
            Err(AnalyseError::ZeroDistance)
        } else if filtered_values.len() > 1 {
            let mut weighted_speed_sum = 0.;
            for i in 0..filtered_values.len() - 1 {
                let ResultValue::FahrtEintrag(current) = filtered_values.get(i).unwrap();
                let ResultValue::FahrtEintrag(next) = filtered_values.get(i + 1).unwrap();
                let local_average_speed = (current.fahrt_speed + next.fahrt_speed) / 2.;
                let local_driving_time = next.fahrt_zeit - current.fahrt_zeit;
                weighted_speed_sum += local_driving_time.as_seconds_f32() * local_average_speed;
            }
            Ok(weighted_speed_sum / self.pure_driving_time()?.as_seconds_f32())
        } else {
            Err(AnalyseError::NoEntries)
        }
    }

    /// Computes the whole driving time including idle times by using the `fahrt_zeit` attribute.
    ///
    /// Throws [AnalyseError::NoEntries] if the [ZusiResult] does not contain any [FahrtEintrag](ResultValue::FahrtEintrag) entries.
    pub fn driving_time(&self) -> Result<Duration, AnalyseError> {
        let result = self.result.as_ref();
        if result.value.len() > 0 {
            let ResultValue::FahrtEintrag(first) = result.value.first().unwrap();
            let ResultValue::FahrtEintrag(last) = result.value.last().unwrap();
            Ok(last.fahrt_zeit - first.fahrt_zeit)
        } else {
            Err(AnalyseError::NoEntries)
        }
    }

    /// Computes the whole driving time excluding idle times by omitting all periods with zero driving speed.
    ///
    /// Throws [AnalyseError::NoEntries] if the [ZusiResult] does not contain any [FahrtEintrag](ResultValue::FahrtEintrag) entries.
    pub fn pure_driving_time(&self) -> Result<Duration, AnalyseError> {
        let result = self.result.as_ref();

        // also need to filter fahrt_weg because of usage in pure_average_speed_by_pure_driving_time
        let filtered_values = filter_valid_fahrt_weg_and_fahrt_speed(result);

        if filtered_values.len() > 1 {
            let mut driving_time = Duration::seconds(0);
            for i in 0..filtered_values.len() - 1 {
                let ResultValue::FahrtEintrag(current) = filtered_values.get(i).unwrap();
                let ResultValue::FahrtEintrag(next) = filtered_values.get(i + 1).unwrap();
                if current.fahrt_speed > 0. || next.fahrt_speed > 0. {
                    driving_time += next.fahrt_zeit - current.fahrt_zeit;
                }
            }
            Ok(driving_time)
        } else if result.value.len() > 0 {
            Ok(Duration::seconds(0))
        } else {
            Err(AnalyseError::NoEntries)
        }
    }
}

impl<R: AsRef<ZusiResult>> AsRef<ResultAnalyser<R>> for ResultAnalyser<R> {
    fn as_ref(&self) -> &ResultAnalyser<R> {
        &self
    }
}

#[derive(Copy, Clone)]
pub enum PureAverageSpeedAlgorithm {
    PureDrivingTime,
    WeightedLocalSpeeds,
}

impl Default for PureAverageSpeedAlgorithm {
    fn default() -> Self {
        Self::PureDrivingTime
    }
}
