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

pub enum TechnicalIndicator {
    Acceleration(String),
    Fastdmatype(String),
    Fastdperiod(String),
    Fastkperiod(String),
    Fastlimit(String),
    Fastmatype(String),
    Fastperiod(String),
    Matype(String),
    Maximum(String),
    Nbdevdn(String),
    Nbdevup(String),
    Signalmatype(String),
    Signalperiod(String),
    Slowdmatype(String),
    Slowdperiod(String),
    Slowkmatype(String),
    Slowkperiod(String),
    Slowlimit(String),
    Slowmatype(String),
    Slowperiod(String),
    Timeperiod1(String),
    Timeperiod2(String),
    Timeperiod3(String),
}
