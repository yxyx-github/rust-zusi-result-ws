use time::{Duration, PrimitiveDateTime};
use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::FahrtEintrag;
use zusi_xml_lib::xml::zusi::result::ZusiResult;

pub fn result_to_fahrt_eintraege_ptr_vec(result: &ZusiResult) -> Vec<&FahrtEintrag> {
    result.fahrt_eintraege.iter().map(|entry| entry).collect()
}

pub fn filter_valid_fahrt_weg_and_fahrt_speed(result: &ZusiResult) -> Vec<&FahrtEintrag> {
    result_to_fahrt_eintraege_ptr_vec(result).into_iter().filter(
        |fahrt_eintrag|
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

    #[test]
    fn test_filter_valid_fahrt_weg_and_fahrt_speed() {
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
                    .fahrt_weg(-1.)
                    .fahrt_zeit(datetime!(2019-01-01 23:18))
                    .fahrt_speed(-1.)
                    .build(),
                FahrtEintrag::builder()
                    .fahrt_weg(22.43)
                    .fahrt_zeit(datetime!(2019-01-01 23:18))
                    .build(),
                FahrtEintrag::builder()
                    .fahrt_weg(1.)
                    .fahrt_zeit(datetime!(2019-01-01 23:18))
                    .fahrt_speed(-1.)
                    .build(),
            ])
            .build();

        assert_eq!(
            filter_valid_fahrt_weg_and_fahrt_speed(&result),
                vec![
                &result.fahrt_eintraege[1],
                &result.fahrt_eintraege[3],
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
