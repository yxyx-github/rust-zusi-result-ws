use std::fmt::{Display, Formatter};
use time::{format_description, PrimitiveDateTime};

const DATE_TIME_FORMAT: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";
const TIME_FORMAT: &str = "[hour]:[minute]:[second]";

#[derive(PartialEq, Clone, Debug)]
pub struct Schedule {
    entries: Vec<ScheduleEntry>
}

impl From<Vec<ScheduleEntry>> for Schedule {
    fn from(value: Vec<ScheduleEntry>) -> Self {
        Schedule {
            entries: value,
        }
    }
}

impl Display for Schedule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for entry in self.entries.iter() {
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

impl Display for ScheduleEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let dtf = format_description::parse(DATE_TIME_FORMAT).unwrap();
        let tf = format_description::parse(TIME_FORMAT).unwrap();

        writeln!(
            f,
            "{} {} {}",
            self.planned_arrival.format(&tf).map_err(|_| std::fmt::Error)?,
            self.actual_arrival.format(&tf).map_err(|_| std::fmt::Error)?,
            self.name,
        )?;
        writeln!(
            f,
            "{} {}",
            self.planned_departure.format(&tf).map_err(|_| std::fmt::Error)?,
            self.actual_departure.format(&tf).map_err(|_| std::fmt::Error)?,
        )
    }
}
