use crate::result_analyser::{AnalyseError, PureAverageSpeedAlgorithm, ResultAnalyser};
use crate::result_analyser_group::{CreateAnalyserGroupError, ResultAnalyserGroup};
use time::macros::datetime;
use time::Duration;
use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::FahrtEintrag;
use zusi_xml_lib::xml::zusi::result::ZusiResult;

#[test]
fn test_cache() {
    let result1 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(0.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(8.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(3.)
                .fahrt_zeit(datetime!(2019-01-01 23:28))
                .fahrt_speed(8.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(4.)
                .fahrt_zeit(datetime!(2019-01-01 23:38))
                .fahrt_speed(0.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(4.)
                .fahrt_zeit(datetime!(2019-01-01 23:48))
                .fahrt_speed(0.)
                .build(),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(0.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(4.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(9.)
                .fahrt_zeit(datetime!(2019-01-01 23:33))
                .fahrt_speed(4.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(16.)
                .fahrt_zeit(datetime!(2019-01-01 23:43))
                .fahrt_speed(0.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(16.)
                .fahrt_zeit(datetime!(2019-01-01 23:53))
                .fahrt_speed(0.)
                .build(),
        ])
        .build();

    let analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    for _ in 0..2 {
        assert_eq!(analyser_group.total_distance().unwrap(), 20.);
        assert_eq!(analyser_group.average_distance().unwrap(), 10.);
        assert_eq!(analyser_group.average_speed().unwrap(), 0.0051282053);
        assert_eq!(analyser_group.pure_average_speed(PureAverageSpeedAlgorithm::PureDrivingTime).unwrap(), 0.0074074073);
        assert_eq!(analyser_group.pure_average_speed(PureAverageSpeedAlgorithm::WeightedLocalSpeeds).unwrap(), 4.4444447);
        assert_eq!(analyser_group.total_driving_time().unwrap(), Duration::minutes(65));
        assert_eq!(analyser_group.total_pure_driving_time().unwrap(), Duration::minutes(45));
    }
}

#[test]
fn test_create_analyser_group_from_ref() {
    let mut analyser = ResultAnalyser::new(
        ZusiResult::builder()
            .fahrt_eintraege(vec![])
            .build()
    );
    let _analyser_group = ResultAnalyserGroup::new(vec![
        &mut analyser
    ]);
}

#[test]
fn test_create_analyser_group_error() {
    let analyser_group = ResultAnalyserGroup::new(vec![] as Vec<ResultAnalyser<ZusiResult>>);
    assert_eq!(analyser_group, Err(CreateAnalyserGroupError::NoAnalysers));
}

#[test]
fn test_total_distance() {
    let result1 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(2.33)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(22.43)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(7.33)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(72.43)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
        ])
        .build();

    let analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(analyser_group.total_distance().unwrap(), 85.2);
}

#[test]
fn test_total_distance_with_error() {
    let result1 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(7.33)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(72.43)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .fahrt_eintraege(vec![])
        .build();

    let analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(
        analyser_group.total_distance(),
        Err(AnalyseError::NoEntries),
    );
}

#[test]
fn test_average_distance() {
    let result1 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(2.33)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(22.43)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(7.33)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(72.43)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
        ])
        .build();

    let analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(analyser_group.average_distance().unwrap(), 42.6);
}

#[test]
fn test_average_distance_with_error() {
    let result1 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(2.33)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(22.43)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .fahrt_eintraege(vec![])
        .build();

    let analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(
        analyser_group.average_distance(),
        Err(AnalyseError::NoEntries),
    );
}

#[test]
fn test_average_speed_2() {
    let result1 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(0.)
                .fahrt_zeit(datetime!(2019-01-01 22:18))
                .fahrt_speed(8.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(3_600.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(8.)
                .build(),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(0.)
                .fahrt_zeit(datetime!(2019-01-01 22:18))
                .fahrt_speed(4.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(7_200.)
                .fahrt_zeit(datetime!(2019-01-01 22:48))
                .fahrt_speed(4.)
                .build(),
        ])
        .build();

    let analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(analyser_group.average_speed().unwrap(), 2.);
}

#[test]
fn test_average_speed_2_zero_driving_time() {
    let result1 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(0.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(8.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(3.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(8.)
                .build(),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(0.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(4.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(9.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(4.)
                .build(),
        ])
        .build();

    let analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(analyser_group.average_speed(), Err(AnalyseError::ZeroDrivingTime));
}

#[test]
fn test_pure_average_speed_2() {
    let result1 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(10.)
                .fahrt_zeit(datetime!(2019-01-01 10:22:18))
                .fahrt_speed(8.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(410.)
                .fahrt_zeit(datetime!(2019-01-01 10:23:08))
                .fahrt_speed(8.)
                .build(),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(10.)
                .fahrt_zeit(datetime!(2019-01-01 10:22:18))
                .fahrt_speed(32.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_weg(810.)
                .fahrt_zeit(datetime!(2019-01-01 10:22:43))
                .fahrt_speed(32.)
                .build(),
        ])
        .build();

    let analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(analyser_group.pure_average_speed(PureAverageSpeedAlgorithm::PureDrivingTime).unwrap(), 16.);
    assert_eq!(analyser_group.pure_average_speed(PureAverageSpeedAlgorithm::WeightedLocalSpeeds).unwrap(), 16.);
}

#[test]
fn test_pure_average_speed_1() {
    let result1 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(3.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(8.)
                .build(),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_weg(9.)
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .fahrt_speed(4.)
                .build(),
        ])
        .build();

    let analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(analyser_group.pure_average_speed(PureAverageSpeedAlgorithm::PureDrivingTime), Err(AnalyseError::ZeroDrivingTime));
    assert_eq!(analyser_group.pure_average_speed(PureAverageSpeedAlgorithm::WeightedLocalSpeeds), Err(AnalyseError::ZeroDistance));
}

#[test]
fn test_pure_average_speed_0() {
    let result1 = ZusiResult::builder()
        .fahrt_eintraege(vec![])
        .build();
    let result2 = ZusiResult::builder()
        .fahrt_eintraege(vec![])
        .build();

    let analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(analyser_group.pure_average_speed(PureAverageSpeedAlgorithm::PureDrivingTime), Err(AnalyseError::NoEntries));
    assert_eq!(analyser_group.pure_average_speed(PureAverageSpeedAlgorithm::WeightedLocalSpeeds), Err(AnalyseError::NoEntries));
}

#[test]
fn test_total_driving_time() {
    let result1 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:28))
                .build(),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:33))
                .build(),
        ])
        .build();

    let analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(analyser_group.total_driving_time().unwrap(), Duration::minutes(25));
}

