use time::macros::datetime;
use time::Duration;
use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::{FahrtEintrag, FahrtTyp};
use zusi_xml_lib::xml::zusi::result::ZusiResult;

use crate::result_analyser::schedule::{Schedule, ScheduleEntry};
use crate::result_analyser::{AnalyseError, PureAverageSpeedAlgorithm, ResultAnalyser};

#[test]
fn test_cache() {
    let result = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(5.)
                .fahrt_zeit(datetime!(2019-01-01 23:18:04))
                .fahrt_speed(46.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(55.)
                .fahrt_zeit(datetime!(2019-01-01 23:18:06))
                .fahrt_speed(4.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(145.)
                .fahrt_zeit(datetime!(2019-01-01 23:18:26))
                .fahrt_speed(5.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_typ(FahrtTyp::Planhalt)
                .fahrt_weg(165.)
                .fahrt_zeit(datetime!(2019-01-01 23:18:34))
                .fahrt_speed(0.)
                .fahrt_text("Station".into())
                .fahrt_fpl_ank(Some(datetime!(2019-01-01 23:18:30).into()))
                .fahrt_fpl_abf(Some(datetime!(2019-01-01 23:19:00).into()))
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(165.)
                .fahrt_zeit(datetime!(2019-01-01 23:19:36))
                .fahrt_speed(0.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(245.)
                .fahrt_zeit(datetime!(2019-01-01 23:19:56))
                .fahrt_speed(8.)
                .build(),
        ])
        .build();

    let mut analyser = ResultAnalyser::new(result);

    for _ in 0..2 {
        assert_eq!(analyser.distance().unwrap(), 240.);
        assert_eq!(analyser.average_speed().unwrap(), 2.142857);
        assert_eq!(analyser.pure_average_speed(PureAverageSpeedAlgorithm::PureDrivingTime).unwrap(), 4.8);
        assert_eq!(analyser.pure_average_speed(PureAverageSpeedAlgorithm::WeightedLocalSpeeds).unwrap(), 4.8);
        assert_eq!(analyser.driving_time().unwrap(), Duration::seconds(112));
        assert_eq!(analyser.pure_driving_time().unwrap(), Duration::seconds(50));
        assert_eq!(analyser.schedule().unwrap(), Schedule::from(vec![
            ScheduleEntry {
                planned_arrival: datetime!(2019-01-01 23:18:30),
                planned_departure: datetime!(2019-01-01 23:19:00),
                actual_arrival: datetime!(2019-01-01 23:18:34),
                actual_departure: datetime!(2019-01-01 23:19:56),
                name: "Station".into(),
            },
        ]));
    }
}

#[test]
fn create_result_analyser_from_ref() {
    let result = ZusiResult::builder()
        .fahrt_eintraege(vec![])
        .build();

    let _analyser = ResultAnalyser::new(&result);
}

#[test]
fn test_distance_2() {
    let result = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(-1.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(2.33)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(22.43)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(-1.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
        ])
        .build();

    let mut analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.distance().unwrap(), 20.1);
}

#[test]
fn test_distance_0() {
    let result = ZusiResult::builder()
        .fahrt_eintraege(vec![])
        .build();

    let mut analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.distance(), Err(AnalyseError::NoEntries));
}

#[test]
fn test_average_speed_3() {
    let result = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(0.)
                .fahrt_zeit(datetime!(2019-01-01 22:18))
                .fahrt_speed(8.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(2.)
                .fahrt_zeit(datetime!(2019-01-01 22:28))
                .fahrt_speed(0.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(3_600.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(8.)
                .build(),
        ])
        .build();

    let mut analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.average_speed().unwrap(), 1.);
}

#[test]
fn test_average_speed_2() {
    let result = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(2.33)
                .fahrt_zeit(datetime!(2019-01-01 23:18:00))
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(38.43)
                .fahrt_zeit(datetime!(2019-01-02 0:18:10))
                .build(),
        ])
        .build();

    let mut analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.average_speed().unwrap(), 0.01);
}

#[test]
fn test_average_speed_2_zero_distance() {
    let result = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(2.33)
                .fahrt_zeit(datetime!(2019-01-01 22:18))
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(2.33)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
        ])
        .build();

    let mut analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.average_speed().unwrap(), 0.);
}

#[test]
fn test_average_speed_2_zero_driving_time() {
    let result = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(2.33)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(38.43)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
        ])
        .build();

    let mut analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.average_speed(), Err(AnalyseError::ZeroDrivingTime));
}

#[test]
fn test_average_speed_0() {
    let result = ZusiResult::builder()
        .fahrt_eintraege(vec![])
        .build();

    let mut analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.average_speed(), Err(AnalyseError::NoEntries));
}

