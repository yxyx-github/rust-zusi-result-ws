use std::cell::RefCell;
use crate::result_analyser::analyser_cache::AnalyserCache;
use crate::result_analyser::helpers::{filter_valid_fahrt_weg_and_fahrt_speed, round_primitive_date_time};
use crate::result_analyser::schedule::{Schedule, ScheduleEntry};
use time::Duration;
use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::FahrtTyp;
use zusi_xml_lib::xml::zusi::result::ZusiResult;

#[cfg(test)]
mod tests;
mod helpers;
mod analyser_cache;
mod schedule;

#[derive(PartialEq, Debug)]
pub enum AnalyseError {
    NoEntries,
    ZeroDistance,
    ZeroDrivingTime,
}

#[derive(PartialEq, Debug)]
pub struct ResultAnalyser<R> {
    result: R,
    cache: RefCell<AnalyserCache>,
}

impl<R: AsRef<ZusiResult>> ResultAnalyser<R> {
    pub fn new(result: R) -> ResultAnalyser<R> {
        Self {
            result,
            cache: RefCell::new(AnalyserCache::new()),
        }
    }

    /// Computes the distance for the whole route by using the `fahrt_weg` attribute.
    ///
    /// Throws [AnalyseError::NoEntries] if the [ZusiResult] does not contain any [FahrtEintrag](ResultValue::FahrtEintrag) entries.
    pub fn distance(&self) -> Result<f32, AnalyseError> {
        if let Some(value) = &self.cache.borrow().distance {
            return Ok(*value);
        }

        let result = self.result.as_ref();

        // also need to filter fahrt_speed because of usage in pure_average_speed_by_pure_driving_time
        let filtered_entries = filter_valid_fahrt_weg_and_fahrt_speed(result);

        if filtered_entries.len() > 0 {
            let first = filtered_entries.first().unwrap();
            let last = filtered_entries.last().unwrap();
            let distance = last.fahrt_weg - first.fahrt_weg;

            self.cache.borrow_mut().distance = Some(distance);
            Ok(distance)
        } else {
            Err(AnalyseError::NoEntries)
        }
    }

