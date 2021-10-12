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
