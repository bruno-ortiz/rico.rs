use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerInfo {
    pub accounts: Vec<Account>,
    pub days_till_recad: i64,
    pub days_till_recad_label: String,
    pub default_account: DefaultAccount,
    pub document: String,
    pub email: String,
    pub env: String,
    pub id: i64,
    pub is_overturned: bool,
    pub is_rico_plus: bool,
    pub is_rico_plus_chat_open: bool,
    pub judicially_blocked: bool,
    pub name: String,
    pub natural_person: bool,
    pub qualified: bool,
    pub reservio_link: String,
    pub rules: Rules,
    pub suitability: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub advisor: i64,
    pub bmf: bool,
    pub default: bool,
    pub digit: i64,
    pub is_rico_plus: bool,
    pub number: i64,
    pub wallets: Vec<Wallet>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    pub default: bool,
    pub id: i64,
    pub name: String,
    pub read_only: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultAccount {
    pub advisor: i64,
    pub bmf: bool,
    pub default: bool,
    pub digit: i64,
    pub is_rico_plus: bool,
    pub number: i64,
    pub wallets: Vec<Wallet>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rules {
    pub beta_prev: i64,
    pub can_view_automatic_application: i64,
    pub is_enable_ir2019: i64,
    pub is_new_leverage_page: i64,
    pub new_bank_registration: i64,
    #[serde(rename = "newBannersWP")]
    pub new_banners_wp: i64,
    pub new_brokerage_notes_page: i64,
    pub new_earnings_page: i64,
    pub new_exchange_page: i64,
    #[serde(rename = "newFeaturedWP")]
    pub new_featured_wp: i64,
    pub new_forwards_contracts_page: i64,
    pub new_home_broker: i64,
    #[serde(rename = "newIPO")]
    pub new_ipo: i64,
    pub new_learn_to_invest_page: i64,
    pub new_margins_page: i64,
    pub new_mensal_brokerage_feature: i64,
    pub new_menu: i64,
    #[serde(rename = "newMenuSTG")]
    pub new_menu_stg: i64,
    #[serde(rename = "newModalWP")]
    pub new_modal_wp: i64,
    pub new_negociation_notes_fixed_income_page: i64,
    pub new_options_page: i64,
    #[serde(rename = "newRLPPage")]
    pub new_rlppage: i64,
    pub new_register_details_endpoint: i64,
    pub new_statement_page: i64,
    pub new_suitability: i64,
    pub new_treasury: i64,
    pub new_version: i64,
    pub new_version_fixed_income: i64,
    #[serde(rename = "newVersionSTG")]
    pub new_version_stg: i64,
    pub new_withdraw: i64,
    pub r8_disabled: i64,
    pub riconnect: i64,
    pub summary_from: i64,
    pub summary_position_refresh: i64,
    pub terms_policy: i64,
}
