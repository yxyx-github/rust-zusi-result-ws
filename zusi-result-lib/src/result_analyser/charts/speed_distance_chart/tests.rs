use super::*;
use time::macros::datetime;
use crate::result_analyser::charts::chart_data::{ChartDataEntry, Speed};

#[test]
fn test_generate() {
    let chart_data = ChartData::from(vec![
        ChartDataEntry {
            distance: 0.,
            time: datetime!(2019-01-01 23:18:04),
            km: 3.7,
            actual_speed: Speed::from_meters_per_second(0.),
            track_speed_limit: Some(Speed::from_meters_per_second(40.)),
            signal_speed_limit: None,
            train_control_system_speed_limit: None,
        },
        ChartDataEntry {
            distance: 100.,
            time: datetime!(2019-01-01 23:18:24),
            km: 3.8,
            actual_speed: Speed::from_meters_per_second(10.),
            track_speed_limit: Some(Speed::from_meters_per_second(40.)),
            signal_speed_limit: Some(Speed::from_meters_per_second(20.)),
            train_control_system_speed_limit: Some(Speed::from_meters_per_second(25.)),
        },
        ChartDataEntry {
            distance: 500.,
            time: datetime!(2019-01-01 23:18:44),
            km: 4.2,
            actual_speed: Speed::from_meters_per_second(20.),
            track_speed_limit: Some(Speed::from_meters_per_second(30.)),
            signal_speed_limit: Some(Speed::from_meters_per_second(20.)),
            train_control_system_speed_limit: Some(Speed::from_meters_per_second(25.)),
        },
        ChartDataEntry {
            distance: 700.,
            time: datetime!(2019-01-01 23:18:54),
            km: 4.4,
            actual_speed: Speed::from_meters_per_second(20.),
            track_speed_limit: Some(Speed::from_meters_per_second(30.)),
            signal_speed_limit: Some(Speed::from_meters_per_second(20.)),
            train_control_system_speed_limit: Some(Speed::from_meters_per_second(25.)),
        },
        ChartDataEntry {
            distance: 800.,
            time: datetime!(2019-01-01 23:19:04),
            km: 4.5,
            actual_speed: Speed::from_meters_per_second(0.),
            track_speed_limit: Some(Speed::from_meters_per_second(30.)),
            signal_speed_limit: Some(Speed::from_meters_per_second(20.)),
            train_control_system_speed_limit: Some(Speed::from_meters_per_second(5.)),
        },
    ]);

    let config = ChartConfig::default();

    let svg = generate(&chart_data, &config);

    println!("{}", svg.get());

    // panic!("Need to implement asserts...");
}