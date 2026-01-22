mod block;
mod blockchain;
mod network;
mod transaction;
mod wallet;

use libp2p::futures::StreamExt;
use libp2p::swarm::SwarmEvent;
use network::*;
use std::env;

#[tokio::main]
async fn main() {
    let mut swarm = build_swarm();

    swarm
        .behaviour_mut()
        .gossipsub
        .subscribe(&tx_topic())
        .unwrap();
    swarm
        .behaviour_mut()
        .gossipsub
        .subscribe(&block_topic())
        .unwrap();

    swarm
        .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
        .unwrap();

        if let Some(addr) = env::args().nth(1) {
    let remote: libp2p::Multiaddr = addr.parse().expect("Invalid multiaddr");
    swarm.dial(remote).expect("Dial failed");
    println!("📞 Dialing {}", addr);
}

    println!("🚀 Toki node started");

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("📡 Listening on {}", address);
            }

            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                println!("🤝 Connected to {}", peer_id);
            }

            SwarmEvent::Behaviour(network::TokiBehaviourEvent::Gossipsub(
                libp2p::gossipsub::Event::Message { message, .. },
            )) => {
                let msg: NetworkMessage = match serde_json::from_slice(&message.data) {
                    Ok(msg) => msg,
                    Err(_) => {
                        println!("⚠️ Invalid network message");
                        continue;
                    }
                };

                match msg {
                    NetworkMessage::NewTransaction(tx) => {
                        println!("📨 Received transaction: {:?}", tx);
                    }
                    NetworkMessage::NewBlock(block) => {
                        println!("📦 Received block: {}", block.index);
                    }
                    _ => {}
                }
            }

            _ => {}
        }
    }
}
