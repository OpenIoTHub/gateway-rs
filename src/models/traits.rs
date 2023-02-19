use crate::models::msg::{ReceiveMSG, SendMSG};
use anyhow;

pub trait SendMsgTrait {
    fn get_type_name(&self) -> String;
}

pub trait ReceiveMsgTrait {
    fn get_msg(msg_type: String, msg_str: String) -> anyhow::Result<ReceiveMSG>;
}
