use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime};

pub fn naive_date_to_naive_date_time(naive_date: NaiveDate) -> NaiveDateTime {
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

pub mod date_serializer {
    use chrono::NaiveDate;
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S: Serializer>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&date.format(FORMAT).to_string())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDate, D::Error> {
        let s: String = String::deserialize(deserializer)?;

        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

pub mod date_time_serializer {
    use chrono::{NaiveDate, NaiveDateTime};
    use log::info;
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S";

    pub fn serialize<S: Serializer>(
        date_time: &NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&date_time.format(FORMAT).to_string())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<NaiveDateTime, D::Error> {
        let s: String = String::deserialize(deserializer)?;

        info!("{:?}", s);

        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
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
