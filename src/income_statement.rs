//! Module for returning income statement for a company
//!
//! This API returns the annual and quarterly income statements for the company
//! of interest. Data is generally refreshed on the same day a company reports
//! its latest earnings and financials.
use crate::{
    deserialize::{from_none_str, from_str},
    error::{Error, Result},
};
use serde::Deserialize;

/// struct to store information for both annual report and quarterly report
#[derive(Clone, Debug, Deserialize)]
pub struct Report {
    #[serde(rename = "fiscalDateEnding")]
    fiscal_date_ending: String,
    #[serde(rename = "reportedCurrency")]
    reported_currency: String,
    #[serde(rename = "totalRevenue", deserialize_with = "from_str")]
    total_revenue: i64,
    #[serde(rename = "totalOperatingExpense", deserialize_with = "from_str")]
    total_operating_expense: i64,
    #[serde(rename = "costOfRevenue", deserialize_with = "from_str")]
    cost_of_revenue: i64,
    #[serde(rename = "grossProfit", deserialize_with = "from_str")]
    gross_profit: i64,
    #[serde(rename = "ebit", deserialize_with = "from_str")]
    ebit: i64,
    #[serde(rename = "netIncome", deserialize_with = "from_str")]
    net_income: i64,
    #[serde(rename = "researchAndDevelopment", deserialize_with = "from_str")]
    research_and_development: i64,
    #[serde(
        rename = "effectOfAccountingCharges",
        deserialize_with = "from_none_str"
    )]
    effect_of_accounting_charges: Option<i64>,
    #[serde(rename = "incomeBeforeTax", deserialize_with = "from_str")]
    income_before_tax: i64,
    #[serde(rename = "minorityInterest", deserialize_with = "from_str")]
    minority_interest: i64,
    #[serde(rename = "sellingGeneralAdministrative", deserialize_with = "from_str")]
    selling_general_administrative: i64,
    #[serde(rename = "otherNonOperatingIncome", deserialize_with = "from_none_str")]
    other_non_operating_income: Option<i64>,
    #[serde(rename = "operatingIncome", deserialize_with = "from_str")]
    operating_income: i64,
    #[serde(rename = "otherOperatingExpense", deserialize_with = "from_str")]
    other_operating_expense: i64,
    #[serde(rename = "interestExpense", deserialize_with = "from_str")]
    interest_expense: i64,
    #[serde(rename = "taxProvision", deserialize_with = "from_none_str")]
    tax_provision: Option<i64>,
    #[serde(rename = "interestIncome", deserialize_with = "from_none_str")]
    interest_income: Option<i64>,
    #[serde(rename = "netInterestIncome", deserialize_with = "from_none_str")]
    net_interest_income: Option<i64>,
    #[serde(rename = "extraordinaryItems", deserialize_with = "from_str")]
    extraordinary_items: i64,
    #[serde(rename = "nonRecurring", deserialize_with = "from_none_str")]
    non_recurring: Option<i64>,
    #[serde(rename = "otherItems", deserialize_with = "from_none_str")]
    other_items: Option<i64>,
    #[serde(rename = "incomeTaxExpense", deserialize_with = "from_str")]
    income_tax_expense: i64,
    #[serde(rename = "totalOtherIncomeExpense", deserialize_with = "from_str")]
    total_other_income_expense: i64,
    #[serde(rename = "discontinuedOperations", deserialize_with = "from_str")]
    discontinued_operations: i64,
    #[serde(
        rename = "netIncomeFromContinuingOperations",
        deserialize_with = "from_str"
    )]
    net_income_from_continuing_operations: i64,
    #[serde(
        rename = "netIncomeApplicableToCommonShares",
        deserialize_with = "from_str"
    )]
    net_income_applicable_to_common_shares: i64,
    #[serde(
        rename = "preferredStockAndOtherAdjustments",
        deserialize_with = "from_none_str"
    )]
    preferred_stock_and_other_adjustments: Option<i64>,
}

impl Report {
    /// Return fiscal date ending
    #[must_use]
    pub fn fiscal_date_ending(&self) -> &str {
        &self.fiscal_date_ending
    }

    /// Return reported currency
    #[must_use]
    pub fn reported_currency(&self) -> &str {
        &self.reported_currency
    }

    /// Return total revenue
    #[must_use]
    pub fn total_revenue(&self) -> i64 {
        self.total_revenue
    }

    /// Return total operating expense
    #[must_use]
    pub fn total_operating_expense(&self) -> i64 {
        self.total_operating_expense
    }

    /// Return cost of revenue
    #[must_use]
    pub fn cost_of_revenue(&self) -> i64 {
        self.cost_of_revenue
    }

    /// Return gross profit
    #[must_use]
    pub fn gross_profit(&self) -> i64 {
        self.gross_profit
    }

    /// Return ebit
    #[must_use]
    pub fn ebit(&self) -> i64 {
        self.ebit
    }

    /// Return net income
    #[must_use]
    pub fn net_income(&self) -> i64 {
        self.net_income
    }

    /// Return research and development
    #[must_use]
    pub fn research_and_development(&self) -> i64 {
        self.research_and_development
    }

    /// Return effect of accounting charges
    #[must_use]
    pub fn effect_of_accounting_charges(&self) -> Option<i64> {
        self.effect_of_accounting_charges
    }

