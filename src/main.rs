use anyhow::Result;
use bip39::{Language, Mnemonic};
use gl_client::bitcoin::Network;
use gl_client::credentials::{Device, Nobody};
use gl_client::node::ClnClient;
use gl_client::pb::cln;
use gl_client::pb::cln::{Amount, AmountOrAny, amount_or_any};
use gl_client::scheduler::Scheduler;
use gl_client::signer::Signer;
use rand::Rng;
use std::env;

#[tokio::main]
async fn main() {
    // gl_init().await.unwrap();
    gl_connect().await.unwrap();
}

async fn gl_connect() -> Result<()> {
    let developer_cred_path = env::var("GL_CRED_PATH")?;
    let creds = Device::from_path(developer_cred_path);
    let scheduler = Scheduler::new(Network::Bitcoin, creds.clone()).await?;
    let mut node: ClnClient = scheduler.node().await?;

    let info = node
        .getinfo(cln::GetinfoRequest::default())
        .await?
        .into_inner();
    println!("{}", hex::encode(info.id)); // 0297a46e36b9f7e37952c87f11bf6aad2ac208fa3d06ab0355340721149faba9cc

    let amount = AmountOrAny {
        value: Some(amount_or_any::Value::Amount(Amount { msat: 10_000 })),
    };

    let invoice = (&mut node)
        .invoice(cln::InvoiceRequest {
            description: format!("desc_{}", rand::random::<u32>()),
            label: format!("label_{}", rand::random::<u32>()),
            amount_msat: Some(amount),
            ..cln::InvoiceRequest::default()
        })
        .await?
        .into_inner();
    println!("{}", invoice.bolt11);

    Ok(())
}

async fn gl_init() -> Result<()> {
    let developer_cert_path = env::var("GL_CRT_PATH")?;
    let developer_key_path = env::var("GL_KEY_PATH")?;
    let developer_seed_path = env::var("GL_SEED_PATH")?;
    let developer_cred_path = env::var("GL_CRED_PATH")?;

    let developer_cert = std::fs::read(developer_cert_path)?;
    let developer_key = std::fs::read(developer_key_path)?;
    let developer_seed = std::fs::read_to_string(developer_seed_path)?;

    let developer_creds = Nobody {
        cert: developer_cert,
        key: developer_key,
        ..Nobody::default()
    };

    let signer = Signer::new(
        hex::decode(developer_seed)?,
        Network::Bitcoin,
        developer_creds.clone(),
    )?;

    let scheduler = Scheduler::new(Network::Bitcoin, developer_creds).await?;

    // Passing in the signer is required because the client needs to prove
    // ownership of the `node_id`
    let registration_response = scheduler.register(&signer, None).await?;
    let device_creds = Device::from_bytes(registration_response.creds);

    std::fs::write(developer_cred_path, &device_creds.to_bytes())?;

    Ok(())
}

fn gen_seed() -> Result<()> {
    const EMPTY_PASSPHRASE: &str = "";
    let (mnemonic, seed) = seed(EMPTY_PASSPHRASE)?;
    println!("{}", mnemonic);
    println!("{}", hex::encode(seed));
    Ok(())
}

fn seed(pass: &str) -> Result<(String, Vec<u8>)> {
    let mut rng = rand::rng();
    let mut entropy = [0u8; 32];
    rng.fill_bytes(&mut entropy);

    // Seed phrase for user
    let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy)?;
    let phrase = mnemonic.words().collect::<Vec<_>>().join(" ");

    let seed = &mnemonic.to_seed(pass)[0..32]; // Only need the first 32 bytes
    Ok((phrase, Vec::from(seed)))
}
