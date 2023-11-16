use chrono::{Duration, Utc};
use juniper::GraphQLEnum;
use serde_derive::{Deserialize, Serialize};

#[derive(GraphQLEnum, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ExpiryPeriod {
    OneHour,
    TwentyFourHours,
    SevenDays,
    ThirtyDays,
    Unlimited,
}

impl ExpiryPeriod {
    pub(crate) fn calculate_expires_at(&self) -> Option<chrono::DateTime<Utc>> {
        let now = Utc::now();
        match self {
            ExpiryPeriod::OneHour => Some(now + Duration::hours(1)),
            ExpiryPeriod::TwentyFourHours => Some(now + Duration::hours(24)),
            ExpiryPeriod::SevenDays => Some(now + Duration::days(7)),
            ExpiryPeriod::ThirtyDays => Some(now + Duration::days(30)),
            ExpiryPeriod::Unlimited => None,
        }
    }
}
