use std::collections::VecDeque;
use std::error::Error;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use bluest::{btuuid, Adapter};
use futures_util::StreamExt;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::task::LocalSet;
use tracing::{error, info};
use tracing::metadata::LevelFilter;
use bluest::corebluetooth::types::{NSInputStream, NSOutputStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let adapter = Adapter::default().await.ok_or("Bluetooth adapter not found")?;
    adapter.wait_available().await?;

    let discovered_device = {
        info!("starting scan");
        let services = &[uuid::uuid!("00000003-e900-4e66-854c-cde416ae1332")];
        let mut scan = adapter.scan(services).await?;
        info!("scan started");
        scan.next().await.ok_or("scan terminated")?
    };

    let device: &bluest::Device = &discovered_device.device;

    info!("{:?} {:?}", discovered_device.rssi, discovered_device.adv_data);
    adapter.connect_device(device).await?;
    info!("connected!");

    let channel = device.open_l2cap_channel(133).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;

    let istream = channel.input_stream();
    let ostream = channel.output_stream();
    tokio::time::sleep(Duration::from_secs(1)).await;

    let req_str = "GET /my-cool-endpoint HTTP/1.1\nHost: osx.ble\n\n";
    let req_bytes = req_str.as_bytes();
    let mut resp = Vec::with_capacity(4096);
    ostream.open();
    istream.open();

    tokio::time::sleep(Duration::from_secs(1)).await;
    if ostream.has_space_available() {
        let nwrite = ostream.write(req_bytes, req_bytes.len());
        info!("wrote: {:?}", req_str);

        tokio::time::sleep(Duration::from_secs(1)).await;

        if istream.has_bytes_available() {
            let nread = istream.read(resp.as_ptr(), resp.capacity());
            if nread > 0 {
                unsafe {
                    info!("got: {:?}", String::from_raw_parts(resp.as_mut_ptr(), nread.try_into().unwrap(), resp.capacity()));
                }
            } else {
                error!("read 0 bytes!");
            }
        } else {
            error!("no bytes available to read");
        }
    } else {
        error!("no space available to write");
    }

    ostream.close();
    istream.close();

    tokio::time::sleep(Duration::from_secs(10)).await;

    adapter.disconnect_device(device).await?;
    info!("disconnected!");

    Ok(())
}

struct NSInputStreamWrapper<'objc, 'executor>{
    inner: &'objc NSInputStream,
    istream_rx: UnboundedReceiver<Vec<u8>>,
    rx_buf: VecDeque<u8>,
    local_executor: &'executor LocalSet,
    // read_task:
}

impl<'a, 'b> AsyncRead for NSInputStreamWrapper<'a, 'b> {
    fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {
        let slef = (&mut (*self.as_mut()));
        match Pin::new(&mut slef.istream_rx).poll_recv(cx) {
            Poll::Ready(bytes) => {
                let mut newv: VecDeque<_> = bytes.expect("rx terminated while polling").into();
                slef.rx_buf.append(&mut newv)
            }
            Poll::Pending => {}
        }

        let n_to_read = std::cmp::min(buf.remaining(), slef.rx_buf.len());
        if n_to_read > 0 {
            // TODO: more optimalizing
            let v = slef.rx_buf.drain(..n_to_read).collect::<Vec<u8>>();
            buf.put_slice(v.as_slice());

            Poll::Ready(Ok(()))
        } else {
            Poll::Pending
        }

        /*if !rx_buf.is_empty() {
            // n_to_read should only be zero if remaining is zero
            let
            let v = std::mem::take(slef.rx_buf);

            return Poll::Ready(());
        }*/
    }
}

struct NSOutputStreamWrapper<'objc>{
    inner: &'objc NSOutputStream
}

impl<'a> AsyncWrite for NSOutputStreamWrapper<'a> {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize, std::io::Error>> {
        // if ostream.has_space_available() {
            let nwrite = self.inner.write(buf, buf.len());
        // }
        Poll::Ready(Ok(nwrite as usize))
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        self.inner.close();

        Poll::Ready(Ok(()))
    }
}