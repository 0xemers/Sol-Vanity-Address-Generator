use rayon::prelude::*;
use solana_sdk::{
    signature::Signer,  // 必须导入 Signer trait
    signer::keypair::Keypair
};
use std::time::Instant;

const PREFIX: &str = "YOU";
const SUFFIX: &str = "XX";

fn main() {
    let start = Instant::now();
    let found = (0..u64::MAX)
        .into_par_iter()
        .find_map_any(|_| {
            let keypair = Keypair::new();
            let pubkey = keypair.pubkey().to_string(); // 现在可以正常调用
            
            if pubkey.starts_with(PREFIX) && pubkey.ends_with(SUFFIX) {
                Some((pubkey, keypair.to_base58_string()))
            } else {
                None
            }
        });

    match found {
        Some((address, private_key)) => {
            println!("\n✅ 生成耗时: {:.2}s", start.elapsed().as_secs_f32());
            println!("地址: {}", address);
            // println!("私钥: {}...{}", &private_key[..12], &private_key[private_key.len()-6..]);
            // 修改后（显示完整私钥）
            println!("私钥: {}", private_key); // 完整显示私钥
        }
        None => println!("未找到匹配地址"),
    }
}