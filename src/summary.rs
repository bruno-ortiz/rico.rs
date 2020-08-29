use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    #[serde(rename = "alocation")]
    pub allocation: i64,
    pub gross_value: f64,
    pub metadata: Metadata,
    pub net_invested_value: f64,
    pub net_value: f64,
    pub positions: Vec<Position>,
    pub should_display_campaign_modal: bool,
    pub total_invested_value: f64,
    pub total_value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub customer_id: String,
    pub date_hour: String,
    pub event_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    #[serde(rename = "alocation")]
    pub allocation: f64,
    pub available_to_withdrawal: f64,
    pub available_to_withdrawal_with_provisions: f64,
    pub error: bool,
    pub gross_value: f64,
    #[serde(rename = "investedAlocation")]
    pub invested_allocation: f64,
    pub net_value: f64,
    pub product_type: String,
    pub product_type_name: String,
    pub total_value: f64,
}
