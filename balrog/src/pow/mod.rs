use sha3::{Digest, Sha3_256};

pub use errors::Error;

pub mod errors;

const MAX_NONCE: u64 = u64::MAX;

const PREFIX: &str = "Vega_SPAM_PoW";

pub fn make(block_hash: &str, tx_id: &str, difficulty: usize) -> Result<(u64, Vec<u8>), Error> {
    if difficulty > 256 {
        return Err(Error::InvalidDifficulty);
    }
    if tx_id.len() == 0 {
        return Err(Error::EmptyTxId);
    }
    if block_hash.len() != 64 {
        return Err(Error::InvalidBlockHash);
    }

    let mut nonce: u64 = 0;
    let mut h = vec![];
    while nonce < MAX_NONCE {
        let message = prepare_message(&block_hash, &tx_id, nonce);
        let mut hasher = Sha3_256::new();
        hasher.update(message);
        h = hasher.finalize().to_vec();
        if count_zeroes(&h) >= difficulty {
            break;
        }
        nonce += 1;
    }

    return Ok((nonce, h));
}

fn count_zeroes(h: &Vec<u8>) -> usize {
    let mut res = 0;
    for x in h.iter() {
        if *x == 0 {
            res += 8;
        } else {
            if x & 128 != 0x00 {
                break;
            }
            if x & 64 != 0x00 {
                res += 1;
                break;
            }
            if x & 32 != 0x00 {
                res += 2;
                break;
            }
            if x & 16 != 0x00 {
                res += 3;
                break;
            }
            if x & 8 != 0x00 {
                res += 4;
                break;
            }
            if x & 4 != 0x00 {
                res += 5;
                break;
            }
            if x & 2 != 0x00 {
                res += 6;
                break;
            }
            if x & 1 != 0x00 {
                res += 7;
            }
            break;
        }
    }

    return res;
}

fn prepare_message(block_hash: &str, tx_id: &str, nonce: u64) -> Vec<u8> {
    let mut out: Vec<u8> = vec![];
    out.extend_from_slice(PREFIX.as_bytes());
    out.extend_from_slice(block_hash.as_bytes());
    out.extend_from_slice(tx_id.as_bytes());
    out.extend_from_slice(&nonce.to_be_bytes());
    return out;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pow() {
        let block_hash = "2FB2146FC01F21D358323174BAA230E7DE61C0F150B7FBC415C896B0C23E50FF";
        let tx_id = "2E7A16D9EF690F0D2BEED115FBA13BA2AAA16C8F971910AD88C72B9DB010C7D4";

        let (nonce, _) = super::make(block_hash, tx_id, 2).unwrap();
        assert_eq!(nonce, 4);
    }
}
