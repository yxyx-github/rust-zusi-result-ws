use time::PrimitiveDateTime;

#[derive(PartialEq, Clone, Debug)]
pub struct ChartData(Vec<ChartDataEntry>);

impl ChartData {
    pub fn entries(&self) -> &Vec<ChartDataEntry> {
        &self.0
    }
}

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
    pub actual_speed: Speed,
    pub track_speed_limit: Option<Speed>,
    pub signal_speed_limit: Option<Speed>,
    pub train_control_system_speed_limit: Option<Speed>,
}

impl ChartDataEntry {
    pub fn max_speed_value(&self) -> Speed {
        [
            self.actual_speed,
            self.track_speed_limit.unwrap_or_default(),
            self.signal_speed_limit.unwrap_or_default(),
            self.train_control_system_speed_limit.unwrap_or_default(),
        ].into_iter().max_by(|a, b|
            a.0.partial_cmp(&b.0).unwrap()
        ).unwrap()
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Speed(f32);

impl Speed {
    pub fn from_meters_per_second(value: f32) -> Self {
        Self(value)
    }

    pub fn from_kilometers_per_hour(value: f32) -> Self {
        Self(value / 3.6)
    }

    pub fn meters_per_second(&self) -> f32 {
        self.0
    }

    pub fn kilometers_per_hour(&self) -> f32 {
        self.0 * 3.6
    }
}

impl Default for Speed {
    fn default() -> Self {
        Speed(f32::default())
    }
}