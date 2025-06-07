use std::io;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    loop {
        let mut wallet = String::new();

        println!("Input wallet to scan (or type 'end' to quit):");
        io::stdin()
            .read_line(&mut wallet)
            .expect("Unable to read stdin");

        let wallet = wallet.trim(); // Enlève le \n et les espaces

        if wallet == "end" {
            println!("Goodbye !");
            break;
        }

        println!("🔍 Scanning: {}", wallet);

        // Conversion en Pubkey
        let address = match wallet.parse::<Pubkey>() {
            Ok(pubkey) => pubkey,
            Err(_) => {
                println!("❌ Invalid wallet address.");
                continue;
            }
        };

        let client = RpcClient::new_with_commitment(
            "https://api.mainnet-beta.solana.com".to_string(),
            CommitmentConfig::confirmed(),
        );

        match client.get_balance(&address).await {
            Ok(balance) => {
                println!("💰 Balance: {:.3} SOL", balance as f64 / LAMPORTS_PER_SOL as f64);
            }
            Err(e) => {
                println!("❌ Failed to fetch balance: {}", e);
            }
        }
    }

    Ok(())
}
