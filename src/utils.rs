use crate::error::{Error, Result};
/// Enum for declaring function for crypto series by defining which type of
/// crypto series to be returned
#[derive(Copy, Clone)]
pub enum CryptoFunction {
    /// returns the daily historical time series for a digital currency (e.g.,
    /// BTC) traded on a specific market (e.g., CNY/Chinese Yuan), refreshed
    /// daily at midnight (UTC). Prices and volumes are quoted in both the
    /// market-specific currency and USD.
    Daily,
    /// returns the weekly historical time series for a digital currency (e.g.,
    /// BTC) traded on a specific market (e.g., CNY/Chinese Yuan), refreshed
    /// daily at midnight (UTC). Prices and volumes are quoted in both the
    /// market-specific currency and USD.
    Weekly,
    /// returns the monthly historical time series for a digital currency (e.g.,
    /// BTC) traded on a specific market (e.g., CNY/Chinese Yuan), refreshed
    /// daily at midnight (UTC). Prices and volumes are quoted in both the
    /// market-specific currency and USD.
    Monthly,
}

/// Enum for declaring function for forex function by defining which type of
/// forex series to be returned
#[derive(Copy, Clone)]
pub enum ForexFunction {
    /// returns intraday time series (timestamp, open, high, low, close) of the
    /// FX currency pair specified, updated realtime
    IntraDay,
    /// returns the daily time series (timestamp, open, high, low, close) of the
    /// FX currency pair specified, updated realtime
    Daily,
    /// returns the weekly time series (timestamp, open, high, low, close) of
    /// the FX currency pair specified, updated realtime.
    Weekly,
    /// returns the monthly time series (timestamp, open, high, low, close) of
    /// the FX currency pair specified, updated realtime
    Monthly,
}

/// Enum for declaring output size of API call
#[derive(Copy, Clone)]
pub enum OutputSize {
    /// Return latest top 100 points recommended if no historical data is
    /// required and decreases api json sizes
    Compact,
    /// Returns full api data points recommended if a full historical data is
    /// required
    Full,
}

/// Enum for declaring function for stock time series by defining which type of
/// series of stock to be returned
#[derive(Copy, Clone)]
pub enum StockFunction {
    /// returns intraday time series (timestamp, open, high, low, close, volume)
    /// of the equity specified
    IntraDay,
    /// returns daily time series (date, daily open, daily high, daily low,
    /// daily close, daily volume) of the global equity specified, covering 20+
    /// years of historical data
    Daily,
    /// returns daily time series (date, daily open, daily high, daily low,
    /// daily close, daily volume, daily adjusted close, and split/dividend
    /// events) of the global equity specified, covering 20+ years of historical
    /// data.
    DailyAdjusted,
    /// returns weekly time series (last trading day of each week, weekly open,
    /// weekly high, weekly low, weekly close, weekly volume) of the global
    /// equity specified, covering 20+ years of historical data.
    Weekly,
    /// returns weekly adjusted time series (last trading day of each week,
    /// weekly open, weekly high, weekly low, weekly close, weekly adjusted
    /// close, weekly volume, weekly dividend) of the global equity specified,
    /// covering 20+ years of historical data.
    WeeklyAdjusted,
    /// returns monthly time series (last trading day of each month, monthly
    /// open, monthly high, monthly low, monthly close, monthly volume) of
    /// the global equity specified, covering 20+ years of historical data.
    Monthly,
    /// returns monthly adjusted time series (last trading day of each month,
    /// monthly open, monthly high, monthly low, monthly close, monthly adjusted
    /// close, monthly volume, monthly dividend) of the equity specified,
    /// covering 20+ years of historical data.
    MonthlyAdjusted,
}

