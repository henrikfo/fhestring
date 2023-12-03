use std::{env, error};
use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint16, FheUint32, FheUint8};

// mod cipertext;

fn main() -> Result<(), Box<dyn error::Error>> {
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::all_disabled()
        .enable_default_integers()
        .build();

    // Key generation
    let (client_key, server_keys) = generate_keys(config);

    // Get string from command line
    // let args: Vec<String> = env::args().collect();
    // let content = &args[1];
    // let pattern = &args[2];

    // Parse string and encode to ASCII representation
    let string = "AbS0";
    let asc = string.chars()
        .map(|s| s as u8)
        .collect::<Vec<_>>();
    println!("{:?}", asc.as_bytes());

    // let clear_a = 1344u32;
    // let clear_b = 5u32;

    // Encrypting the input data using the (private) client_key
    // FheUint32: Encrypted equivalent to u32
    // let mut encrypted_a = FheUint16::try_encrypt(clear_a, &client_key)?;
    // let encrypted_b = FheUint16::try_encrypt(clear_b, &client_key)?;


    // assert_eq!(clear_res, 1_u8);

    Ok(())
}

    // Encrypt string