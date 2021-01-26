#[derive(serde::Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum TimeInForce {
    #[serde(rename="FTC")]
    GoodTillCancelled,
    #[serde(rename="IOC")]
    ImmediateOrCancel,
    #[serde(rename="FOK")]
    FillOrKill,
    #[serde(rename="Day")]
    Day,
    #[serde(rename="GTD")]
    GoodTillDate,
}
