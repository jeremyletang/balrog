pub fn format_balance(mut balance: String, decimals: u64) -> String {
    let udecs = decimals as usize;
    let mut len = balance.len();
    if len <= udecs {
        while len < udecs {
            balance.insert_str(0, "0");
            len += 1;
        }
        balance.insert_str(0, "0.");
    } else {
        let mut pos = len - udecs;
        balance.insert_str(pos, ".");
        // pos += ;
        while pos > 3 {
            pos -= 3;
            balance.insert_str(pos, ",")
        }
    }

    // then remove trailing 0s
    let mut balance = balance.trim_end_matches('0').to_string();
    if balance.ends_with('.') {
        balance = balance.trim_end_matches('.').to_string();
    }
    return balance;
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
        assert_eq!("0.0", super::format_balance("0".to_string(), 18),);
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
        assert_eq!(
            "10,198,232,640.2159686835285",
            super::format_balance("10198232640215968683528500000".to_string(), 18),
        );
    }
}
