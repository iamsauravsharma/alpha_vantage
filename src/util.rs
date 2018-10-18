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

pub enum StockFunction {
    IntraDay,
    Daily,
    DailyAdjusted,
    Weekly,
    WeeklyAdjusted,
    Monthly,
    MonthlyAdjusted,
}

pub enum ForexFunction {
    IntraDay,
    Daily,
    Weekly,
    Monthly,
}
