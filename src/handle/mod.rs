use crate::handle_msg::connect;
use crate::login;
use crate::login::work_conn::login_work_conn;
use crate::models::msg::{ReceiveMSG, SendMSG};
use crate::models::new_service::MdnsResult;
use crate::models::token;
use crate::utils::msgio;
use crate::utils::msgio::send_msg;
use futures::{stream, AsyncRead, AsyncWrite, AsyncWriteExt, TryStreamExt};
use jwt::{Header, Token};
use log::info;
use std::net::TcpStream;
use std::ops::Deref;
use std::thread;
use yamux::{Config, Connection, ConnectionError, Mode, Stream};

// handle all yaml conn except work conn
pub async fn handle_client<T>(mut c: Connection<T>) -> Result<(), ConnectionError>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    stream::poll_fn(|cx| c.poll_next_inbound(cx))
        .try_for_each_concurrent(None, |mut stream| async move {
            // {
            info!("yamux new income stream");
            // let (mut r, mut w) = AsyncReadExt::split(&mut stream);
            // futures::io::copy(&mut r, &mut w).await?;
            let msg_obj = msgio::receive_msg(&mut stream).await.unwrap_or(ReceiveMSG::OK{});
            // 处理请求
            match msg_obj {
                // ReceiveMSG::Message { .. } => {}
                // ReceiveMSG::GatewayLogin { .. } => {}
                // ReceiveMSG::OpenIoTHubLogin { .. } => {}
                ReceiveMSG::NewSubSession { .. } => {
                    info!("NewSubSession...");
                    //  处理成多路复用
                }
                ReceiveMSG::ConnectTCP { .. } => {
                    info!("ConnectTCP...{:?}", msg_obj);
                    thread::Builder::new().stack_size(4*1024).spawn(move || {
                        smol::block_on(connect::tcp::join_tcp(stream,msg_obj));
                    }).unwrap();
                }
                ReceiveMSG::ConnectSTCP { .. } => {
                    info!("ConnectSTCP...{:?}", msg_obj);
                }
                ReceiveMSG::ConnectUDP { .. } => {
                    info!("ConnectUDP...{:?}", msg_obj);
                }
                ReceiveMSG::ConnectSerialPort { .. } => {
                    info!("ConnectSerialPort...{:?}", msg_obj);
                }
                ReceiveMSG::ConnectWs { .. } => {
                    info!("ConnectWs...{:?}", msg_obj);
                }
                ReceiveMSG::ConnectWss { .. } => {
                    info!("ConnectSerialPort...{:?}", msg_obj);
                }
                ReceiveMSG::ConnectSSH { .. } => {
                    info!("ConnectSSH...{:?}", msg_obj);
                }
                ReceiveMSG::Ping { .. } => {
                    info!("ReceiveMSG::Ping...{:?}", msg_obj);
                    let rsp = SendMSG::Pong{};
                    send_msg(stream, rsp).await;
                }
                // ReceiveMSG::Pong { .. } => {}
                ReceiveMSG::ReqNewP2PCtrlAsServer { .. } => {}
                ReceiveMSG::ReqNewP2PCtrlAsClient { .. } => {}
                ReceiveMSG::RemoteNetInfo { .. } => {}
                ReceiveMSG::CheckStatusRequest { .. } => {}
                // ReceiveMSG::CheckStatusResponse { .. } => {}
                ReceiveMSG::NewService { .. } => {
                    info!("NewService:{:?}", msg_obj);
                    let mut rst = Vec::new();
                    rst.push(MdnsResult {
                        instance: "_openiothub-gateway._tcp.local".to_string(),
                        service: "_services._dns-sd._udp".to_string(),
                        domain: "local".to_string(),
                        host_name: "IoTDevice".to_string(),
                        port: 8080,
                        text: vec!["a=b".to_string()],
                        ttl: 3600,
                        addr_ipv4: vec!["127.0.0.1".to_string()],
                        addr_ipv6: vec![],
                    });
                    // TODO 返回真实mdns列表
                    let result = serde_json::to_string(&rst).unwrap();
                    info!("result:{}",result);
                    let rsp = SendMSG::JsonResponse{
                        code: 0,
                        msg: "Success".to_string(),
                        result: result,
                    };
                    send_msg(stream, rsp).await.unwrap();
                }
                ReceiveMSG::RequestNewWorkConn { .. } => {
                    thread::Builder::new().stack_size(4*1024).spawn(move || {
                        let jwt_str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJSdW5JZCI6ImI4YzUxOTI1LTViYzktNGRkMy05YTM1LWYzMjAyNDMxYWEwOCIsIkhvc3QiOiJndW9uZWkubmF0LWNsb3VkLmNvbSIsIlRjcFBvcnQiOjM0MzIwLCJLY3BQb3J0IjozNDMyMCwiVGxzUG9ydCI6MzQzMjEsIkdycGNQb3J0IjozNDMyMiwiVURQQXBpUG9ydCI6MzQzMjEsIktDUEFwaVBvcnQiOjM0MzIyLCJQZXJtaXNzaW9uIjpbImdhdGV3YXkiXSwiVHh0cyI6e30sImV4cCI6MjAxNjc4Mjg0NDk4LCJuYmYiOjE2NzgyNTU2OTh9.iMxk4COineumzJbHQJ7p07jO-jq7Y1CDILpxm6Ilkbs".to_string();
                        let jwt_decoded: Token<Header, token::TokenClaim, _> =
                            jwt::Token::parse_unverified(&jwt_str).unwrap();
                        smol::block_on(login_work_conn(jwt_str.clone(), jwt_decoded.claims().deref()));
                    }).unwrap();
                }
                // ReceiveMSG::GatewayWorkConn { .. } => {}
                // ReceiveMSG::JsonResponse { .. } => {}
                // ReceiveMSG::InstallPlugin { .. } => {}
                // ReceiveMSG::UpgradePlugin { .. } => {}
                // ReceiveMSG::RemovePlugin { .. } => {}
                // ReceiveMSG::RunPlugin { .. } => {}
                // ReceiveMSG::StopPlugin { .. } => {}
                // ReceiveMSG::QueryInstalledPlugin { .. } => {}
                // ReceiveMSG::RespInstalledPlugin { .. } => {}
                // ReceiveMSG::Msg { .. } => {}
                ReceiveMSG::GetMyUDPPublicAddr { .. } => {}
                // ReceiveMSG::OK { .. } => {}
                // ReceiveMSG::Error { .. } => {}
                ReceiveMSG::DeleteGatewayJwt { .. } => {}
                _ => {}
            }
            // }
            // stream.close().await?;
            Ok(())
        })
        .await
}
// work conn only
pub async fn handle_stream<T>(mut stream: T)
where
    T: AsyncWrite + AsyncRead + Unpin + 'static,
{
    // {
    info!("yamux new income stream");
    // let (mut r, mut w) = AsyncReadExt::split(&mut stream);
    // futures::io::copy(&mut r, &mut w).await?;
    let msg_obj = msgio::receive_msg(&mut stream).await.unwrap(); //.unwrap_or(ReceiveMSG::OK{});
    info!("msg_obj:{:?}", msg_obj);
    // 处理请求
    match msg_obj {
        // 单独的工作连接，目前处理生成多路复用就行了
        ReceiveMSG::NewSubSession { .. } => {
            info!("NewSubSession...");
            //  处理成多路复用
            let mut config = Config::default();
            config.set_receive_window(4 * 1024);
            config.set_max_buffer_size(4 * 1024);
            config.set_split_send_size(4 * 1024);
            let server = Connection::new(stream, config, Mode::Server);
            // let (mut ctrl, c_conn) = Control::new(server);
            // smol::spawn(echo_server(client)).detach();
            // control.open_stream().await;
            smol::block_on(handle_client(server));
        }
        _ => {}
    }
}
