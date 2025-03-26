use time::Duration;

#[derive(PartialEq, Debug)]
pub struct AnalyserCache {
    pub distance: Option<f32>,
    pub average_speed: Option<f32>,
    pub pure_average_speed_by_pure_driving_time: Option<f32>,
    pub pure_average_speed_by_weighted_local_speeds: Option<f32>,
    pub driving_time: Option<Duration>,
    pub pure_driving_time: Option<Duration>,
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
        }
    }
}
