use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum AuthProviders {
    CyberSherlock,
    Google,
    Facebook,
}
impl AuthProviders {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
    pub fn is_equal(&self, provider: &AuthProviders) -> bool {
        self.to_string() == provider.to_string()
    }
}

impl FromStr for AuthProviders {
    type Err = &'static str;

    // Required method
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "google" => Ok(AuthProviders::Google),
            "facebook" => Ok(AuthProviders::Facebook),
            _ => Err("Invalid AuthProvider"),
        }
    }
}

pub mod datetime_as_mongo_bson {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(val: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let datetime = mongodb::bson::DateTime::from(val.to_owned());
        datetime.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

        match value {
            serde_json::Value::String(datetime_str) => {
                // For a string, try to parse directly
                datetime_str.parse().map_err(|_| {
                    serde::de::Error::custom("Error parsing DateTime from BSON string")
                })
            }
            serde_json::Value::Object(map) => {
                if let Some(date_obj) = map.get("$date") {
                    if let Some(serde_json::Value::String(number_str)) = date_obj.get("$numberLong")
                    {
                        // Parse the number string as a long integer
                        let nanos = number_str.parse::<i64>().map_err(|_| {
                            serde::de::Error::custom("Error parsing $numberLong value")
                        })?;

                        // Convert nanoseconds to seconds
                        let seconds = nanos / 1_000;
                        // Get the remaining nanoseconds
                        let nanos_remainder = (nanos % 1_000) as u32 * 1_000_000;

                        // Create a DateTime<Utc> from seconds and nanoseconds

                        if let Some(date_time) = DateTime::from_timestamp(seconds, nanos_remainder)
                        {
                            Ok(date_time)
                        } else {
                            Err(serde::de::Error::custom("Invalid timestamp"))
                        }
                    } else {
                        Err(serde::de::Error::custom(
                            "Missing '$numberLong' field in $date object",
                        ))
                    }
                } else {
                    Err(serde::de::Error::custom(
                        "Missing '$date' field in $date object",
                    ))
                }
            }
            _ => Err(serde::de::Error::custom("Unexpected BSON data type")),
        }
    }
}
