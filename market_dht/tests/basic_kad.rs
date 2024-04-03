use std::{thread, time::Duration, borrow::Cow};

use market_dht::{config::Config, multiaddr, net::spawn_bridge, ResponseData, KadResponseData};
use pretty_assertions::{self, assert_eq};
use tokio::runtime::Runtime;


// Test to find LOCAL peers within a Kad network
#[test]
fn test_get_closest_local_peers() {
    let peer1 = spawn_bridge(
        Config::builder()
            .with_listener(multiaddr!(Ip4([127, 0, 0, 1]), Tcp(1236u16)))
            .with_thread_name("peer1".to_owned())
            .build(),
    )
    .unwrap();

    let _peer2 = spawn_bridge(
        Config::builder()
            .with_listener(multiaddr!(Ip4([127, 0, 0, 1]), Tcp(1237u16)))
            .with_thread_name("peer2".to_owned())
            .with_boot_nodes(
                vec![("/ip4/127.0.0.1/tcp/1236".to_owned(), peer1.id().to_string())]
                .try_into()
                .unwrap(),
                )
                .build(),
        )
        .unwrap();

    let _peer3 = spawn_bridge(
        Config::builder()
            .with_listener(multiaddr!(Ip4([127, 0, 0, 1]), Tcp(1238u16)))
            .with_thread_name("peer3".to_owned())
            .with_boot_nodes(
                vec![("/ip4/127.0.0.1/tcp/1236".to_owned(), peer1.id().to_string())]
                .try_into()
                .unwrap(),
                )
                .build(),
        )
        .unwrap();

    let _peer4 = spawn_bridge(
        Config::builder()
            .with_listener(multiaddr!(Ip4([127, 0, 0, 1]), Tcp(1239u16)))
            .with_thread_name("peer4".to_owned())
            .with_boot_nodes(
                vec![("/ip4/127.0.0.1/tcp/1236".to_owned(), peer1.id().to_string())]
                .try_into()
                .unwrap(),
                )
                .build(),
        )
        .unwrap();

    thread::sleep(Duration::from_secs(1));
    Runtime::new().unwrap().block_on(async {
        let kad_response = _peer4
            .get_closest_local_peers(Cow::Owned(_peer3.id().to_bytes()))
            .await
            .unwrap();
        if let ResponseData::KadResponse(KadResponseData::ClosestLocalPeers { peers }) = kad_response {
            assert_eq!(3, peers.len());
        } else {
            panic!("Didn't get the correct response!")
        }
    });
}

#[test]
fn test_get_closest_peers(){
    let peer1 = spawn_bridge(
        Config::builder()
            .with_listener(multiaddr!(Ip4([127, 0, 0, 1]), Tcp(1240u16)))
            .with_thread_name("peer1".to_owned())
            .build(),
    )
    .unwrap();

    let _peer2 = spawn_bridge(
        Config::builder()
            .with_listener(multiaddr!(Ip4([127, 0, 0, 1]), Tcp(1241u16)))
            .with_thread_name("peer2".to_owned())
            .with_boot_nodes(
                vec![("/ip4/127.0.0.1/tcp/1240".to_owned(), peer1.id().to_string())]
                .try_into()
                .unwrap(),
                )
                .build(),
        )
        .unwrap();

    let _peer3 = spawn_bridge(
        Config::builder()
            .with_listener(multiaddr!(Ip4([127, 0, 0, 1]), Tcp(1242u16)))
            .with_thread_name("peer3".to_owned())
            .with_boot_nodes(
                vec![("/ip4/127.0.0.1/tcp/1240".to_owned(), peer1.id().to_string())]
                .try_into()
                .unwrap(),
                )
                .build(),
        )
        .unwrap();

    let _peer4 = spawn_bridge(
        Config::builder()
            .with_listener(multiaddr!(Ip4([127, 0, 0, 1]), Tcp(1243u16)))
            .with_thread_name("peer4".to_owned())
            .with_boot_nodes(
                vec![("/ip4/127.0.0.1/tcp/1240".to_owned(), peer1.id().to_string())]
                .try_into()
                .unwrap(),
                )
                .build(),
        )
        .unwrap();

    thread::sleep(Duration::from_secs(1));
    Runtime::new().unwrap().block_on(async {
        let kad_response = _peer4
            .get_closest_peers(Cow::Owned(_peer4.id().to_bytes()))
            .await
            .unwrap();
        if let ResponseData::KadResponse(KadResponseData::ClosestPeers { key: _ , peers }) = kad_response {
            assert_eq!(3, peers.len());
        } else {
            panic!("Didn't get the correct response!")
        }
    });
}