#[test]
fn test_pure_average_speed() {
    let result = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(5.)
                .fahrt_zeit(datetime!(2019-01-01 23:18:04))
                .fahrt_speed(46.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(55.)
                .fahrt_zeit(datetime!(2019-01-01 23:18:06))
                .fahrt_speed(4.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(145.)
                .fahrt_zeit(datetime!(2019-01-01 23:18:26))
                .fahrt_speed(5.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(165.)
                .fahrt_zeit(datetime!(2019-01-01 23:18:34))
                .fahrt_speed(0.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(165.)
                .fahrt_zeit(datetime!(2019-01-01 23:19:36))
                .fahrt_speed(0.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(245.)
                .fahrt_zeit(datetime!(2019-01-01 23:19:56))
                .fahrt_speed(8.)
                .build(),
        ])
        .build();

    let mut analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_average_speed(PureAverageSpeedAlgorithm::PureDrivingTime).unwrap(), 4.8);
    assert_eq!(analyser.pure_average_speed(PureAverageSpeedAlgorithm::WeightedLocalSpeeds).unwrap(), 4.8);
}

#[test]
fn test_pure_average_speed_2() {
    let result = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(-1.)
                .fahrt_zeit(datetime!(2019-01-01 23:18:00))
                .fahrt_speed(30.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(5.)
                .fahrt_zeit(datetime!(2019-01-01 23:18:04))
                .fahrt_speed(30.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(45.)
                .fahrt_zeit(datetime!(2019-01-01 23:18:06))
                .fahrt_speed(10.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(85.)
                .fahrt_zeit(datetime!(2019-01-01 23:18:26))
                .fahrt_speed(-1.)
                .build(),
        ])
        .build();

    let mut analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_average_speed(PureAverageSpeedAlgorithm::PureDrivingTime).unwrap(), 20.);
    assert_eq!(analyser.pure_average_speed(PureAverageSpeedAlgorithm::WeightedLocalSpeeds).unwrap(), 20.);
}

#[test]
fn test_pure_average_speed_1() {
    let result = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(5.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(10.)
                .build(),
        ])
        .build();

    let mut analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_average_speed(PureAverageSpeedAlgorithm::PureDrivingTime), Err(AnalyseError::ZeroDrivingTime));
    assert_eq!(analyser.pure_average_speed(PureAverageSpeedAlgorithm::WeightedLocalSpeeds), Err(AnalyseError::ZeroDistance));
}

#[test]
fn test_pure_average_speed_0() {
    let result = ZusiResult::builder()
        .fahrt_eintraege(vec![])
        .build();

    let mut analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_average_speed(PureAverageSpeedAlgorithm::PureDrivingTime), Err(AnalyseError::NoEntries));
    assert_eq!(analyser.pure_average_speed(PureAverageSpeedAlgorithm::WeightedLocalSpeeds), Err(AnalyseError::NoEntries));
}

#[test]
fn test_driving_time_2() {
    let result = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(10.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-02 0:38))
                .fahrt_speed(30.)
                .build(),
        ])
        .build();

    let mut analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.driving_time().unwrap(), Duration::minutes(80));
}

#[test]
fn test_driving_time_0() {
    let result = ZusiResult::builder()
        .fahrt_eintraege(vec![])
        .build();

    let mut analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.distance(), Err(AnalyseError::NoEntries));
}

#[test]
fn test_pure_driving_time() {
    let result = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 0:40))
                .fahrt_speed(-1.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 20:00))
                .fahrt_speed(30.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:00))
                .fahrt_speed(0.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-02 0:00))
                .fahrt_speed(0.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-02 0:30))
                .fahrt_speed(30.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-02 0:40))
                .fahrt_speed(-1.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-02 0:45))
                .fahrt_speed(0.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-02 1:05))
                .fahrt_speed(30.)
                .build(),
        ])
        .build();

    let mut analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_driving_time().unwrap(), Duration::minutes(245));
}

#[test]
fn test_pure_driving_time_1() {
    let result = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-02 1:05))
                .fahrt_speed(30.)
                .build(),
        ])
        .build();

    let mut analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_driving_time().unwrap(), Duration::seconds(0));
}

#[test]
fn test_pure_driving_time_0() {
    let result = ZusiResult::builder()
        .fahrt_eintraege(vec![])
        .build();

    let mut analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_driving_time(), Err(AnalyseError::NoEntries));
}

#[test]
fn test_schedule() {
    let result = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:18:04))
                .fahrt_speed(2.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_typ(FahrtTyp::Planhalt)
                .fahrt_zeit(datetime!(2019-01-01 23:18:06))
                .fahrt_speed(0.)
                .fahrt_text("CityA".into())
                .fahrt_fpl_ank(Some(datetime!(2019-01-01 23:17:00).into()))
                .fahrt_fpl_abf(Some(datetime!(2019-01-01 23:17:30).into()))
                .build(),
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:18:26))
                .fahrt_speed(0.)
                .fahrt_text("ignored City".into())
                .fahrt_fpl_ank(Some(datetime!(2019-01-01 23:17:00).into()))
                .fahrt_fpl_abf(Some(datetime!(2019-01-01 23:17:30).into()))
                .build(),
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:18:34))
                .fahrt_speed(1.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_typ(FahrtTyp::Planhalt)
                .fahrt_zeit(datetime!(2019-01-01 23:19:36))
                .fahrt_speed(0.)
                .fahrt_text("CityB".into())
                .fahrt_fpl_ank(Some(datetime!(2019-01-01 23:19:00).into()))
                .fahrt_fpl_abf(Some(datetime!(2019-01-01 23:19:30).into()))
                .build(),
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:19:56))
                .fahrt_speed(3.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_typ(FahrtTyp::Planhalt)
                .fahrt_zeit(datetime!(2019-01-01 23:22:50))
                .fahrt_speed(3.)
                .build(),
        ])
        .build();

    let mut analyser = ResultAnalyser::new(result);

    assert_eq!(analyser.schedule().unwrap(), Schedule::from(vec![
        ScheduleEntry {
            planned_arrival: datetime!(2019-01-01 23:17:00),
            planned_departure: datetime!(2019-01-01 23:17:30),
            actual_arrival: datetime!(2019-01-01 23:18:06),
            actual_departure: datetime!(2019-01-01 23:18:34),
            name: "CityA".into(),
        },
        ScheduleEntry {
            planned_arrival: datetime!(2019-01-01 23:19:00),
            planned_departure: datetime!(2019-01-01 23:19:30),
            actual_arrival: datetime!(2019-01-01 23:19:36),
            actual_departure: datetime!(2019-01-01 23:19:56),
            name: "CityB".into(),
        },
    ]));
}
