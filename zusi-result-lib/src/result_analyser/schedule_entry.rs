use time::PrimitiveDateTime;

#[derive(PartialEq, Clone, Debug)]
pub struct ScheduleEntry {
    pub planned_arrival: PrimitiveDateTime,
    pub planned_departure: PrimitiveDateTime,
    pub actual_arrival: PrimitiveDateTime,
    pub actual_departure: PrimitiveDateTime,
    pub name: String,
}
