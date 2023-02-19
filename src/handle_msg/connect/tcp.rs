use crate::models::msg::ReceiveMSG;
use anyhow::{Error, Ok, Result};
use futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, TryStreamExt};
use smol::{future, io};
use std::net::{TcpStream, ToSocketAddrs};
use std::thread;
use yamux::Stream;

pub async fn join_tcp(mut conn: Stream, msg: ReceiveMSG) -> Result<()> {
    if let ReceiveMSG::ConnectTCP {
        target_ip,
        target_port,
    } = msg
    {
        let addr = format!("{}:{}", target_ip, target_port)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap();
        let local_conn = smol::Async::<TcpStream>::connect(addr).await.unwrap();
        let (mut r1, mut w1) = AsyncReadExt::split(&mut conn);
        // Pipe messages from stdin to the server and pipe messages from the server to stdout.
        future::try_zip(
            io::copy(&mut r1, &mut &local_conn),
            io::copy(&local_conn, &mut w1),
        )
        .await?;
    }
    Ok(())
}
// tls tcp
fn join_stcp<T>(mut conn: T) -> Result<()> {
    Ok(())
}
