use core::num;
use std::io;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL,pubkey,  pubkey::Pubkey,signature::Keypair, signer::Signer};

use anyhow::Result;


#[tokio::main]
async fn main() {
    menu().await;
    struct wallet{
        pubkey: String,
        number: u8,
        privkey: String,

    
    };
}


async fn menu() {
    loop {
        println!("📊 Welcome to Neptune Dashboard for Solana");
        println!("1. Check wallet balance");
        println!("2. Create wallet");
        println!("3. See wallet list");
        println!("Type 'exit' to quit");

        let mut user_choice = String::new();
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Unable to read input");

        let user_choice = user_choice.trim();

        if user_choice == "1" {
            // balance() se termine quand l'utilisateur tape "end"
            adress_balance().await.unwrap();

        } else if user_choice == "2" {
        create_wallet();

        } else if user_choice == "3" {
        println!("Printing wallet list...");
        

        }else if user_choice.eq_ignore_ascii_case("exit") {
            println!("👋 Goodbye !");
            break;
        } else {
            println!("❌ Invalid input. Please try again.");
            print!("\x1B[2J\x1B[H");
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
            print!("\x1B[2J\x1B[H");

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


fn create_wallet(){
    println!("Enter number of wallets to create 'end' to quit :");

    let mut wallet_to_create = String::new();
    io::stdin()
        .read_line(&mut wallet_to_create)
        .expect("Failed to read line");

    if wallet_to_create.trim() == "end" {
    println!("↩️ Returning to main menu...");
    print!("\x1B[2J\x1B[H");}

    else{

    let number: i32 = wallet_to_create
        .trim()
        .parse()
        .expect("Please enter a valid integer");


    println!("Creating {} wallets",number); 
    for i in 1..number+1{
    
    
    
    let keypair = Keypair::new();
    let address = keypair.pubkey();

    println!(" Wallet {} : {address}",i)
    }
}
}
