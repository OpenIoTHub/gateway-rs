use crate::handle;
use crate::models::msg::{ReceiveMSG, SendMSG};
use crate::models::token::TokenClaim;
use crate::models::traits::ReceiveMsgTrait;
use crate::utils::msgio;
use futures::{stream, AsyncRead, AsyncReadExt, AsyncWrite, Stream, StreamExt, TryStreamExt};
use log::*;
use smol::io::AsyncWriteExt;
use std::io::{Error, Read};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream, ToSocketAddrs};
use std::thread;
use yamux::{Config, Connection, ConnectionError, Control, Mode};
// use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

pub fn login_conn(login_token_str: String, login_token_obj: &TokenClaim) -> anyhow::Result<()> {
    async fn connect(login_token_str: String, addr: SocketAddr) -> smol::io::Result<()> {
        // Create a listener.
        // let stream = smol::Async::<TcpStream>::connect(([192, 168, 158, 75], 12345)).await?;
        let stream = smol::Async::<TcpStream>::connect(addr).await?;
        // Spawn a task that echoes messages from the client back to it.
        smol::spawn(handle_connected_login_conn(login_token_str.clone(), stream)).detach();
        Ok(())
    }

    info!("read to connect to server login conn using async (smol-rs)!");
    let addr_string = format!("{}:{}", login_token_obj.host, login_token_obj.tcp_port);
    info!("addr_string:{}", addr_string);
    let addr = addr_string.to_socket_addrs()?.next().unwrap();
    info!("socket addr:{}", addr);
    thread::Builder::new()
        .stack_size(16 * 1024)
        .spawn(move || {
            smol::block_on(connect(login_token_str, addr));
        })?;

    anyhow::Ok(())
}

/// Echoes messages from the client back to it.
async fn handle_connected_login_conn(
    login_token_str: String,
    mut stream: smol::Async<TcpStream>,
) -> anyhow::Result<()> {
    info!("(smol-rs) connect openiothub server ok!");
    // 发送登录消息
    let login_msg = SendMSG::GatewayLogin {
        token: login_token_str,
        os: "esp32-IDF".to_string(),
        arch: "RISC-V".to_string(),
        version: "0.1".to_string(),
        disable_muxer: false,
    };
    // 发送消息类型
    // let type_str = "models.GatewayLogin".to_string();
    // stream.write((type_str.len() as u32).to_be_bytes().to_vec().as_slice()).await?;
    // stream.write(type_str.as_bytes()).await?;
    // // 发送消息内容
    // let login_msg_str = serde_json::to_string(&login_msg).unwrap();
    // info!("login_msg_str:{}",login_msg_str);
    // stream.write((login_msg_str.len() as u32).to_be_bytes().to_vec().as_slice()).await?;
    // stream.write(login_msg_str.as_bytes()).await?;

    msgio::send_msg(&mut stream, login_msg).await?;
    //  处理成多路复用
    let mut config = Config::default();
    config.set_receive_window(4 * 1024);
    config.set_max_buffer_size(4 * 1024);
    config.set_split_send_size(4 * 1024);
    let mut server = Connection::new(stream, config, Mode::Server);
    // let (mut ctrl, c_conn) = Control::new(server);
    // smol::spawn(echo_server(client)).detach();
    // control.open_stream().await;
    thread::Builder::new()
        .stack_size(16 * 1024)
        .spawn(move || {
            smol::block_on(handle::handle_client(server));
        })?;
    //  处理多路复用的stream(后续可能有其他连接)
    // stream.write("abc123".as_bytes());
    // smol::io::copy(&stream, &mut &stream).await?;
    // TODO 云亿连协议实现 登录-接收请求-处理请求 kcp、tls支持/p2p的实现
    Ok(())
}
