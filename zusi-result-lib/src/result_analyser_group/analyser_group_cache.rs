use time::Duration;

#[derive(PartialEq, Debug)]
pub struct AnalyserGroupCache {
    pub total_distance: Option<f32>,
    pub average_distance: Option<f32>,
    pub average_speed: Option<f32>,
    pub pure_average_speed_by_pure_driving_time: Option<f32>,
    pub pure_average_speed_by_weighted_local_speeds: Option<f32>,
    pub total_driving_time: Option<Duration>,
    pub total_pure_driving_time: Option<Duration>,
}

impl AnalyserGroupCache {
    pub fn new() -> AnalyserGroupCache {
        Self {
            total_distance: None,
            average_distance: None,
            average_speed: None,
            pure_average_speed_by_pure_driving_time: None,
            pure_average_speed_by_weighted_local_speeds: None,
            total_driving_time: None,
            total_pure_driving_time: None,
        }
    }
}
