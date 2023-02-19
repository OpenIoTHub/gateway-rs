use serde::{Deserialize, Serialize};
use serde_json::{self, to_string, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct MdnsResult {
    #[serde(rename = "name")]
    pub instance: String, //`json:"name"`
    #[serde(rename = "type")]
    pub service: String, //`json:"type"`
    #[serde(rename = "domain")]
    pub domain: String, //`json:"domain"`
    #[serde(rename = "hostname")]
    pub host_name: String, //`json:"hostname"`
    #[serde(rename = "port")]
    pub port: u32, //`json:"port"`
    #[serde(rename = "text")]
    pub text: Vec<String>, //`json:"text"`
    #[serde(rename = "ttl")]
    pub ttl: u32, //`json:"ttl"`
    #[serde(rename = "addripv4")]
    pub addr_ipv4: Vec<String>, //`json:"addripv4"`
    #[serde(rename = "addripv6")]
    pub addr_ipv6: Vec<String>, //`json:"addripv6"`
}
