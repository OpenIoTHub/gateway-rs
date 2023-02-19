use crate::models::traits::{ReceiveMsgTrait, SendMsgTrait};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::{self, to_string, Result};

//服务器需要处理的消息
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)] //https://www.cnblogs.com/jiangbo4444/p/15932305.html
pub enum SendMSG {
    Message {},
    // login
    GatewayLogin {
        #[serde(rename = "Token")]
        token: String,
        #[serde(rename = "Os")]
        os: String,
        #[serde(rename = "Arch")]
        arch: String,
        #[serde(rename = "Version")]
        version: String,
        //禁止muxer，用于支持嵌入式等受限设备
        #[serde(rename = "DisableMuxer")]
        disable_muxer: bool,
    },

    // Connect TO
    OpenIoTHubLogin {
        #[serde(rename = "Token")]
        token: String,
        #[serde(rename = "Os")]
        os: String,
        #[serde(rename = "Arch")]
        arch: String,
        #[serde(rename = "Version")]
        version: String,
    },

    NewSubSession {},

    // connect       //tcp,stcp,udp,serialport,ws,wss
    ConnectTCP {
        #[serde(rename = "TargetIP")]
        target_ip: String,
        #[serde(rename = "TargetPort")]
        target_port: i32,
    },

    ConnectSTCP {
        #[serde(rename = "TargetIP")]
        target_ip: String,
        #[serde(rename = "TargetPort")]
        target_port: i32,
    },

    ConnectUDP {
        #[serde(rename = "TargetIP")]
        target_ip: String,
        #[serde(rename = "TargetPort")]
        target_port: i32,
    },

    ConnectSerialPort {},

    ConnectWs {
        #[serde(rename = "TargetUrl")]
        target_url: String,
        #[serde(rename = "Protocol")]
        protocol: String,
        #[serde(rename = "Origin")]
        origin: String,
    },

    ConnectWss {
        #[serde(rename = "TargetUrl")]
        target_url: String,
        #[serde(rename = "Protocol")]
        protocol: String,
        #[serde(rename = "Origin")]
        origin: String,
    },

    ConnectSSH {
        #[serde(rename = "TargetIP")]
        target_ip: String,
        #[serde(rename = "TargetPort")]
        target_port: i32,
        #[serde(rename = "UserName")]
        user_name: String,
        #[serde(rename = "PassWord")]
        password: String,
    },

    ///Ping
    Ping {},

    Pong {},

    //P2P让远端以listener身份运行
    ReqNewP2PCtrlAsServer {
        #[serde(rename = "IntranetIp")]
        intranet_ip: String,
        #[serde(rename = "IntranetPort")]
        intranet_port: i32,
        #[serde(rename = "ExternalIp")]
        external_ip: String,
        #[serde(rename = "ExternalPort")]
        external_port: i32,
    },

    //让内网端以dial的身份连接我
    ReqNewP2PCtrlAsClient {
        #[serde(rename = "IntranetIp")]
        intranet_ip: String,
        #[serde(rename = "IntranetPort")]
        intranet_port: i32,
        #[serde(rename = "ExternalIp")]
        external_ip: String,
        #[serde(rename = "ExternalPort")]
        external_port: i32,
    },

    //TODO:NETINFO Model
    RemoteNetInfo {
        #[serde(rename = "IntranetIp")]
        intranet_ip: String,
        #[serde(rename = "IntranetPort")]
        intranet_port: i32,
        #[serde(rename = "ExternalIp")]
        external_ip: String,
        #[serde(rename = "ExternalPort")]
        external_port: i32,
    },

    CheckStatusRequest {
        #[serde(rename = "Type")]
        type_name: String,
        #[serde(rename = "Addr")]
        addr: String,
    },

    CheckStatusResponse {
        //Code:0:在线;1:离线
        #[serde(rename = "Code")]
        code: i32,
        #[serde(rename = "Message")]
        message: String,
    },

    NewService {
        #[serde(rename = "Type")]
        type_name: String,
        #[serde(rename = "Config")]
        config: String,
    },

    RequestNewWorkConn {
        #[serde(rename = "Type")]
        type_name: String,
        #[serde(rename = "Config")]
        config: String,
    },

    GatewayWorkConn {
        #[serde(rename = "RunId")]
        run_id: String,
        #[serde(rename = "Secret")]
        secret: String,
        #[serde(rename = "Version")]
        version: String,
    },

    JsonResponse {
        #[serde(rename = "Code")]
        code: i32,
        #[serde(rename = "Msg")]
        msg: String,
        #[serde(rename = "Result")]
        result: String,
    },

    ///plugin
    InstallPlugin {
        #[serde(rename = "TargetUrl")]
        target_url: String,
    },

    UpgradePlugin {
        #[serde(rename = "TargetUrl")]
        target_url: String,
    },

    RemovePlugin {
        #[serde(rename = "TargetUrl")]
        target_url: String,
    },

    RunPlugin {
        #[serde(rename = "TargetUrl")]
        target_url: String,
    },

    StopPlugin {
        #[serde(rename = "TargetUrl")]
        target_url: String,
    },

