pub mod chart_data;
pub mod speed_distance_chart;

#[derive(Default)]
pub struct ChartConfig {
    aspect_ratio: AspectRatio,
}

pub enum AspectRatio {
    Auto,
    Fixed(f64),
}

impl Default for AspectRatio {
    fn default() -> Self {
        AspectRatio::Auto
    }
}

pub struct SVGString(String);

impl SVGString {
    pub fn new() -> SVGString {
        SVGString("".into())
    }
    pub fn get(&self) -> &String {
        &self.0
    }
    pub fn get_mut(&mut self) -> &mut String {
        &mut self.0
    }
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<String> for SVGString {
    fn from(value: String) -> Self {
        SVGString(value)
    }
}

impl From<&str> for SVGString {
    fn from(value: &str) -> Self {
        SVGString(value.into())
    }
}

impl From<SVGString> for String {
    fn from(value: SVGString) -> Self {
        value.0
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Resolution {
    x: usize,
    y: usize,
}