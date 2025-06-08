use std::io;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};

#[tokio::main]
async fn main() {
    menu().await;
}


async fn menu() {
    loop {
        println!("📊 Welcome to Neptune Dashboard for Solana");
        println!("1. Check wallet balance");
        println!("2. Scan token");
        println!("Type 'exit' to quit");

        let mut user_choice = String::new();
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Unable to read input");

        let user_choice = user_choice.trim();

        if user_choice == "1" {
            // balance() se termine quand l'utilisateur tape "end"
            adress_balance().await.unwrap();
        } else if user_choice.eq_ignore_ascii_case("exit") {
            println!("👋 Goodbye !");
            break;
        } else {
            println!("❌ Invalid input. Please try again.");
        }

        println!(); // Un peu d'espace entre les cycles
    }
}

async fn adress_balance() -> anyhow::Result<()> {
    loop {
        let mut wallet = String::new();

        println!("🔍 Input wallet to scan (or type 'end' to return to menu):");
        io::stdin()
            .read_line(&mut wallet)
            .expect("Unable to read stdin");

        let wallet = wallet.trim();

        if wallet == "end" {
            println!("↩️ Returning to main menu...");
            break; // On quitte la boucle -> retour au menu
        }

        println!("📡 Scanning pubkey: {}", wallet);

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


async fn token_balance() -> Result<()> {
    loop{
        let mut ca = String::new();

        println!("🔍 Input token to scan (or type 'end' to return to menu):");
        io::stdin()
            .read_line(&mut ca)
            .expect("Unable to read stdin");

        let ca = ca.trim();

        if ca == "end" {
            println!("↩️ Returning to main menu...");
            break; // On quitte la boucle -> retour au menu
        }

        println!("📡 Scanning contract adress: {}", ca);
    let client = RpcClient::new_with_commitment(
        String::from("https://api.devnet.solana.com"),
        CommitmentConfig::confirmed(),
    );
}

    /*let mint = pubkey!(&ca);

    let token_supply = client.get_token_supply(&mint).await?;

    println!("{:#?}", token_supply);
    }
    Ok(())*/
