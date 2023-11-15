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

// 必要に応じて、`ExpiryPeriod` 関連の追加ロジックをここに追加
