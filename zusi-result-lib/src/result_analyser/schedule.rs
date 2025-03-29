use std::fmt::{Display, Formatter};
use time::{format_description, Duration, PrimitiveDateTime};

const TIME_FORMAT: &str = "[hour]:[minute]:[second]";

#[derive(PartialEq, Clone, Debug)]
pub struct Schedule(Vec<ScheduleEntry>);

impl From<Vec<ScheduleEntry>> for Schedule {
    fn from(value: Vec<ScheduleEntry>) -> Self {
        Schedule(value)
    }
}

impl Display for Schedule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for entry in self.0.iter() {
            writeln!(f, "{entry}")?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct ScheduleEntry {
    pub planned_arrival: PrimitiveDateTime,
    pub planned_departure: PrimitiveDateTime,
    pub actual_arrival: PrimitiveDateTime,
    pub actual_departure: PrimitiveDateTime,
    pub name: String,
}

impl ScheduleEntry {
    pub fn arrival_delay(&self) -> Delay {
        Delay(self.actual_arrival - self.planned_arrival)
    }

    pub fn departure_delay(&self) -> Delay {
        Delay(self.actual_departure - self.planned_departure)
    }
}

impl Display for ScheduleEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let format = format_description::parse(TIME_FORMAT).unwrap();

        writeln!(
            f,
            "{} {} {} {}",
            self.planned_arrival.format(&format).map_err(|_| std::fmt::Error)?,
            self.arrival_delay(),
            self.actual_arrival.format(&format).map_err(|_| std::fmt::Error)?,
            self.name,
        )?;
        writeln!(
            f,
            "{} {} {}",
            self.planned_departure.format(&format).map_err(|_| std::fmt::Error)?,
            self.departure_delay(),
            self.actual_departure.format(&format).map_err(|_| std::fmt::Error)?,
        )
    }
}

pub struct Delay(Duration);

impl From<Delay> for Duration {
    fn from(value: Delay) -> Self {
        value.0
    }
}

impl Display for Delay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = self.0.abs();
        let minutes = value.whole_minutes();
        let seconds = (value - Duration::minutes(minutes)).whole_seconds();
        write!(f, "{}{}.{:02.}", if self.0.is_positive() { "+" } else { "-" }, minutes, seconds)
    }
}
