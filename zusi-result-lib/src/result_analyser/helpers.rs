use time::{Duration, PrimitiveDateTime};
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

pub fn round_primitive_date_time(pdt: PrimitiveDateTime) -> PrimitiveDateTime {
    let nanoseconds = pdt.nanosecond();
    let half_second = 500_000_000;

    if nanoseconds >= half_second {
        pdt + Duration::seconds(1) - Duration::nanoseconds(nanoseconds as i64)
    } else {
        pdt - Duration::nanoseconds(nanoseconds as i64)
    }
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
    fn test_round_primitive_date_time() {
        assert_eq!(
            round_primitive_date_time(datetime!(2019-01-01 23:18:30)),
            datetime!(2019-01-01 23:18:30),
        );
        assert_eq!(
            round_primitive_date_time(datetime!(2019-01-01 23:18:30.4)),
            datetime!(2019-01-01 23:18:30),
        );
        assert_eq!(
            round_primitive_date_time(datetime!(2019-01-01 23:18:30.5)),
            datetime!(2019-01-01 23:18:31),
        );
        assert_eq!(
            round_primitive_date_time(datetime!(2019-01-01 23:18:30.9)),
            datetime!(2019-01-01 23:18:31),
        );
    }
}