#[test]
fn test_total_driving_time_with_error() {
    let result1 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:28))
                .build(),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .fahrt_eintraege(vec![])
        .build();

    let analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(
        analyser_group.total_distance(),
        Err(AnalyseError::NoEntries),
    );
}

#[test]
fn test_total_pure_driving_time() {
    let result1 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:08))
                .fahrt_speed(10.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:28))
                .fahrt_speed(0.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:33))
                .fahrt_speed(0.)
                .build(),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:08))
                .fahrt_speed(0.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:28))
                .fahrt_speed(0.)
                .build(),
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:43))
                .fahrt_speed(10.)
                .build(),
        ])
        .build();

    let analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(analyser_group.total_pure_driving_time().unwrap(), Duration::minutes(35));
}

#[test]
fn test_total_pure_driving_time_with_error() {
    let result1 = ZusiResult::builder()
        .fahrt_eintraege(vec![
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:18))
                .build(),
            FahrtEintrag::builder()
                .fahrt_zeit(datetime!(2019-01-01 23:28))
                .build(),
        ])
        .build();
    let result2 = ZusiResult::builder()
        .fahrt_eintraege(vec![])
        .build();

    let analyser_group = ResultAnalyserGroup::new(vec![
        ResultAnalyser::new(result1),
        ResultAnalyser::new(result2),
    ]).unwrap();

    assert_eq!(
        analyser_group.total_distance(),
        Err(AnalyseError::NoEntries),
    );
}

#[test]
fn test_try_from_results() {
    let result = ZusiResult::builder()
        .fahrt_eintraege(vec![])
        .build();

    assert!(ResultAnalyserGroup::try_from(vec![result]).is_ok())
}

#[test]
fn test_try_from_zero_results() {
    assert_eq!(
        ResultAnalyserGroup::try_from(vec![]) as Result<ResultAnalyserGroup<_ , ZusiResult>, CreateAnalyserGroupError>,
        Err(CreateAnalyserGroupError::NoAnalysers),
    );
}
