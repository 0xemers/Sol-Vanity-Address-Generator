use rayon::prelude::*;
use solana_sdk::{
    signature::Signer,
    signer::keypair::Keypair
};
use std::time::Instant;
use std::sync::atomic::{AtomicUsize, Ordering};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 地址前缀
    #[arg(short, long, default_value = "YOU")]
    prefix: String,

    /// 地址后缀
    #[arg(short, long, default_value = "XX")]
    suffix: String,
    
    /// 使用的线程数量 (默认使用全部可用核心)
    #[arg(short, long)]
    threads: Option<usize>,
}

fn main() {
    let args = Args::parse();
    
    // 如果指定了线程数，配置Rayon线程池
    if let Some(thread_count) = args.threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(thread_count)
            .build_global()
            .expect("无法设置全局线程池");
        println!("使用 {} 个线程", thread_count);
    } else {
        println!("使用所有可用核心 (默认)");
    }
    
    println!("正在搜索以 '{}' 开头且以 '{}' 结尾的地址...", args.prefix, args.suffix);
    
    let start = Instant::now();
    let attempt_count = AtomicUsize::new(0);
    
    let found = (0..u64::MAX)
        .into_par_iter()
        .find_map_any(|_| {
            let count = attempt_count.fetch_add(1, Ordering::Relaxed);
            if count % 100000 == 0 {
                let elapsed = start.elapsed();
                let rate = count as f64 / elapsed.as_secs_f64();
                println!("已尝试: {} 个地址 (速度: {:.2} 地址/秒)", count, rate);
            }
            
            let keypair = Keypair::new();
            let pubkey = keypair.pubkey().to_string();
            
            if pubkey.starts_with(&args.prefix) && pubkey.ends_with(&args.suffix) {
                Some((pubkey, keypair.to_base58_string()))
            } else {
                None
            }
        });

    match found {
        Some((address, private_key)) => {
            println!("\n✅ 生成耗时: {:.2}s", start.elapsed().as_secs_f32());
            println!("尝试次数: {}", attempt_count.load(Ordering::Relaxed));
            println!("地址: {}", address);
            println!("私钥: {}", private_key);
        }
        None => println!("未找到匹配地址"),
    }
}
