use zusi_xml_lib::xml::zusi::result::{ResultValue, ZusiResult};

pub fn zusi_result_to_ptr_vec(result: &ZusiResult) -> Vec<&ResultValue> {
    result.value.iter().map(|result_value| result_value).collect()
}

pub fn filter_valid_fahrt_weg_and_fahrt_speed(result: &ZusiResult) -> Vec<&ResultValue> {
    zusi_result_to_ptr_vec(result).into_iter().filter(
        |ResultValue::FahrtEintrag(fahrt_eintrag)|
            fahrt_eintrag.fahrt_weg != -1. && fahrt_eintrag.fahrt_speed != -1.
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

        let expected_filtered_result_values = vec![
            &result.value[1],
            &result.value[3],
        ];

        assert_eq!(
            filter_valid_fahrt_weg_and_fahrt_speed(&result),
            expected_filtered_result_values,
        )
    }
}