/// Enum for declaring different optional value of Technical Indicator used so
/// user can pass different optional value as same datatype
pub enum TechnicalIndicator {
    /// The acceleration factor. Positive floats are accepted. By default,
    /// acceleration=0.01.
    Acceleration(f32),
    /// Moving average type for the fastd moving average. By default,
    /// fastdmatype=0. Integers 0 - 8 are accepted with the following mappings.
    /// 0 = Simple Moving Average (SMA), 1 = Exponential Moving Average (EMA), 2
    /// = Weighted Moving Average (WMA), 3 = Double Exponential Moving Average
    /// (DEMA), 4 = Triple Exponential Moving Average (TEMA), 5 = Triangular
    /// Moving Average (TRIMA), 6 = T3 Moving Average, 7 = Kaufman Adaptive
    /// Moving Average (KAMA), 8 = MESA Adaptive Moving Average (MAMA).
    Fastdmatype(u32),
    /// The time period of the fastd moving average. Positive integers are
    /// accepted. By default, fastdperiod=3
    Fastdperiod(u32),
    /// The time period of the fastk moving average. Positive integers are
    /// accepted. By default, fastkperiod=5
    Fastkperiod(u32),
    /// Positive floats are accepted. By default, fastlimit=0.01.
    Fastlimit(f32),
    /// Moving average type for the faster moving average. By default,
    /// fastmatype=0. Integers 0 - 8 are accepted with the following mappings. 0
    /// = Simple Moving Average (SMA), 1 = Exponential Moving Average (EMA), 2 =
    /// Weighted Moving Average (WMA), 3 = Double Exponential Moving Average
    /// (DEMA), 4 = Triple Exponential Moving Average (TEMA), 5 = Triangular
    /// Moving Average (TRIMA), 6 = T3 Moving Average, 7 = Kaufman Adaptive
    /// Moving Average (KAMA), 8 = MESA Adaptive Moving Average (MAMA).
    Fastmatype(u32),
    /// Positive integers are accepted. By default, fastperiod=12.
    Fastperiod(u32),
    /// Moving average type. By default, matype=0. Integers 0 - 8 are accepted
    /// with the following mappings. 0 = Simple Moving Average (SMA), 1 =
    /// Exponential Moving Average (EMA), 2 = Weighted Moving Average (WMA), 3 =
    /// Double Exponential Moving Average (DEMA), 4 = Triple Exponential Moving
    /// Average (TEMA), 5 = Triangular Moving Average (TRIMA), 6 = T3 Moving
    /// Average, 7 = Kaufman Adaptive Moving Average (KAMA), 8 = MESA Adaptive
    /// Moving Average (MAMA).
    Matype(u32),
    /// The acceleration factor maximum value. Positive floats are accepted. By
    /// default, maximum=0.20
    Maximum(f32),
    /// The standard deviation multiplier of the lower band. Positive integers
    /// are accepted. By default, nbdevdn=2.
    Nbdevdn(u32),
    /// The standard deviation multiplier of the upper band. Positive integers
    /// are accepted. By default, nbdevup=2
    Nbdevup(u32),
    /// Moving average type for the signal moving average. By default,
    /// signalmatype=0. Integers 0 - 8 are accepted with the following mappings.
    /// 0 = Simple Moving Average (SMA), 1 = Exponential Moving Average (EMA), 2
    /// = Weighted Moving Average (WMA), 3 = Double Exponential Moving Average
    /// (DEMA), 4 = Triple Exponential Moving Average (TEMA), 5 = Triangular
    /// Moving Average (TRIMA), 6 = T3 Moving Average, 7 = Kaufman Adaptive
    /// Moving Average (KAMA), 8 = MESA Adaptive Moving Average (MAMA).
    Signalmatype(u32),
    /// Positive integers are accepted. By default, signalperiod=9.
    Signalperiod(u32),
    /// Moving average type for the slowd moving average. By default,
    /// slowdmatype=0. Integers 0 - 8 are accepted with the following mappings.
    /// 0 = Simple Moving Average (SMA), 1 = Exponential Moving Average (EMA), 2
    /// = Weighted Moving Average (WMA), 3 = Double Exponential Moving Average
    /// (DEMA), 4 = Triple Exponential Moving Average (TEMA), 5 = Triangular
    /// Moving Average (TRIMA), 6 = T3 Moving Average, 7 = Kaufman Adaptive
    /// Moving Average (KAMA), 8 = MESA Adaptive Moving Average (MAMA).
    Slowdmatype(u32),
    /// The time period of the slowd moving average. Positive integers are
    /// accepted. By default, slowdperiod=3.
    Slowdperiod(u32),
    /// Moving average type for the slowk moving average. By default,
    /// slowkmatype=0. Integers 0 - 8 are accepted with the following mappings.
    /// 0 = Simple Moving Average (SMA), 1 = Exponential Moving Average (EMA), 2
    /// = Weighted Moving Average (WMA), 3 = Double Exponential Moving Average
    /// (DEMA), 4 = Triple Exponential Moving Average (TEMA), 5 = Triangular
    /// Moving Average (TRIMA), 6 = T3 Moving Average, 7 = Kaufman Adaptive
    /// Moving Average (KAMA), 8 = MESA Adaptive Moving Average (MAMA).
    Slowkmatype(u32),
    /// The time period of the slowk moving average. Positive integers are
    /// accepted. By default, slowkperiod=3.
    Slowkperiod(u32),
    /// Positive floats are accepted. By default, slowlimit=0.01.
    Slowlimit(f32),
    /// Moving average type for the slower moving average. By default,
    /// slowmatype=0. Integers 0 - 8 are accepted with the following mappings. 0
    /// = Simple Moving Average (SMA), 1 = Exponential Moving Average (EMA), 2 =
    /// Weighted Moving Average (WMA), 3 = Double Exponential Moving Average
    /// (DEMA), 4 = Triple Exponential Moving Average (TEMA), 5 = Triangular
    /// Moving Average (TRIMA), 6 = T3 Moving Average, 7 = Kaufman Adaptive
    /// Moving Average (KAMA), 8 = MESA Adaptive Moving Average (MAMA).
    Slowmatype(u32),
    /// Positive integers are accepted. By default, slowperiod=26.
    Slowperiod(u32),
    /// The first time period for the indicator. Positive integers are accepted.
    /// By default, timeperiod1=7.
    Timeperiod1(u32),
    /// The second time period for the indicator. Positive integers are
    /// accepted. By default, timeperiod2=14.
    Timeperiod2(u32),
    /// The third time period for the indicator. Positive integers are accepted.
    /// By default, timeperiod3=28.
    Timeperiod3(u32),
}

