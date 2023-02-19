use crate::models::msg;
use crate::models::msg::{ReceiveMSG, SendMSG};
use crate::models::traits::{ReceiveMsgTrait, SendMsgTrait};
use anyhow::{self, Ok as anyhow_ok};
use futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use log::info;
use serde::Serialize;

pub async fn send_msg<T, E>(mut conn: T, msg_obj: E) -> anyhow::Result<()>
where
    T: AsyncWrite + Unpin, // + AsyncRead
    E: SendMsgTrait + Serialize,
{
    // 获取消息类型
    let mut type_str = msg_obj.get_type_name();
    // 发送消息类型
    conn.write((type_str.len() as u32).to_be_bytes().to_vec().as_slice())
        .await?;
    conn.write(type_str.as_bytes()).await?;
    // 发送消息内容
    let msg_json_str = serde_json::to_string(&msg_obj).unwrap();
    conn.write(
        (msg_json_str.len() as u32)
            .to_be_bytes()
            .to_vec()
            .as_slice(),
    )
    .await?;
    conn.write(msg_json_str.as_bytes()).await?;
    info!("msg_json_str:{}", msg_json_str);
    conn.flush().await.unwrap();
    anyhow_ok(())
}

pub async fn receive_msg<T>(mut conn: T) -> anyhow::Result<ReceiveMSG>
where
    T: AsyncRead + Unpin, // + AsyncWrite
{
    // 先获取消息的类型
    let mut msg_len = vec![0u8; 4];
    conn.read_exact(&mut msg_len).await?;
    let msg_len_u32 = u32::from_be_bytes(msg_len.clone().try_into().unwrap()) as usize;
    let mut msg_type = vec![0u8; msg_len_u32];
    conn.read_exact(&mut msg_type).await?;
    let msg_type_str = String::from_utf8(msg_type).unwrap();
    //
    conn.read_exact(&mut msg_len).await?;
    let msg_len_u32 = u32::from_be_bytes(msg_len.clone().try_into().unwrap()) as usize;
    let mut msg_content = vec![0u8; msg_len_u32];
    conn.read_exact(&mut msg_content).await?;
    let msg_content_str = String::from_utf8(msg_content).unwrap();
    ReceiveMSG::get_msg(msg_type_str, msg_content_str)
    // anyhow::Error("")
    // anyhow_ok(ReceiveMSG::GatewayLogin {
    //     token: "".to_string(),
    //     os: "".to_string(),
    //     arch: "".to_string(),
    //     version: "".to_string(),
    //     disable_muxer: false,
    // })
}
