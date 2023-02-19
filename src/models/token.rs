use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// jwt模型
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaim {
    #[serde(rename = "RunId")]
    pub run_id: String,
    #[serde(rename = "Host")]
    pub host: String,
    #[serde(rename = "TcpPort")]
    pub tcp_port: u32,
    #[serde(rename = "KcpPort")]
    pub kcp_port: u32,
    #[serde(rename = "TlsPort")]
    pub tls_port: u32,
    #[serde(rename = "GrpcPort")]
    pub grpc_port: u32,
    #[serde(rename = "UDPApiPort")]
    pub udp_api_port: u32,
    #[serde(rename = "KCPApiPort")]
    pub kcp_api_port: u32,
    #[serde(rename = "Permission")]
    pub permission: Box<[String]>,
    #[serde(rename = "Txts")]
    pub txts: Box<HashMap<String, String>>,
}
