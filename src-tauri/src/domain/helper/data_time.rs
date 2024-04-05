use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime};
use log::info;

pub fn naive_date_to_naive_date_time(naive_date: NaiveDate) -> NaiveDateTime {
    println!("naive_date_to_naive_date_time {:?}", naive_date);
    NaiveDateTime::new(naive_date, NaiveTime::from_hms_opt(0, 0, 0).unwrap())
}
pub fn naive_date_time_to_naive_date(naive_date_time: NaiveDateTime) -> NaiveDate {
    NaiveDate::from_ymd_opt(
        naive_date_time.year(),
        naive_date_time.month(),
        naive_date_time.day(),
    )
    .unwrap()
}

pub mod option_date_serializer {
    use chrono::NaiveDate;
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S: Serializer>(
        date: &Option<NaiveDate>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match date {
            Some(date) => serializer.serialize_str(&date.format(FORMAT).to_string()),
            None => serializer.serialize_str(""),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<NaiveDate>, D::Error> {
        let s: Option<String> = Option::deserialize(deserializer)?;
        if let Some(s) = s {
            return Ok(Some(
                NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?,
            ));
        }

        Ok(None)
    }
}
