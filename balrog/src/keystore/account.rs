use super::Error;
use bip39::{Language, Mnemonic};
use slip10::Node;

const MAGIC_NB: u32 = 1789;
const ORGIN_INDEX: u32 = slip10::FIRST_HARDENED_INDEX + MAGIC_NB;

pub struct Account {
    node: Node,
    index_max: u64,
}

impl Account {
    pub fn import(mnemonic: &str) -> Result<Account, Error> {
        let m = match Mnemonic::parse_in(Language::English, mnemonic) {
            Ok(m) => m,
            Err(e) => {
                return Err(Error::Bip39(e));
            }
        };
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
        let node = match Node::new_master_node(&seed) {
            Ok(n) => n,
            Err(e) => {
                return Err(Error::Slip10(e));
            }
        };
        let node = node.derive(ORGIN_INDEX).unwrap();
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

    pub fn keypairs(&self) -> () {}
}
