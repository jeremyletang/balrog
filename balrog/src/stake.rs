use crate::client::DatanodeV2BlockingClient;
use crate::errors::Error;
use crate::keystore;
use crate::util::format_balance;
use tabled::{object::Segment, Alignment, Modify, Table, Tabled};

#[derive(Tabled)]
struct Stake {
    public_key: String,
    balance: String,
}

pub fn show(network: &str, info: &keystore::KeystoreInfo) -> Result<(), Error> {
    let mut pks = vec![];

    for key in info.keypairs.iter() {
        pks.push(hex::encode(&key.public));
    }
    return show_pks(network, pks);
}

pub fn show_pks(network: &str, pks: Vec<String>) -> Result<(), Error> {
    let mut clt = DatanodeV2BlockingClient::connect(network.to_string())?;
    let mut accounts = vec![];

    for pkey in pks.iter() {
        let pstake = clt.get_party_stake(&*pkey)?;
        accounts.push(Stake {
            public_key: pkey.clone(),
            balance: format_balance(pstake.current_stake_available, 18),
        });
    }

    print!(
        "{}",
        Table::new(accounts)
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .to_string()
    );
    return Ok(());
}
