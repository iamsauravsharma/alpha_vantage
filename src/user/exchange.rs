pub(super) struct Exchange {
    from_currency: String,
    to_currency: String,
    api: String,
}

impl Exchange {
    pub(super) fn new(from_currency: String, to_currency: String, api: String) -> Exchange {
        Exchange {
            from_currency,
            to_currency,
            api,
        }
    }
}
