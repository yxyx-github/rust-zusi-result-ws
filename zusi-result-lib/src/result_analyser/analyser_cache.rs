use crate::result_analyser::schedule::Schedule;
use time::Duration;
use crate::result_analyser::charts::chart_data::ChartData;

#[derive(PartialEq, Debug)]
pub struct AnalyserCache {
    pub distance: Option<f32>,
    pub average_speed: Option<f32>,
    pub pure_average_speed_by_pure_driving_time: Option<f32>,
    pub pure_average_speed_by_weighted_local_speeds: Option<f32>,
    pub driving_time: Option<Duration>,
    pub pure_driving_time: Option<Duration>,
    pub schedule: Option<Schedule>,
    pub chart_data: Option<ChartData>,
}

impl AnalyserCache {
    pub fn new() -> AnalyserCache {
        Self {
            distance: None,
            average_speed: None,
            pure_average_speed_by_pure_driving_time: None,
            pure_average_speed_by_weighted_local_speeds: None,
            driving_time: None,
            pure_driving_time: None,
            schedule: None,
            chart_data: None,
        }
    }
}
