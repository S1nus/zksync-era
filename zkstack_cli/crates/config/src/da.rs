//! Mirrored types for data availability configs.

use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AvailDefaultConfig {
    pub api_node_url: String,
    pub app_id: u32,
    pub finality_state: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AvailGasRelayConfig {
    pub gas_relay_api_url: String,
    pub max_retries: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AvailClientConfig {
    FullClient(AvailDefaultConfig),
    GasRelay(AvailGasRelayConfig),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AvailConfig {
    pub bridge_api_url: String,
    pub timeout_ms: usize,
    #[serde(flatten)]
    pub config: AvailClientConfig,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AvailSecrets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_phrase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_relay_api_key: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CelestiaConfig {
    // gRPC URL for celestia-app instance
    pub api_node_url: String,
    // gRPC URL of the Celestia eq-service instance
    // https://github.com/celestiaorg/eq-service
    pub eq_service_grpc_url: String,
    pub namespace: String,
    pub chain_id: String,
    pub timeout_ms: u64,
    // Tendermint RPC URL of the Celestia core instance
    pub celestia_core_tendermint_rpc_url: String,
    pub blobstream_contract_address: String,
    pub blobstream_events_num_pages: u64,
    pub blobstream_events_page_size: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CelestiaSecrets {
    pub private_key: String,
}
