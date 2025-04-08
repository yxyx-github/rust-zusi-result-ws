use std::marker::PhantomData;
use time::Duration;
use zusi_xml_lib::xml::zusi::result::ZusiResult;

use crate::result_analyser::{AnalyseError, PureAverageSpeedAlgorithm, ResultAnalyser};
use crate::result_analyser_group::analyser_group_cache::AnalyserGroupCache;

#[cfg(test)]
mod tests;
mod analyser_group_cache;

#[derive(PartialEq, Debug)]
pub enum CreateAnalyserGroupError {
    NoAnalysers,
}

#[derive(PartialEq, Debug)]
pub struct ResultAnalyserGroup<A, R> {
    analysers: Vec<A>,
    cache: AnalyserGroupCache,
    _phantom: PhantomData<R>,
}

impl<A: AsMut<ResultAnalyser<R>>, R: AsRef<ZusiResult>> ResultAnalyserGroup<A, R> {
    pub fn new(analysers: Vec<A>) -> Result<ResultAnalyserGroup<A, R>, CreateAnalyserGroupError> {
        if analysers.len() == 0 {
            Err(CreateAnalyserGroupError::NoAnalysers)
        } else {
            Ok(Self {
                analysers,
                cache: AnalyserGroupCache::new(),
                _phantom: PhantomData,
            })
        }
    }

    /// Computes the sum of the distance values for all routes.
    /// For more details see [distance](ResultAnalyser::distance).
    ///
    /// Errors will be propagated.
    pub fn total_distance(&mut self) -> Result<f32, AnalyseError> {
        if let Some(value) = &self.cache.total_distance {
            return Ok(*value);
        }

        let mut total_distance = 0.;

        for analyser in self.analysers.iter_mut() {
            total_distance += analyser.as_mut().distance()?;
        }

        self.cache.total_distance = Some(total_distance);
        Ok(total_distance)
    }

    /// Computes the average distance per route.
    ///
    /// Errors will be propagated.
    pub fn average_distance(&mut self) -> Result<f32, AnalyseError> {
        if let Some(value) = &self.cache.average_distance {
            return Ok(*value);
        }

        // analysers.len() can't be zero due to a check on creation.
        let average_distance = self.total_distance()? / self.analysers.len() as f32;

        self.cache.average_distance = Some(average_distance);
        Ok(average_distance)
    }

    /// Computes the average speed for all routes including idle times.
    /// For more details see [average_speed](ResultAnalyser::average_speed).
    ///
    /// Errors will be propagated.
    pub fn average_speed(&mut self) -> Result<f32, AnalyseError> {
        if let Some(value) = &self.cache.average_speed {
            return Ok(*value);
        }

        if self.total_driving_time()?.is_zero() {
            Err(AnalyseError::ZeroDrivingTime)
        } else {
            let average_speed = self.total_distance()? / self.total_driving_time()?.as_seconds_f32();

            self.cache.average_speed = Some(average_speed);
            Ok(average_speed)
        }
    }

    /// Computes the average speed for all routes excluding idle times.
    /// The [algorithm](PureAverageSpeedAlgorithm) argument only affects the calculation of the single [analyzers](ResultAnalyser), not the [AnalyzerGroup](ResultAnalyserGroup) itself.
    /// For more details see [pure_average_speed](ResultAnalyser::pure_average_speed)
    ///
    /// Errors will be propagated.
    pub fn pure_average_speed(&mut self, algorithm: PureAverageSpeedAlgorithm) -> Result<f32, AnalyseError> {
        match algorithm {
            PureAverageSpeedAlgorithm::PureDrivingTime => self.pure_average_speed_by_pure_driving_time(),
            PureAverageSpeedAlgorithm::WeightedLocalSpeeds => self.pure_average_speed_by_weighted_local_speeds(),
        }
    }

    /// Computes the average speed for all routes excluding idle times using the [PureDrivingTime](PureAverageSpeedAlgorithm::PureDrivingTime) algorithm.
    ///
    /// Errors will be propagated.
    fn pure_average_speed_by_pure_driving_time(&mut self) -> Result<f32, AnalyseError> {
        if let Some(value) = &self.cache.pure_average_speed_by_pure_driving_time {
            return Ok(*value);
        }

        if self.total_pure_driving_time()?.is_zero() {
            Err(AnalyseError::ZeroDrivingTime)
        } else {
            let pure_average_speed = self.total_distance()? / self.total_pure_driving_time()?.as_seconds_f32();

            self.cache.pure_average_speed_by_pure_driving_time = Some(pure_average_speed);
            Ok(pure_average_speed)
        }
    }

    /// Computes the average speed for all routes excluding idle times using the [WeightedLocalSpeeds](PureAverageSpeedAlgorithm::WeightedLocalSpeeds) algorithm.
    ///
    /// Errors will be propagated.
    fn pure_average_speed_by_weighted_local_speeds(&mut self) -> Result<f32, AnalyseError> {
        if let Some(value) = &self.cache.pure_average_speed_by_weighted_local_speeds {
            return Ok(*value);
        }

        let mut weighted_speed_sum = 0.;
        for analyser in self.analysers.iter_mut() {
            weighted_speed_sum += analyser.as_mut().pure_driving_time()?.as_seconds_f32() * analyser.as_mut().pure_average_speed(PureAverageSpeedAlgorithm::WeightedLocalSpeeds)?;
        }

        let pure_average_speed = weighted_speed_sum / self.total_pure_driving_time()?.as_seconds_f32();

        self.cache.pure_average_speed_by_weighted_local_speeds = Some(pure_average_speed);
        Ok(pure_average_speed)
    }

    /// Computes the sum of the driving times including idle times for all routes.
    /// For more details see [driving_time](ResultAnalyser::driving_time).
    ///
    /// Errors will be propagated.
    pub fn total_driving_time(&mut self) -> Result<Duration, AnalyseError> {
        if let Some(value) = &self.cache.total_driving_time {
            return Ok(*value);
        }

        let mut total_driving_time = Duration::seconds(0);

        for analyser in self.analysers.iter_mut() {
            total_driving_time += analyser.as_mut().driving_time()?;
        }

        self.cache.total_driving_time = Some(total_driving_time);
        Ok(total_driving_time)
    }

    /// Computes the sum of the driving times excluding idle times for all routes.
    /// For more details see [pure_driving_time](ResultAnalyser::pure_driving_time).
    ///
    /// Errors will be propagated.
    pub fn total_pure_driving_time(&mut self) -> Result<Duration, AnalyseError> {
        if let Some(value) = &self.cache.total_pure_driving_time {
            return Ok(*value);
        }

        let mut total_pure_driving_time = Duration::seconds(0);

        for analyser in self.analysers.iter_mut() {
            total_pure_driving_time += analyser.as_mut().pure_driving_time()?;
        }

        self.cache.total_pure_driving_time = Some(total_pure_driving_time);
        Ok(total_pure_driving_time)
    }
}

impl<R: AsRef<ZusiResult>> TryFrom<Vec<R>> for ResultAnalyserGroup<ResultAnalyser<R>, R> {
    type Error = CreateAnalyserGroupError;

    fn try_from(value: Vec<R>) -> Result<Self, Self::Error> {
        ResultAnalyserGroup::new(
            value.into_iter().map(|r| ResultAnalyser::new(r)).collect()
        )
    }
}