    ///query installed plugin
    QueryInstalledPlugin {},

    RespInstalledPlugin {},

    ///rsponse Msg
    Msg {
        #[serde(rename = "MsgType")]
        msg_type: String,
        #[serde(rename = "MsgContent")]
        msg_content: String,
    },

    GetMyUDPPublicAddr {},

    OK {},

    Error {
        #[serde(rename = "Code")]
        code: i32,
        #[serde(rename = "Message")]
        message: String,
    },

    DeleteGatewayJwt {},
}

// 获取枚举类型的模型名称，eg: models.GatewayLogin
impl SendMsgTrait for SendMSG {
    fn get_type_name(&self) -> String {
        // std::any::type_name::<Self>().to_string()
        match self {
            SendMSG::Message { .. } => "models.Message".to_string(),
            SendMSG::GatewayLogin { .. } => "models.GatewayLogin".to_string(),
            SendMSG::OpenIoTHubLogin { .. } => "models.OpenIoTHubLogin".to_string(),
            SendMSG::NewSubSession { .. } => "models.NewSubSession".to_string(),
            SendMSG::ConnectTCP { .. } => "models.ConnectTCP".to_string(),
            SendMSG::ConnectSTCP { .. } => "models.ConnectSTCP".to_string(),
            SendMSG::ConnectUDP { .. } => "models.ConnectUDP".to_string(),
            SendMSG::ConnectSerialPort { .. } => "models.ConnectSerialPort".to_string(),
            SendMSG::ConnectWs { .. } => "models.ConnectWs".to_string(),
            SendMSG::ConnectWss { .. } => "models.ConnectWss".to_string(),
            SendMSG::ConnectSSH { .. } => "models.ConnectSSH".to_string(),
            SendMSG::Ping { .. } => "models.Ping".to_string(),
            SendMSG::Pong { .. } => "models.Pong".to_string(),
            SendMSG::ReqNewP2PCtrlAsServer { .. } => "models.ReqNewP2PCtrlAsServer".to_string(),
            SendMSG::ReqNewP2PCtrlAsClient { .. } => "models.ReqNewP2PCtrlAsClient".to_string(),
            SendMSG::RemoteNetInfo { .. } => "models.RemoteNetInfo".to_string(),
            SendMSG::CheckStatusRequest { .. } => "models.CheckStatusRequest".to_string(),
            SendMSG::CheckStatusResponse { .. } => "models.CheckStatusResponse".to_string(),
            SendMSG::NewService { .. } => "models.NewService".to_string(),
            SendMSG::RequestNewWorkConn { .. } => "models.RequestNewWorkConn".to_string(),
            SendMSG::GatewayWorkConn { .. } => "models.GatewayWorkConn".to_string(),
            SendMSG::JsonResponse { .. } => "models.JsonResponse".to_string(),
            SendMSG::InstallPlugin { .. } => "models.InstallPlugin".to_string(),
            SendMSG::UpgradePlugin { .. } => "models.UpgradePlugin".to_string(),
            SendMSG::RemovePlugin { .. } => "models.RemovePlugin".to_string(),
            SendMSG::RunPlugin { .. } => "models.RunPlugin".to_string(),
            SendMSG::StopPlugin { .. } => "models.StopPlugin".to_string(),
            SendMSG::QueryInstalledPlugin { .. } => "models.QueryInstalledPlugin".to_string(),
            SendMSG::RespInstalledPlugin { .. } => "models.RespInstalledPlugin".to_string(),
            SendMSG::Msg { .. } => "models.Msg".to_string(),
            SendMSG::GetMyUDPPublicAddr { .. } => "models.GetMyUDPPublicAddr".to_string(),
            SendMSG::OK { .. } => "models.OK".to_string(),
            SendMSG::Error { .. } => "models.Error".to_string(),
            SendMSG::DeleteGatewayJwt { .. } => "models.DeleteGatewayJwt".to_string(),
            _ => "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum ReceiveMSG {
    Message {},
    // login
    GatewayLogin {
        #[serde(rename = "Token")]
        token: String,
        #[serde(rename = "Os")]
        os: String,
        #[serde(rename = "Arch")]
        arch: String,
        #[serde(rename = "Version")]
        version: String,
        //禁止muxer，用于支持嵌入式等受限设备
        #[serde(rename = "DisableMuxer")]
        disable_muxer: bool,
    },

    // Connect TO
    OpenIoTHubLogin {
        #[serde(rename = "Token")]
        token: String,
        #[serde(rename = "Os")]
        os: String,
        #[serde(rename = "Arch")]
        arch: String,
        #[serde(rename = "Version")]
        version: String,
    },

    NewSubSession {},

    // connect       //tcp,stcp,udp,serialport,ws,wss
    ConnectTCP {
        #[serde(rename = "TargetIP")]
        target_ip: String,
        #[serde(rename = "TargetPort")]
        target_port: i32,
    },

    ConnectSTCP {
        #[serde(rename = "TargetIP")]
        target_ip: String,
        #[serde(rename = "TargetPort")]
        target_port: i32,
    },

    ConnectUDP {
        #[serde(rename = "TargetIP")]
        target_ip: String,
        #[serde(rename = "TargetPort")]
        target_port: i32,
    },

    ConnectSerialPort {},

    ConnectWs {
        #[serde(rename = "TargetUrl")]
        target_url: String,
        #[serde(rename = "Protocol")]
        protocol: String,
        #[serde(rename = "Origin")]
        origin: String,
    },

    ConnectWss {
        #[serde(rename = "TargetUrl")]
        target_url: String,
        #[serde(rename = "Protocol")]
        protocol: String,
        #[serde(rename = "Origin")]
        origin: String,
    },

    ConnectSSH {
        #[serde(rename = "TargetIP")]
        target_ip: String,
        #[serde(rename = "TargetPort")]
        target_port: i32,
        #[serde(rename = "UserName")]
        user_name: String,
        #[serde(rename = "PassWord")]
        password: String,
    },

    ///Ping
    Ping {},

    Pong {},

    //P2P让远端以listener身份运行
    ReqNewP2PCtrlAsServer {
        #[serde(rename = "IntranetIp")]
        intranet_ip: String,
        #[serde(rename = "IntranetPort")]
        intranet_port: i32,
        #[serde(rename = "ExternalIp")]
        external_ip: String,
        #[serde(rename = "ExternalPort")]
        external_port: i32,
    },

    //让内网端以dial的身份连接我
    ReqNewP2PCtrlAsClient {
        #[serde(rename = "IntranetIp")]
        intranet_ip: String,
        #[serde(rename = "IntranetPort")]
        intranet_port: i32,
        #[serde(rename = "ExternalIp")]
        external_ip: String,
        #[serde(rename = "ExternalPort")]
        external_port: i32,
    },

    //TODO:NETINFO Model
    RemoteNetInfo {
        #[serde(rename = "IntranetIp")]
        intranet_ip: String,
        #[serde(rename = "IntranetPort")]
        intranet_port: i32,
        #[serde(rename = "ExternalIp")]
        external_ip: String,
        #[serde(rename = "ExternalPort")]
        external_port: i32,
    },

    CheckStatusRequest {
        #[serde(rename = "Type")]
        type_name: String,
        #[serde(rename = "Addr")]
        addr: String,
    },

    CheckStatusResponse {
        //Code:0:在线;1:离线
        #[serde(rename = "Code")]
        code: i32,
        #[serde(rename = "Message")]
        message: String,
    },

    NewService {
        #[serde(rename = "Type")]
        type_name: String,
        #[serde(rename = "Config")]
        config: String,
    },

    RequestNewWorkConn {
        #[serde(rename = "Type")]
        type_name: String,
        #[serde(rename = "Config")]
        config: String,
    },

    GatewayWorkConn {
        #[serde(rename = "RunId")]
        run_id: String,
        #[serde(rename = "Secret")]
        secret: String,
        #[serde(rename = "Version")]
        version: String,
    },

    JsonResponse {
        #[serde(rename = "Code")]
        code: i32,
        #[serde(rename = "Msg")]
        msg: String,
        #[serde(rename = "Result")]
        result: String,
    },

    ///plugin
    InstallPlugin {
        #[serde(rename = "TargetUrl")]
        target_url: String,
    },

    UpgradePlugin {
        #[serde(rename = "TargetUrl")]
        target_url: String,
    },

    RemovePlugin {
        #[serde(rename = "TargetUrl")]
        target_url: String,
    },

    RunPlugin {
        #[serde(rename = "TargetUrl")]
        target_url: String,
    },

    StopPlugin {
        #[serde(rename = "TargetUrl")]
        target_url: String,
    },

    ///query installed plugin
    QueryInstalledPlugin {},

    RespInstalledPlugin {},

    ///rsponse Msg
    Msg {
        #[serde(rename = "MsgType")]
        msg_type: String,
        #[serde(rename = "MsgContent")]
        msg_content: String,
    },

    GetMyUDPPublicAddr {},

    OK {},

    Error {
        #[serde(rename = "Code")]
        code: i32,
        #[serde(rename = "Message")]
        message: String,
    },

    DeleteGatewayJwt {},
}

// 获取枚举类型的模型名称，eg: models.GatewayLogin
impl ReceiveMsgTrait for ReceiveMSG {
    fn get_msg(msg_type: String, msg_str: String) -> anyhow::Result<ReceiveMSG> {
        let formated_json_str = format!(
            "{{\"type\":\"{}\",\"content\":{}}}",
            msg_type.strip_prefix("models.").unwrap(),
            msg_str
        );
        info!("formated_json_str:{}", formated_json_str);
        let parsed: ReceiveMSG = serde_json::from_str(&formated_json_str).unwrap();
        anyhow::Ok(parsed)
    }
}

#[test]
fn t1() {
    let v = SendMSG::Error {
        code: 1,
        message: "23".to_string(),
    };
    let json: String = serde_json::to_string(&v).unwrap(); // this is ok
    println!("{}", json);
}
