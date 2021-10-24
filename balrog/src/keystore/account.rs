use super::Error;
use bip39::{Language, Mnemonic};
use hex;
use serde::{Deserialize, Serialize, Serializer};
use slip10::Node;

const MAGIC_NB: u32 = 1789;
const ORGIN_INDEX: u32 = slip10::FIRST_HARDENED_INDEX + MAGIC_NB;

pub struct Account {
    node: Node,
    index_max: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Keypair {
    #[serde(serialize_with = "serialize_bytes")]
    pub public: Vec<u8>,
    #[serde(serialize_with = "serialize_bytes")]
    pub secret: Vec<u8>,
}

fn serialize_bytes<S>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    return s.serialize_str(&hex::encode(v));
}

impl Account {
    pub fn import(mnemonic: &str) -> Result<Account, Error> {
        let m = Mnemonic::parse_in(Language::English, mnemonic)?;
        let seed = m.to_seed("");
        return Account::from_bip39_seed(&seed);
    }

    pub fn generate() -> Result<(Account, String), Error> {
        let m = Mnemonic::generate_in(Language::English, 24).unwrap();
        let seed = m.to_seed("");
        let acc = Account::from_bip39_seed(&seed)?;
        return Ok((acc, m.to_string()));
    }

    fn from_bip39_seed(seed: &[u8]) -> Result<Account, Error> {
        let node = Node::new_master_node(&seed)?;
        let node = node.derive(ORGIN_INDEX)?;
        return Ok(Account { node, index_max: 1 });
    }

    pub fn from_seed(seed: Vec<u8>, index_max: u64) -> Account {
        return Account {
            node: Node::from_hash(&seed),
            index_max,
        };
    }

    pub fn address(&self) -> String {
        return self.node.address();
    }

    pub fn index_max(&self) -> u64 {
        return self.index_max;
    }

    pub fn seed(&self) -> Vec<u8> {
        return self.node.hash();
    }

    pub fn keypairs(&self) -> Result<Vec<Keypair>, Error> {
        // always derive the second part of the path
        let node = self.node.derive(slip10::FIRST_HARDENED_INDEX)?;
        // now we can derive keys
        let mut out = vec![];
        for i in 1..self.index_max + 1 {
            let n = node.derive(slip10::FIRST_HARDENED_INDEX + i as u32)?;
            let (p, s) = n.keypair();
            out.push(Keypair {
                public: p.bytes().into(),
                secret: s.bytes().into(),
            });
        }
        return Ok(out);
    }
}
