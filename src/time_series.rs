use crate::util::*;
use reqwest::Url;

const LINK: &str = "https://www.alphavantage.co/query?function=";

pub fn create_url(
    function: Function,
    symbol: &str,
    interval: Option<Interval>,
    output_size: Option<OutputSize>,
    api: String,
) -> Url {
    let function = match function {
        Function::IntraDay => "TIME_SERIES_INTRADAY",
        Function::Daily => "TIME_SERIES_DAILY",
        Function::DailyAdjusted => "TIME_SERIES_DAILY_ADJUSTED",
        Function::Weekly => "TIME_SERIES_WEEKLY",
        Function::WeeklyAdjusted => "TIME_SERIES_WEEKLY_ADJUSTED",
        Function::Monthly => "TIME_SERIES_MONTHLY",
        Function::MonthlyAdjusted => "TIME_SERIES_MONTHLY_ADJUSTED",
    };

    let mut url = String::from(format!("{}{}&symbol={}", LINK, function, symbol));
    let interval = match interval {
        Some(interval) => match interval {
            Interval::OneMin => "1min",
            Interval::FiveMin => "5min",
            Interval::FifteenMin => "15min",
            Interval::ThirtyMin => "30min",
            Interval::SixtyMin => "60min",
        },
        None => "",
    };

    if interval != "" {
        url.push_str(format!("&interval={}", interval).as_str());
    }

    url.push_str(match output_size {
        Some(OutputSize::Full) => "&outputsize=full",
        _ => "",
    });

    url.push_str(format!("&apikey={}", api).as_str());
    url.parse().unwrap()
}