/// Enum for declaring interval for technical indicator
#[derive(Copy, Clone)]
pub enum TechnicalIndicatorInterval {
    /// 1 min interval
    OneMin,
    /// 5 min interval
    FiveMin,
    /// 15 min interval
    FifteenMin,
    /// 30 min interval
    ThirtyMin,
    /// 60 min interval
    SixtyMin,
    /// daily interval
    Daily,
    /// weekly interval
    Weekly,
    /// monthly interval
    Monthly,
}

/// Enum for declaring interval for intraday time series
#[derive(Copy, Clone)]
pub enum TimeSeriesInterval {
    /// 1 min interval
    OneMin,
    /// 5 min interval
    FiveMin,
    /// 15 min interval
    FifteenMin,
    /// 30 min interval
    ThirtyMin,
    /// 60 min interval
    SixtyMin,
}

pub(crate) fn detect_common_helper_error(
    information: Option<String>,
    error_message: Option<String>,
    note: Option<String>,
) -> Result<()> {
    if let Some(information) = information {
        return Err(Error::AlphaVantageInformation(information));
    }
    if let Some(error_message) = error_message {
        return Err(Error::AlphaVantageErrorMessage(error_message));
    }
    if let Some(note) = note {
        return Err(Error::AlphaVantageNote(note));
    }
    Ok(())
}
