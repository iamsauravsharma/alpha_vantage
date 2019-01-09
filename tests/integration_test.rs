#[test]
// Integration test exchange method call using api
fn exchange_test() {
    let a = alpha_vantage::set_with_timeout("demo", 120);
    assert_eq!(a.exchange("BTC", "CNY").rate().is_ok(), true);
    assert_eq!(a.exchange("USD", "JPY").refreshed_time().is_ok(), true);
}

#[test]
// test Quote method call
fn quote_test() {
    let a = alpha_vantage::set_with_timeout("demo", 120);
    assert_eq!(a.quote("MSFT").price().is_ok(), true);
    assert_eq!(
        a.quote("BA").price(),
        Err(
            "Information : The **demo** API key is for demo purposes only. Please claim your free \
             API key at (https://www.alphavantage.co/support/#api-key) to explore our full API \
             offerings. It takes fewer than 20 seconds, and we are committed to making it free \
             forever."
                .to_string()
        )
    )
}
