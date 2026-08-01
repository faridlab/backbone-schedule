use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "weekday_type", rename_all = "snake_case")]
pub enum WeekdayType {
    Weekday,
    Weekend,
}

impl std::fmt::Display for WeekdayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Weekday => write!(f, "weekday"),
            Self::Weekend => write!(f, "weekend"),
        }
    }
}

impl FromStr for WeekdayType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "weekday" => Ok(Self::Weekday),
            "weekend" => Ok(Self::Weekend),
            _ => Err(format!("Unknown WeekdayType variant: {}", s)),
        }
    }
}

impl Default for WeekdayType {
    fn default() -> Self {
        Self::Weekday
    }
}
