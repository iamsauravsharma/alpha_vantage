#[derive(Copy, Clone)]
pub enum Interval {
    OneMin,
    FiveMin,
    FifteenMin,
    ThirtyMin,
    SixtyMin,
    None,
}

#[derive(Copy, Clone)]
pub enum OutputSize {
    Compact,
    Full,
    None,
}

#[derive(Copy, Clone)]
pub enum StockFunction {
    IntraDay,
    Daily,
    DailyAdjusted,
    Weekly,
    WeeklyAdjusted,
    Monthly,
    MonthlyAdjusted,
}

#[derive(Copy, Clone)]
pub enum ForexFunction {
    IntraDay,
    Daily,
    Weekly,
    Monthly,
}
