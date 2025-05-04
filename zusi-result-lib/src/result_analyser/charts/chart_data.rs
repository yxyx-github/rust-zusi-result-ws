use time::PrimitiveDateTime;

#[derive(PartialEq, Clone, Debug)]
pub struct ChartData(Vec<ChartDataEntry>);

impl From<Vec<ChartDataEntry>> for ChartData {
    fn from(value: Vec<ChartDataEntry>) -> Self {
        ChartData(value)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct ChartDataEntry {
    pub distance: f32,
    pub time: PrimitiveDateTime,
    pub km: f32,
    pub actual_speed: f32,
    pub track_speed_limit: Option<f32>,
    pub signal_speed_limit: Option<f32>,
    pub train_control_system_speed_limit: Option<f32>,
}