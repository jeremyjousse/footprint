use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime};
use log::info;

pub fn naive_date_to_naive_date_time(naive_date: NaiveDate) -> NaiveDateTime {
    println!("naive_date_to_naive_date_time {:?}", naive_date);
    NaiveDateTime::new(naive_date, NaiveTime::from_hms_opt(0, 0, 0).unwrap())
}
pub fn naive_date_time_to_naive_date(naive_date_time: NaiveDateTime) -> NaiveDate {
    println!("naive_date_time_to_naive_date {:?}", naive_date_time);
    NaiveDate::from_ymd_opt(
        naive_date_time.year(),
        naive_date_time.month(),
        naive_date_time.day(),
    )
    .unwrap()
}

// https://earvinkayonga.com/posts/deserialize-date-in-rust/
// https://serde.rs/custom-date-format.html

pub mod option_date_serializer {
    use chrono::NaiveDate;
    use log::info;
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S: Serializer>(
        date: &Option<NaiveDate>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        println!("serialize {:?}", date);
        match date {
            Some(date) => {
                let s = format!("{}", date.format(FORMAT));
                serializer.serialize_str(&s)
            }
            None => serializer.serialize_str(""),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<NaiveDate>, D::Error> {
        let s = String::deserialize(deserializer)?;
        info!("deserialize {:?}", s);
        NaiveDate::parse_from_str(&s, FORMAT)
            .map(|date| Some(date))
            .map_err(serde::de::Error::custom)
    }
}
