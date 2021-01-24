use super::*;
use crate::allocation::*;

use util::Error;

use tokio::net::UdpSocket;

async fn create_channel_bind(lifetime: Duration) -> Result<Allocation, Error> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    let a = Allocation::new(socket, FiveTuple::default());

    let addr = SocketAddr::new(Ipv4Addr::new(0, 0, 0, 0).into(), 0);
    let c = ChannelBind::new(ChannelNumber(MIN_CHANNEL_NUMBER), addr);

    a.add_channel_bind(c, lifetime).await?;

    Ok(a)
}

#[tokio::test]
async fn test_channel_bind() -> Result<(), Error> {
    let a = create_channel_bind(Duration::from_millis(20)).await?;

    let result = a.get_channel_addr(&ChannelNumber(MIN_CHANNEL_NUMBER)).await;
    if let Some(addr) = result {
        assert_eq!(addr.ip().to_string(), "0.0.0.0");
    } else {
        assert!(false, "expected some, but got none");
    }

    Ok(())
}

async fn test_channel_bind_start() -> Result<(), Error> {
    let a = create_channel_bind(Duration::from_millis(20)).await?;
    tokio::time::sleep(Duration::from_millis(30)).await;

    assert!(a
        .get_channel_addr(&ChannelNumber(MIN_CHANNEL_NUMBER))
        .await
        .is_none());

    Ok(())
}

async fn test_channel_bind_reset() -> Result<(), Error> {
    let a = create_channel_bind(Duration::from_millis(30)).await?;

    tokio::time::sleep(Duration::from_millis(20)).await;
    {
        let channel_bindings = a.channel_bindings.lock().await;
        if let Some(c) = channel_bindings.get(&ChannelNumber(MIN_CHANNEL_NUMBER)) {
            c.refresh(Duration::from_millis(30)).await;
        }
    }
    tokio::time::sleep(Duration::from_millis(20)).await;

    assert!(a
        .get_channel_addr(&ChannelNumber(MIN_CHANNEL_NUMBER))
        .await
        .is_some());

    Ok(())
}
