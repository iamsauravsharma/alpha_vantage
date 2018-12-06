#[test]
// Integration test exchange method call using api
fn exchnage_test() {
    let a = alpha_vantage::set_api("demo");
    assert_eq!(a.exchange("BTC", "CNY").get_rate().is_ok(), true);
    assert_eq!(a.exchange("USD", "JPY").get_refreshed_time().is_ok(), true);
}

#[test]
// test Quote method call
fn quote_test() {
    let a = alpha_vantage::set_api("demo");
    assert_eq!(a.quote("MSFT").get_price().is_ok(), true);
    assert_eq!(
        a.quote("BA").get_price(),
        Err(
            "Information : The **demo** API key is for demo purposes only. Please claim your free \
             API key at (https://www.alphavantage.co/support/#api-key) to explore our full API \
             offerings. It takes fewer than 20 seconds, and we are committed to making it free \
             forever."
                .to_string()
        )
    )
}
