pub enum Interval {
    OneMin,
    FiveMin,
    FifteenMin,
    ThirtyMin,
    SixtyMin,
    None,
}

pub enum OutputSize {
    Compact,
    Full,
    None,
}

pub enum Function {
    IntraDay,
    Daily,
    DailyAdjusted,
    Weekly,
    WeeklyAdjusted,
    Monthly,
    MonthlyAdjusted,
}
