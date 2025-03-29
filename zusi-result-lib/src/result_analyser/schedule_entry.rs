use time::PrimitiveDateTime;

#[derive(PartialEq, Debug)]
pub struct ScheduleEntry {
    planned_arrival: PrimitiveDateTime,
    planned_departure: PrimitiveDateTime,
    actual_arrival: PrimitiveDateTime,
    actual_departure: PrimitiveDateTime,
    name: String,
}
