use crate::client::DatanodeV2BlockingClient;
use crate::errors::Error;
use crate::keystore;
use std::collections::{HashMap, HashSet};
use tabled::{Table, Tabled};
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
    let mut clt = DatanodeV2BlockingClient::connect(network.to_string())?;
    let mut asset_ids = HashSet::new();
    let mut accounts = vec![];
    let mut assets = HashMap::new();

    for key in info.keypairs.iter() {
        let pkey = hex::encode(&key.public);
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

    print!("{}", Table::new(balances).to_string());
    return Ok(());
}

fn format_balance(mut balance: String, decimals: u64) -> String {
    let mut udecs = decimals as usize;
    let mut len = balance.len();
    if len <= udecs {
        while len < udecs {
            balance.insert_str(0, "0");
            len += 1;
        }
        balance.insert_str(0, "0.");
        return balance;
    }

    let mut pos = len - udecs;
    balance.insert_str(pos, ".");
    // pos += ;
    while pos > 3 {
        pos -= 3;
        balance.insert_str(pos, ",")
    }

    return balance;
}

fn account_type_to_string(atype: AccountType) -> String {
    match atype {
        AccountType::Margin => "margin".to_string(),
        AccountType::General => "general".to_string(),
        AccountType::Bond => "bond".to_string(),
        _ => "unsupported".to_string(),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_format_balance() {
        assert_eq!(
            "0.011111111111111111",
            super::format_balance("11111111111111111".to_string(), 18),
        );
        assert_eq!(
            "0.111111111111111111",
            super::format_balance("111111111111111111".to_string(), 18),
        );
        assert_eq!(
            "0.000000000000000000",
            super::format_balance("0".to_string(), 18),
        );
        assert_eq!(
            "0.000000000000000001",
            super::format_balance("1".to_string(), 18),
        );
        assert_eq!(
            "2,640.215968683528541617",
            super::format_balance("2640215968683528541617".to_string(), 18),
        );
        assert_eq!(
            "10,198,232,640.215968683528541617",
            super::format_balance("10198232640215968683528541617".to_string(), 18),
        );
    }
}
