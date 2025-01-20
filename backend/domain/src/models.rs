use chrono::{DateTime, Utc};
use diesel::prelude::*;
use rocket::{
    form::{self, FromForm, FromFormField, ValueField},
    serde::{Deserialize, Serialize},
};

#[derive(Queryable, QueryableByName, Selectable, Ord, Eq, PartialEq, PartialOrd, Debug)]
#[diesel(table_name = crate::schema::photos)]
pub struct DateTimeUtcForm {
    pub time_taken: DateTime<Utc>,
}

impl DateTimeUtcForm {
    pub fn inner(&self) -> &DateTime<Utc> {
        &self.time_taken
    }
}

impl<'r> FromFormField<'r> for DateTimeUtcForm {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        println!("{}", field.value);
        match DateTime::parse_from_str(field.value, "%Y-%m-%d %H:%M") {
            Ok(date) => Ok(DateTimeUtcForm {
                time_taken: date.into(),
            }),
            Err(e) => panic!("{}", e),
        }
    }
}

#[derive(Queryable, Selectable, Serialize, Ord, Eq, PartialEq, PartialOrd, Debug, FromForm)]
#[diesel(table_name = crate::schema::photos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Photo {
    pub photo_id: i32,
    pub post_id: i32,
    pub description: Option<String>,
    pub photographer: Option<String>,
    pub photo_path: String,
    #[serde(with = "date_format")]
    pub time_taken: DateTimeUtcForm,
}

#[derive(
    Queryable,
    Selectable,
    Insertable,
    Serialize,
    Deserialize,
    Ord,
    Eq,
    PartialEq,
    PartialOrd,
    Debug,
    FromForm,
)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub post_id: i32,
    pub description: Option<String>,
    pub like_count: Option<i32>,
    pub song: Option<String>,
}

mod date_format {
    use super::DateTimeUtcForm;
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%d/%m/%Y %H:%M";

    pub fn serialize<S>(date: &DateTimeUtcForm, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let date = &date.inner();
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}
