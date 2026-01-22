use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;
use crate::block::Block;

use libp2p::{
    gossipsub::{
        self, IdentTopic, MessageAuthenticity, ValidationMode,
    },
    swarm::{NetworkBehaviour, Swarm, Config as SwarmConfig},
    identity,
    tcp::tokio::Transport,
    noise,
    yamux,
    Transport as _,
};


pub const TX_TOPIC: &str = "toki-transactions";
pub const BLOCK_TOPIC: &str = "toki-blocks";

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkMessage {
    NewTransaction(Transaction),
    NewBlock(Block),
    RequestChain,
    ChainResponse(Vec<Block>),
}

#[derive(NetworkBehaviour, Debug)]
pub struct TokiBehaviour {
    pub gossipsub: gossipsub::Behaviour,
}

impl TokiBehaviour {
    pub fn new(id_keys: &identity::Keypair) -> Self {
        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .validation_mode(ValidationMode::Strict)
            .build()
            .unwrap();

        let gossipsub = gossipsub::Behaviour::new(
            MessageAuthenticity::Signed(id_keys.clone()),
            gossipsub_config,
        )
        .unwrap();

        TokiBehaviour { gossipsub }
    }
}

pub fn build_swarm() -> Swarm<TokiBehaviour> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = libp2p::PeerId::from(id_keys.public());

    println!("🆔 PeerId: {}", peer_id);

    let transport = Transport::new(libp2p::tcp::Config::default())
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(noise::Config::new(&id_keys).unwrap())
        .multiplex(yamux::Config::default())
        .boxed();

    let behaviour = TokiBehaviour::new(&id_keys);

    let swarm_config = SwarmConfig::with_tokio_executor();

    Swarm::new(transport, behaviour, peer_id, swarm_config)
}

pub fn tx_topic() -> IdentTopic {
    IdentTopic::new(TX_TOPIC)
}

pub fn block_topic() -> IdentTopic {
    IdentTopic::new(BLOCK_TOPIC)
}
