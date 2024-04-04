use std::{thread, time::Duration, borrow::Cow};

use market_dht::{config::Config, multiaddr, net::spawn_bridge, ResponseData, KadResponseData};
use pretty_assertions::{self, assert_eq};
use tokio::runtime::Runtime;

#[test]
fn test_register_file(){
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

    thread::sleep(Duration::from_secs(1));
    let sha_hash = [123u8; 32];
    Runtime::new().unwrap().block_on(async {
        let response = peer1.register_file(Cow::Owned(sha_hash.to_vec()),
            [190, 32, 11, 23],
            9001,
            300,
            "peer1".to_string()
            ).await;
        if let Ok(ResponseData::KadResponse(KadResponseData::RegisterFile { key })) = response {
            assert_eq!(key, sha_hash.to_vec());
        } else {
            panic!("Didn't get the correct response!")
        }
    });
}


