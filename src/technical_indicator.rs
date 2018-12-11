use crate::util::TechnicalIndicator;
use reqwest::Url;

pub fn create_url(
    function: &str,
    symbol: &str,
    interval: &str,
    apikey: &str,
    series_type: Option<&str>,
    time_period: Option<&str>,
    temporary_value: Vec<TechnicalIndicator>,
) -> Url {
    let mut created_link = format!(
        "https://www.alphavantage.co/query?function={}&symbol={}&interval={}&apikey={}",
        function, symbol, interval, apikey
    );
    if series_type.is_some() {
        created_link.push_str(format!("&series_type={}", series_type.unwrap()).as_str());
    }
    if time_period.is_some() {
        created_link.push_str(format!("&time_period={}", time_period.unwrap()).as_str());
    }
    for values in temporary_value {
        match values {
            TechnicalIndicator::Acceleration(val) => {
                created_link.push_str(format!("&Acceleration={}", val).as_str())
            }
            TechnicalIndicator::Fastdmatype(val) => {
                created_link.push_str(format!("&fastdmatype={}", val).as_str())
            }
            TechnicalIndicator::Fastdperiod(val) => {
                created_link.push_str(format!("&fastdperiod={}", val).as_str())
            }
            TechnicalIndicator::Fastkperiod(val) => {
                created_link.push_str(format!("&fastkperiod={}", val).as_str())
            }
            TechnicalIndicator::Fastlimit(val) => {
                created_link.push_str(format!("&fastlimit={}", val).as_str())
            }
            TechnicalIndicator::Fastmatype(val) => {
                created_link.push_str(format!("&fastmatype={}", val).as_str())
            }
            TechnicalIndicator::Fastperiod(val) => {
                created_link.push_str(format!("&fastperiod={}", val).as_str())
            }
            TechnicalIndicator::Matype(val) => {
                created_link.push_str(format!("&matype={}", val).as_str())
            }
            TechnicalIndicator::Maximum(val) => {
                created_link.push_str(format!("&maximum={}", val).as_str())
            }
            TechnicalIndicator::Nbdevdn(val) => {
                created_link.push_str(format!("&nbdevdn={}", val).as_str())
            }
            TechnicalIndicator::Nbdevup(val) => {
                created_link.push_str(format!("&nbdevup={}", val).as_str())
            }
            TechnicalIndicator::Signalmatype(val) => {
                created_link.push_str(format!("&signalmatype={}", val).as_str())
            }
            TechnicalIndicator::Signalperiod(val) => {
                created_link.push_str(format!("&signalperiod={}", val).as_str())
            }
            TechnicalIndicator::Slowdmatype(val) => {
                created_link.push_str(format!("&slowdmatype={}", val).as_str())
            }
            TechnicalIndicator::Slowdperiod(val) => {
                created_link.push_str(format!("&slowdperiod={}", val).as_str())
            }
            TechnicalIndicator::Slowkmatype(val) => {
                created_link.push_str(format!("&slowkmatype={}", val).as_str())
            }
            TechnicalIndicator::Slowkperiod(val) => {
                created_link.push_str(format!("&slowkperiod={}", val).as_str())
            }
            TechnicalIndicator::Slowlimit(val) => {
                created_link.push_str(format!("&slowlimit={}", val).as_str())
            }
            TechnicalIndicator::Slowmatype(val) => {
                created_link.push_str(format!("&slowmatype={}", val).as_str())
            }
            TechnicalIndicator::Slowperiod(val) => {
                created_link.push_str(format!("&slowperiod={}", val).as_str())
            }
            TechnicalIndicator::Timeperiod1(val) => {
                created_link.push_str(format!("&timeperiod1={}", val).as_str())
            }
            TechnicalIndicator::Timeperiod2(val) => {
                created_link.push_str(format!("&timeperiod2={}", val).as_str())
            }
            TechnicalIndicator::Timeperiod3(val) => {
                created_link.push_str(format!("&timeperiod3={}", val).as_str())
            }
        }
    }
    created_link.parse().unwrap()
}
