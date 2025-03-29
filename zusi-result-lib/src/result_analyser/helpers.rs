use zusi_xml_lib::xml::zusi::result::{ResultValue, ZusiResult};
use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::FahrtTyp;

pub fn zusi_result_to_ptr_vec(result: &ZusiResult) -> Vec<&ResultValue> {
    result.value.iter().map(|result_value| result_value).collect()
}

pub fn filter_valid_fahrt_weg_and_fahrt_speed(result: &ZusiResult) -> Vec<&ResultValue> {
    zusi_result_to_ptr_vec(result).into_iter().filter(
        |ResultValue::FahrtEintrag(fahrt_eintrag)|
            fahrt_eintrag.fahrt_weg != -1. && fahrt_eintrag.fahrt_speed != -1.
    ).collect()
}

pub fn filter_planhalt(result: &ZusiResult) -> Vec<&ResultValue> {
    filter_valid_fahrt_weg_and_fahrt_speed(result).into_iter().filter(
        |ResultValue::FahrtEintrag(fahrt_eintrag)|
            fahrt_eintrag.fahrt_typ == FahrtTyp::Planhalt && fahrt_eintrag.fahrt_fpl_ank.is_some() && fahrt_eintrag.fahrt_fpl_abf.is_some()
    ).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;
    use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::FahrtEintrag;

    #[test]
    fn test_filter_valid_fahrt_weg_and_fahrt_speed() {
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
                    .fahrt_weg(-1.)
                    .fahrt_zeit(datetime!(2019-01-01 23:18))
                    .fahrt_speed(-1.)
                    .build()),
                ResultValue::FahrtEintrag(FahrtEintrag::builder()
                    .fahrt_weg(22.43)
                    .fahrt_zeit(datetime!(2019-01-01 23:18))
                    .build()),
                ResultValue::FahrtEintrag(FahrtEintrag::builder()
                    .fahrt_weg(1.)
                    .fahrt_zeit(datetime!(2019-01-01 23:18))
                    .fahrt_speed(-1.)
                    .build()),
            ])
            .build();

        assert_eq!(
            filter_valid_fahrt_weg_and_fahrt_speed(&result),
                vec![
                &result.value[1],
                &result.value[3],
            ],
        )
    }

    #[test]
    fn test_filter_planhalt() {
        let result = ZusiResult::builder()
            .datum(datetime!(2019-01-01 23:14))
            .value(vec![
                ResultValue::FahrtEintrag(FahrtEintrag::builder()
                    .fahrt_zeit(datetime!(2019-01-01 23:18:04))
                    .fahrt_speed(2.)
                    .build()),
                ResultValue::FahrtEintrag(FahrtEintrag::builder()
                    .fahrt_typ(FahrtTyp::Planhalt)
                    .fahrt_zeit(datetime!(2019-01-01 23:18:06))
                    .fahrt_speed(0.)
                    .fahrt_fpl_ank(Some(datetime!(2019-01-01 23:17:00).into()))
                    .fahrt_fpl_abf(Some(datetime!(2019-01-01 23:17:30).into()))
                    .build()),
                ResultValue::FahrtEintrag(FahrtEintrag::builder()
                    .fahrt_zeit(datetime!(2019-01-01 23:18:26))
                    .fahrt_speed(0.)
                    .fahrt_fpl_ank(Some(datetime!(2019-01-01 23:17:00).into()))
                    .fahrt_fpl_abf(Some(datetime!(2019-01-01 23:17:30).into()))
                    .build()),
                ResultValue::FahrtEintrag(FahrtEintrag::builder()
                    .fahrt_zeit(datetime!(2019-01-01 23:18:34))
                    .fahrt_speed(1.)
                    .build()),
                ResultValue::FahrtEintrag(FahrtEintrag::builder()
                    .fahrt_typ(FahrtTyp::Planhalt)
                    .fahrt_zeit(datetime!(2019-01-01 23:19:36))
                    .fahrt_speed(0.)
                    .fahrt_fpl_ank(Some(datetime!(2019-01-01 23:19:00).into()))
                    .fahrt_fpl_abf(Some(datetime!(2019-01-01 23:19:30).into()))
                    .build()),
                ResultValue::FahrtEintrag(FahrtEintrag::builder()
                    .fahrt_zeit(datetime!(2019-01-01 23:19:56))
                    .fahrt_speed(3.)
                    .build()),
                ResultValue::FahrtEintrag(FahrtEintrag::builder()
                    .fahrt_typ(FahrtTyp::Planhalt)
                    .fahrt_zeit(datetime!(2019-01-01 23:22:50))
                    .fahrt_speed(3.)
                    .build()),
            ])
            .build();

        assert_eq!(
            filter_planhalt(&result),
                vec![
                &result.value[1],
                &result.value[4],
            ],
        )
    }
}
