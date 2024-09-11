use time::Duration;
use time::macros::datetime;
use zusi_xml_lib::xml::zusi::result::{ResultValue, ZusiResult};
use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::FahrtEintrag;

use crate::result_analyser::{AnalyseError, PureAverageSpeedAlgorithm, ResultAnalyser};

#[test]
fn create_result_analyser_from_ref() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![])
        .build();

    let _analyser = ResultAnalyser::new(&result);
}

#[test]
fn test_distance_2() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(-1.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(2.33)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(22.43)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(-1.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
        ])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.distance().unwrap(), 20.1);
}

#[test]
fn test_distance_0() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.distance(), Err(AnalyseError::NoEntries));
}

#[test]
fn test_average_speed_2() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(2.33)
                .fahrt_zeit(datetime!(2019-01-01 23:18:00))
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(38.43)
                .fahrt_zeit(datetime!(2019-01-02 0:18:10))
                .build()),
        ])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.average_speed().unwrap(), 0.01);
}

#[test]
fn test_average_speed_2_zero_distance() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(2.33)
                .fahrt_zeit(datetime!(2019-01-01 22:18))
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(2.33)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
        ])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.average_speed().unwrap(), 0.);
}

#[test]
fn test_average_speed_2_zero_driving_time() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(2.33)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(38.43)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build()),
        ])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.average_speed(), Err(AnalyseError::ZeroDrivingTime));
}

#[test]
fn test_average_speed_0() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.average_speed(), Err(AnalyseError::NoEntries));
}

#[test]
fn test_pure_average_speed() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(5.)
                .fahrt_zeit(datetime!(2019-01-01 23:18:04))
                .fahrt_speed(46.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(55.)
                .fahrt_zeit(datetime!(2019-01-01 23:18:06))
                .fahrt_speed(4.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(145.)
                .fahrt_zeit(datetime!(2019-01-01 23:18:26))
                .fahrt_speed(5.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(165.)
                .fahrt_zeit(datetime!(2019-01-01 23:18:34))
                .fahrt_speed(0.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(165.)
                .fahrt_zeit(datetime!(2019-01-01 23:19:36))
                .fahrt_speed(0.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(245.)
                .fahrt_zeit(datetime!(2019-01-01 23:19:56))
                .fahrt_speed(8.)
                .build()),
        ])
        .build();

    let analyser = ResultAnalyser::new(result);
    // TODO: find out reason for different results
    assert_eq!(analyser.pure_average_speed(PureAverageSpeedAlgorithm::PureDrivingTime).unwrap(), 4.8);
    assert_eq!(analyser.pure_average_speed(PureAverageSpeedAlgorithm::WeightedLocalSpeeds).unwrap(), 8.4375);
}

#[test]
fn test_pure_average_speed_2() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(-1.)
                .fahrt_zeit(datetime!(2019-01-01 23:18:00))
                .fahrt_speed(30.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(5.)
                .fahrt_zeit(datetime!(2019-01-01 23:18:04))
                .fahrt_speed(30.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(45.)
                .fahrt_zeit(datetime!(2019-01-01 23:18:06))
                .fahrt_speed(10.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(85.)
                .fahrt_zeit(datetime!(2019-01-01 23:18:26))
                .fahrt_speed(-1.)
                .build()),
        ])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_average_speed(PureAverageSpeedAlgorithm::PureDrivingTime).unwrap(), 20.);
    assert_eq!(analyser.pure_average_speed(PureAverageSpeedAlgorithm::WeightedLocalSpeeds).unwrap(), 20.);
}

#[test]
fn test_pure_average_speed_1() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_weg(5.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(10.)
                .build()),
        ])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_average_speed(PureAverageSpeedAlgorithm::PureDrivingTime), Err(AnalyseError::ZeroDrivingTime));
    assert_eq!(analyser.pure_average_speed(PureAverageSpeedAlgorithm::WeightedLocalSpeeds), Err(AnalyseError::ZeroDistance));
}

#[test]
fn test_pure_average_speed_0() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_average_speed(PureAverageSpeedAlgorithm::PureDrivingTime), Err(AnalyseError::NoEntries));
    assert_eq!(analyser.pure_average_speed(PureAverageSpeedAlgorithm::WeightedLocalSpeeds), Err(AnalyseError::NoEntries));
}

#[test]
fn test_driving_time_2() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(10.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-02 0:38))
                .fahrt_speed(30.)
                .build()),
        ])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.driving_time().unwrap(), Duration::minutes(80));
}

#[test]
fn test_driving_time_0() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.distance(), Err(AnalyseError::NoEntries));
}

#[test]
fn test_pure_driving_time() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 0:40))
                .fahrt_speed(-1.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 20:00))
                .fahrt_speed(30.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:00))
                .fahrt_speed(0.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-02 0:00))
                .fahrt_speed(0.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-02 0:30))
                .fahrt_speed(30.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-02 0:40))
                .fahrt_speed(-1.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-02 0:45))
                .fahrt_speed(0.)
                .build()),
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-02 1:05))
                .fahrt_speed(30.)
                .build()),
        ])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_driving_time().unwrap(), Duration::minutes(245));
}

#[test]
fn test_pure_driving_time_1() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![
            ResultValue::FahrtEintrag(FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-02 1:05))
                .fahrt_speed(30.)
                .build()),
        ])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_driving_time().unwrap(), Duration::seconds(0));
}

#[test]
fn test_pure_driving_time_0() {
    let result = ZusiResult::builder()
        .datum(datetime!(2019-01-01 23:14))
        .value(vec![])
        .build();

    let analyser = ResultAnalyser::new(result);
    assert_eq!(analyser.pure_driving_time(), Err(AnalyseError::NoEntries));
}
