/// Enum for declaring interval for intraday
#[derive(Copy, Clone)]
pub enum Interval {
    OneMin,
    FiveMin,
    FifteenMin,
    ThirtyMin,
    SixtyMin,
    None,
}

/// Enum for declaring output size
#[derive(Copy, Clone)]
pub enum OutputSize {
    Compact,
    Full,
    None,
}

/// Enum for declaring function for stock time series
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

/// Enum for declaring function for forex function
#[derive(Copy, Clone)]
pub enum ForexFunction {
    IntraDay,
    Daily,
    Weekly,
    Monthly,
}

#[derive(Copy, Clone)]
pub enum CryptoFunction {
    Daily,
    Weekly,
    Monthly,
}
