use crate::client::DatanodeV2BlockingClient;
use crate::errors::Error;
use crate::keystore;
use crate::util::format_balance;
use std::collections::{HashMap, HashSet};
use tabled::{object::Segment, Alignment, Modify, Table, Tabled};
use vega_rust_sdk::vega::AccountType;

#[derive(Tabled)]
struct Balance {
    public_key: String,
    account_type: String,
    asset: String,
    market: String,
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
    let mut asset_ids = HashSet::new();
    let mut accounts = vec![];
    let mut assets = HashMap::new();

    for pkey in pks.iter() {
        let res = clt.get_account(&*pkey)?;
        for account in res.get_ref().accounts.iter() {
            asset_ids.insert(account.asset.clone());
            accounts.push(account.clone());
        }
    }

    for id in asset_ids.iter() {
        let mut res = clt.get_asset(&*id)?;
        let details = res
            .get_mut()
            .asset
            .as_ref()
            .unwrap()
            .details
            .as_ref()
            .unwrap()
            .clone();
        assets.insert(id.clone(), details);
    }

    let mut balances = vec![];
    for account in accounts.iter() {
        let symbol = assets[&*account.asset].symbol.clone();
        let dec = assets[&*account.asset].decimals;
        balances.push(Balance {
            public_key: account.owner.clone(),
            account_type: account_type_to_string(account.r#type()),
            asset: symbol,
            balance: format_balance(account.balance.to_string(), dec),
            market: account.market_id.clone(),
        })
    }

    print!(
        "{}",
        Table::new(balances)
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .to_string()
    );
    return Ok(());
}

fn account_type_to_string(atype: AccountType) -> String {
    match atype {
        AccountType::Margin => "margin".to_string(),
        AccountType::General => "general".to_string(),
        AccountType::Bond => "bond".to_string(),
        _ => "unsupported".to_string(),
    }
}
