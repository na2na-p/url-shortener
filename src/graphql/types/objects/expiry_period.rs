use juniper::GraphQLEnum;

#[derive(GraphQLEnum, Debug, Clone, Copy)]
pub enum ExpiryPeriod {
    OneHour,
    TwentyFourHours,
    SevenDays,
    ThirtyDays,
    Unlimited,
}

// 必要に応じて、`ExpiryPeriod` 関連の追加ロジックをここに追加
