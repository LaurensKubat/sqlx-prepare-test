use futures::stream::{Stream, StreamExt};
use sqlx::Executor;

#[derive(Debug, thiserror::Error)]
pub enum FetchBidError {
    #[error("whoops")]
    Sqlx(#[from] sqlx::Error),

    #[error("whoops")]
    Deserialize(#[from] serde_json::Error),
}

pub fn fetch_bids<'a, 'c, C: Executor<'c, Database = sqlx::Postgres> + 'c>(
    conn: C,
    postal_code: &str,
    include_electricity: bool,
    include_gas: bool,
    include_heat: bool,
    include_internet: bool,
) -> impl Stream<Item = Result<Bid, FetchBidError>> + 'c {
    sqlx::query_file!(
        "src/sql/fetch_bids.sql",
        postal_code,
        include_electricity,
        include_gas,
        include_heat,
        include_internet
    )
    .fetch(conn)
    .map(|r| match r {
        Ok(val) => Ok(serde_json::from_value(val.json.unwrap())?),
        Err(err) => Err(FetchBidError::Sqlx(err)),
    })
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bid {
    pub id: Uuid,
    #[serde(rename = "electricity_bid")]
    pub electricity_bid: Option<ElectricityBid>,
    #[serde(rename = "gas_bid")]
    pub gas_bid: Option<GasBid>,
    #[serde(rename = "heat_bid")]
    pub heat_bid: Option<HeatBid>,
    #[serde(rename = "internet_bid")]
    pub internet_bid: Option<InternetBid>,
    pub discounts: Vec<Discount>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectricityBid {
    pub id: Uuid,
    #[serde(rename = "postal_code_lower")]
    pub postal_code_lower: String,
    #[serde(rename = "postal_code_upper")]
    pub postal_code_upper: String,
    #[serde(rename = "single_per_kwh")]
    pub single_per_kwh: f64,
    #[serde(rename = "double_low_per_kwh")]
    pub double_low_per_kwh: f64,
    #[serde(rename = "double_high_per_kwh")]
    pub double_high_per_kwh: f64,
    pub fixed: f64,
    #[serde(rename = "network_management_fee")]
    pub network_management_fee: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GasBid {
    pub id: Uuid,
    #[serde(rename = "postal_code_lower")]
    pub postal_code_lower: String,
    #[serde(rename = "postal_code_upper")]
    pub postal_code_upper: String,
    #[serde(rename = "per_m3")]
    pub per_m3: f64,
    pub fixed: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeatBid {
    pub id: Uuid,
    #[serde(rename = "postal_code_lower")]
    pub postal_code_lower: String,
    #[serde(rename = "postal_code_upper")]
    pub postal_code_upper: String,
    pub per_gj: f64,
    pub fixed: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InternetBid {
    pub id: Uuid,
    pub postal_code_lower: String,
    pub postal_code_upper: String,
    pub speed_lower_bound: f64,
    pub per_month: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Discount {
    pub id: Uuid,
    #[serde(rename = "bid_id")]
    pub bid_id: String,
    #[serde(rename = "one_time")]
    pub one_time: f64,
    pub monthly: f64,
    #[serde(rename = "number_of_months")]
    pub number_of_months: i64,
}
