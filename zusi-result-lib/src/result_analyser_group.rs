use std::cell::RefCell;
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
    cache: RefCell<AnalyserGroupCache>,
    _phantom: PhantomData<R>,
}

impl<A: AsRef<ResultAnalyser<R>>, R: AsRef<ZusiResult>> ResultAnalyserGroup<A, R> {
    pub fn new(analysers: Vec<A>) -> Result<ResultAnalyserGroup<A, R>, CreateAnalyserGroupError> {
        if analysers.len() == 0 {
            Err(CreateAnalyserGroupError::NoAnalysers)
        } else {
            Ok(Self {
                analysers,
                cache: RefCell::new(AnalyserGroupCache::new()),
                _phantom: PhantomData,
            })
        }
    }

    /// Computes the sum of the distance values for all routes.
    /// For more details see [distance](ResultAnalyser::distance).
    ///
    /// Errors will be propagated.
    pub fn total_distance(&self) -> Result<f32, AnalyseError> {
        if let Some(value) = &self.cache.borrow().total_distance {
            return Ok(*value);
        }

        let total_distance = self.analysers.iter().try_fold(0., |total_distance, analyser|
            Ok(total_distance + analyser.as_ref().distance()?)
        )?;

        self.cache.borrow_mut().total_distance = Some(total_distance);
        Ok(total_distance)
    }

    /// Computes the average distance per route.
    ///
    /// Errors will be propagated.
    pub fn average_distance(&self) -> Result<f32, AnalyseError> {
        if let Some(value) = &self.cache.borrow().average_distance {
            return Ok(*value);
        }

        // analysers.len() can't be zero due to a check on creation.
        let average_distance = self.total_distance()? / self.analysers.len() as f32;

        self.cache.borrow_mut().average_distance = Some(average_distance);
        Ok(average_distance)
    }

    /// Computes the average speed for all routes including idle times.
    /// For more details see [average_speed](ResultAnalyser::average_speed).
    ///
    /// Errors will be propagated.
    pub fn average_speed(&self) -> Result<f32, AnalyseError> {
        if let Some(value) = &self.cache.borrow().average_speed {
            return Ok(*value);
        }

        if self.total_driving_time()?.is_zero() {
            Err(AnalyseError::ZeroDrivingTime)
        } else {
            let average_speed = self.total_distance()? / self.total_driving_time()?.as_seconds_f32();

            self.cache.borrow_mut().average_speed = Some(average_speed);
            Ok(average_speed)
        }
    }

    /// Computes the average speed for all routes excluding idle times.
    /// The [algorithm](PureAverageSpeedAlgorithm) argument only affects the calculation of the single [analyzers](ResultAnalyser), not the [AnalyzerGroup](ResultAnalyserGroup) itself.
    /// For more details see [pure_average_speed](ResultAnalyser::pure_average_speed)
    ///
    /// Errors will be propagated.
    pub fn pure_average_speed(&self, algorithm: PureAverageSpeedAlgorithm) -> Result<f32, AnalyseError> {
        match algorithm {
            PureAverageSpeedAlgorithm::PureDrivingTime => self.pure_average_speed_by_pure_driving_time(),
            PureAverageSpeedAlgorithm::WeightedLocalSpeeds => self.pure_average_speed_by_weighted_local_speeds(),
        }
    }

    /// Computes the average speed for all routes excluding idle times using the [PureDrivingTime](PureAverageSpeedAlgorithm::PureDrivingTime) algorithm.
    ///
    /// Errors will be propagated.
    fn pure_average_speed_by_pure_driving_time(&self) -> Result<f32, AnalyseError> {
        if let Some(value) = &self.cache.borrow().pure_average_speed_by_pure_driving_time {
            return Ok(*value);
        }

        if self.total_pure_driving_time()?.is_zero() {
            Err(AnalyseError::ZeroDrivingTime)
        } else {
            let pure_average_speed = self.total_distance()? / self.total_pure_driving_time()?.as_seconds_f32();

            self.cache.borrow_mut().pure_average_speed_by_pure_driving_time = Some(pure_average_speed);
            Ok(pure_average_speed)
        }
    }

    /// Computes the average speed for all routes excluding idle times using the [WeightedLocalSpeeds](PureAverageSpeedAlgorithm::WeightedLocalSpeeds) algorithm.
    ///
    /// Errors will be propagated.
    fn pure_average_speed_by_weighted_local_speeds(&self) -> Result<f32, AnalyseError> {
        if let Some(value) = &self.cache.borrow().pure_average_speed_by_weighted_local_speeds {
            return Ok(*value);
        }

        let weighted_speed_sum = self.analysers.iter().try_fold(0., |weighted_speed_sum, analyser|
            Ok(
                weighted_speed_sum
                    + analyser.as_ref().pure_driving_time()?.as_seconds_f32()
                    * analyser.as_ref().pure_average_speed(PureAverageSpeedAlgorithm::WeightedLocalSpeeds)?
            )
        )?;

        let pure_average_speed = weighted_speed_sum / self.total_pure_driving_time()?.as_seconds_f32();

        self.cache.borrow_mut().pure_average_speed_by_weighted_local_speeds = Some(pure_average_speed);
        Ok(pure_average_speed)
    }

    /// Computes the sum of the driving times including idle times for all routes.
    /// For more details see [driving_time](ResultAnalyser::driving_time).
    ///
    /// Errors will be propagated.
    pub fn total_driving_time(&self) -> Result<Duration, AnalyseError> {
        if let Some(value) = &self.cache.borrow().total_driving_time {
            return Ok(*value);
        }

        let total_driving_time = self.analysers.iter().try_fold(Duration::seconds(0), |total_driving_time, analyser|
            Ok(total_driving_time + analyser.as_ref().driving_time()?)
        )?;

        self.cache.borrow_mut().total_driving_time = Some(total_driving_time);
        Ok(total_driving_time)
    }

    /// Computes the sum of the driving times excluding idle times for all routes.
    /// For more details see [pure_driving_time](ResultAnalyser::pure_driving_time).
    ///
    /// Errors will be propagated.
    pub fn total_pure_driving_time(&self) -> Result<Duration, AnalyseError> {
        if let Some(value) = &self.cache.borrow().total_pure_driving_time {
            return Ok(*value);
        }

        let total_pure_driving_time = self.analysers.iter().try_fold(Duration::seconds(0), |total_pure_driving_time, analyser|
            Ok(total_pure_driving_time + analyser.as_ref().pure_driving_time()?)
        )?;

        self.cache.borrow_mut().total_pure_driving_time = Some(total_pure_driving_time);
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