    /// Return income before tax
    #[must_use]
    pub fn income_before_tax(&self) -> i64 {
        self.income_before_tax
    }

    /// Return minority interest
    #[must_use]
    pub fn minority_interest(&self) -> i64 {
        self.minority_interest
    }

    /// Return selling general administrative
    #[must_use]
    pub fn selling_general_administrative(&self) -> i64 {
        self.selling_general_administrative
    }

    /// Return other non operating income. Return None if api return None
    #[must_use]
    pub fn other_non_operating_income(&self) -> Option<i64> {
        self.other_non_operating_income
    }

    /// Return operating income
    #[must_use]
    pub fn operating_income(&self) -> i64 {
        self.operating_income
    }

    /// Return other operating expense
    #[must_use]
    pub fn other_operating_expense(&self) -> i64 {
        self.other_operating_expense
    }

    /// Return interest expense
    #[must_use]
    pub fn interest_expense(&self) -> i64 {
        self.interest_expense
    }

    /// Return tax provision. Return None if api return None
    #[must_use]
    pub fn tax_provision(&self) -> Option<i64> {
        self.tax_provision
    }

    /// Return interest income. Return None if api return None
    #[must_use]
    pub fn interest_income(&self) -> Option<i64> {
        self.interest_income
    }

    /// Return net interest income. Return None if api return None
    #[must_use]
    pub fn net_interest_income(&self) -> Option<i64> {
        self.net_interest_income
    }

    /// Return extraordinary items
    #[must_use]
    pub fn extraordinary_items(&self) -> i64 {
        self.extraordinary_items
    }

    /// Return non recurring. Return None if api return None
    #[must_use]
    pub fn non_recurring(&self) -> Option<i64> {
        self.non_recurring
    }

    /// Return other items. Return None if api return None
    #[must_use]
    pub fn other_items(&self) -> Option<i64> {
        self.other_items
    }

    /// Return income tax expense
    #[must_use]
    pub fn income_tax_expense(&self) -> i64 {
        self.income_tax_expense
    }

    /// Return total other income expense
    #[must_use]
    pub fn total_other_income_expense(&self) -> i64 {
        self.total_other_income_expense
    }

    /// Return discontinued operations
    #[must_use]
    pub fn discontinued_operations(&self) -> i64 {
        self.discontinued_operations
    }

    /// Return net income from continuing operations
    #[must_use]
    pub fn net_income_from_continuing_operations(&self) -> i64 {
        self.net_income_from_continuing_operations
    }

    /// Return net income applicable to common share
    #[must_use]
    pub fn net_income_applicable_to_common_shares(&self) -> i64 {
        self.net_income_applicable_to_common_shares
    }

    /// Return preferred stock and other adjustments. Return None if api return
    /// None
    #[must_use]
    pub fn preferred_stock_and_other_adjustments(&self) -> Option<i64> {
        self.preferred_stock_and_other_adjustments
    }
}

/// type alias for report for annual report
pub type AnnualReport = Report;
/// type alias for report for quarterly report
pub type QuarterlyReport = Report;

/// Struct to store income statement information
#[derive(Default)]
pub struct IncomeStatement {
    symbol: String,
    annual_reports: Vec<AnnualReport>,
    quarterly_reports: Vec<QuarterlyReport>,
}

impl IncomeStatement {
    /// Return symbol
    ///
    /// ```
    /// #[async_std::main]
    /// async fn main() {
    ///     let api = alpha_vantage::set_api("demo");
    ///     let income_statement = api.income_statement("IBM").await.unwrap();
    ///     let symbol = income_statement.symbol();
    ///     assert_eq!(symbol, "IBM");
    /// }
    /// ```
    #[must_use]
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    /// Return Vec of annual reports
    #[must_use]
    pub fn annual_reports(&self) -> &Vec<AnnualReport> {
        &self.annual_reports
    }

    /// Return Vec of quarterly reports
    #[must_use]
    pub fn quarterly_reports(&self) -> &Vec<QuarterlyReport> {
        &self.quarterly_reports
    }
}
#[derive(Debug, Deserialize)]
pub(crate) struct IncomeStatementHelper {
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "Note")]
    note: Option<String>,
    #[serde(rename = "symbol")]
    symbol: Option<String>,
    #[serde(rename = "annualReports")]
    annual_reports: Option<Vec<AnnualReport>>,
    #[serde(rename = "quarterlyReports")]
    quarterly_reports: Option<Vec<QuarterlyReport>>,
}

impl IncomeStatementHelper {
    pub(crate) fn convert(self) -> Result<IncomeStatement> {
        let mut income_sheet = IncomeStatement::default();
        if let Some(information) = self.information {
            return Err(Error::AlphaVantageInformation(information));
        }
        if let Some(error_message) = self.error_message {
            return Err(Error::AlphaVantageErrorMessage(error_message));
        }
        if let Some(note) = self.note {
            return Err(Error::AlphaVantageNote(note));
        }
        income_sheet.symbol = self.symbol.unwrap();
        income_sheet.annual_reports = self.annual_reports.unwrap();
        income_sheet.quarterly_reports = self.quarterly_reports.unwrap();
        Ok(income_sheet)
    }
}