    /// Computes the average speed including idle times by using the overall driving time and distance.
    ///
    /// Throws [AnalyseError::ZeroDrivingTime] if the computed driving time is zero.
    pub fn average_speed(&self) -> Result<f32, AnalyseError> {
        if let Some(value) = &self.cache.borrow().average_speed {
            return Ok(*value);
        }

        if self.driving_time()?.is_zero() {
            Err(AnalyseError::ZeroDrivingTime)
        } else {
            let average_speed = self.distance()? / self.driving_time()?.as_seconds_f32();

            self.cache.borrow_mut().average_speed = Some(average_speed);
            Ok(average_speed)
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

    /// Computes the average speed excluding idle times using the [pure_driving_time](ResultAnalyser::pure_driving_time).
    ///
    /// Throws [AnalyseError::NoEntries] if the [ZusiResult] does not contain any [FahrtEintrag](ResultValue::FahrtEintrag) entries.
    /// Throws [AnalyseError::ZeroDrivingTime] if the time driven is zero.
    fn pure_average_speed_by_pure_driving_time(&self) -> Result<f32, AnalyseError> {
        if let Some(value) = &self.cache.borrow().pure_average_speed_by_pure_driving_time {
            return Ok(*value);
        }

        let distance = self.distance()?;
        let pure_driving_time = self.pure_driving_time()?.as_seconds_f32();
        if pure_driving_time == 0.0 {
            Err(AnalyseError::ZeroDrivingTime)
        } else {
            let pure_average_speed = distance / pure_driving_time;

            self.cache.borrow_mut().pure_average_speed_by_pure_driving_time = Some(pure_average_speed);
            Ok(pure_average_speed)
        }
    }

    /// Computes the average speed excluding idle times.
    /// For each two [FahrtEintrag](ResultValue::FahrtEintrag) entries, the average speed between is computed.
    /// All these local average speeds will be averaged together weighted by their individual local distance.
    ///
    /// Throws [AnalyseError::NoEntries] if the [ZusiResult] does not contain any [FahrtEintrag](ResultValue::FahrtEintrag) entries.
    /// Throws [AnalyseError::ZeroDistance] if the distance driven is zero.
    fn pure_average_speed_by_weighted_local_speeds(&self) -> Result<f32, AnalyseError> {
        if let Some(value) = &self.cache.borrow().pure_average_speed_by_weighted_local_speeds {
            return Ok(*value);
        }

        let distance = self.distance()?;

        let result = self.result.as_ref();

        let filtered_values = filter_valid_fahrt_weg_and_fahrt_speed(result);

        if distance == 0. {
            Err(AnalyseError::ZeroDistance)
        } else if filtered_values.len() > 1 {
            let weighted_speed_sum = filtered_values.windows(2).fold(0., |weighted_speed_sum, window| {
                let (current, next) = (window[0], window[1]);
                let local_average_speed = (current.fahrt_speed + next.fahrt_speed) / 2.;
                let local_driving_time = next.fahrt_zeit - current.fahrt_zeit;
                weighted_speed_sum + local_driving_time.as_seconds_f32() * local_average_speed
            });
            let pure_average_speed = weighted_speed_sum / self.pure_driving_time()?.as_seconds_f32();

            self.cache.borrow_mut().pure_average_speed_by_weighted_local_speeds = Some(pure_average_speed);
            Ok(pure_average_speed)
        } else {
            Err(AnalyseError::NoEntries)
        }
    }

    /// Computes the whole driving time including idle times by using the `fahrt_zeit` attribute.
    ///
    /// Throws [AnalyseError::NoEntries] if the [ZusiResult] does not contain any [FahrtEintrag](ResultValue::FahrtEintrag) entries.
    pub fn driving_time(&self) -> Result<Duration, AnalyseError> {
        if let Some(value) = &self.cache.borrow().driving_time {
            return Ok(*value);
        }

        let result = self.result.as_ref();
        if result.fahrt_eintraege.len() > 0 {
            let first = result.fahrt_eintraege.first().unwrap();
            let last = result.fahrt_eintraege.last().unwrap();
            let driving_time = last.fahrt_zeit - first.fahrt_zeit;

            self.cache.borrow_mut().driving_time = Some(driving_time);
            Ok(driving_time)
        } else {
            Err(AnalyseError::NoEntries)
        }
    }

    /// Computes the whole driving time excluding idle times by omitting all periods with zero driving speed.
    ///
    /// Throws [AnalyseError::NoEntries] if the [ZusiResult] does not contain any [FahrtEintrag](ResultValue::FahrtEintrag) entries.
    pub fn pure_driving_time(&self) -> Result<Duration, AnalyseError> {
        if let Some(value) = &self.cache.borrow().pure_driving_time {
            return Ok(*value);
        }

        let result = self.result.as_ref();

        // also need to filter fahrt_weg because of usage in pure_average_speed_by_pure_driving_time
        let filtered_values = filter_valid_fahrt_weg_and_fahrt_speed(result);

        if filtered_values.len() > 1 {
            let pure_driving_time = filtered_values.windows(2).fold(Duration::seconds(0), |mut pure_driving_time, window| {
                let (current, next) = (window[0], window[1]);
                if current.fahrt_speed > 0. || next.fahrt_speed > 0. {
                    pure_driving_time += next.fahrt_zeit - current.fahrt_zeit;
                }
                pure_driving_time
            });
            self.cache.borrow_mut().pure_driving_time = Some(pure_driving_time);
            Ok(pure_driving_time)
        } else if result.fahrt_eintraege.len() > 0 {
            Ok(Duration::seconds(0))
        } else {
            Err(AnalyseError::NoEntries)
        }
    }

    pub fn schedule(&self) -> Result<Schedule, AnalyseError> {
        if let Some(value) = &self.cache.borrow().schedule {
            return Ok((*value).clone());
        }

        let result = self.result.as_ref();

        let schedule: Schedule = filter_valid_fahrt_weg_and_fahrt_speed(result).into_iter().fold(
            (vec![], false),
            |(mut entries, mut missing_departure), fahrt_eintrag| {
                match (
                    &fahrt_eintrag.fahrt_typ,
                    &fahrt_eintrag.fahrt_zeit,
                    &fahrt_eintrag.fahrt_fpl_ank,
                    &fahrt_eintrag.fahrt_fpl_abf,
                    &fahrt_eintrag.fahrt_text,
                    &fahrt_eintrag.fahrt_speed,
                ) {
                    (FahrtTyp::Planhalt, fahrt_zeit, Some(ank), Some(abf), text, _) => {
                        entries.push(ScheduleEntry {
                            planned_arrival: round_primitive_date_time((*ank).into()),
                            planned_departure: round_primitive_date_time((*abf).into()),
                            actual_arrival: fahrt_zeit.clone(),
                            actual_departure: round_primitive_date_time((*abf).into()),
                            name: text.clone(),
                        });
                        missing_departure = true;
                    }
                    (_, fahrt_zeit, _, _, _, speed) if *speed > 0. && missing_departure == true => {
                        if let Some(entry) = entries.last_mut() {
                            entry.actual_departure = fahrt_zeit.clone();
                            missing_departure = false;
                        }
                    }
                    _ => {}
                };
                (entries, missing_departure)
            }
        ).0.into();

        self.cache.borrow_mut().schedule = Some(schedule.clone());
        Ok(schedule)
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
