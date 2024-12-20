use base64::encode;
use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::{ChaCha20, Key, Nonce};
use hex::decode;
use rand::{Rng, RngCore};
use sha2::{Digest, Sha256};
use core::str;
use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    process::exit,
};

fn main() {
    println!("Welcome to Encrypto! Encrypt your txt files to be stored on any platform you want");
    menu();
}

fn menu() {
    println!("Menu:");
    println!("1) Encrypt File");
    println!("2) Decrypt File");
    println!("Anything else to exit");
    match read_input().as_str() {
        "1\n" => encrypt(),
        "2\n" => decrypt(),
        _ => println!("Exiting..."),
    }
}

fn read_input() -> String {
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");
    value
}

fn read_file_() -> io::Result<Vec<u8>> {
    //FILE NAME INPUT
    let input_path = "src/resources/input.txt";
    let mut file = File::open(input_path).expect("Failed to open file"); //CHECK HOW TO HANDLE THIS CASE MORE CLEANLY
                                                                         // let mut content = String::new();
                                                                         // match file.read_to_string(&mut content) {
                                                                         //     Ok(_) => {}
                                                                         //     Err(e) => {
                                                                         //         println!("Error reading content from file {}. Exiting...", e);
                                                                         //         exit(1)
                                                                         //     }
                                                                         // };
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
    //content
}

fn write_file(content: String) {
    //FILE NAME OUTPUT
    let output_path = "src/resources/output.txt";
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(output_path)
        .expect("Failed to open file"); //CHECK HOW TO HANDLE BETTER
    match file.write_all(content.as_bytes()) {
        Ok(_) => println!("Written content to output.txt"),
        Err(e) => {
            println!("Error writing content to file {}. Exiting...", e);
            exit(1)
        }
    };
}

fn encrypt() {
    println!("encrypt!");
    let content = match read_file_() {
        Ok(data) => data,
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };
    println!("Enter Key");
    let key = read_input();
    let sha_key = string_to_sha256(key);
    let (encrypted_base64, encrypted_nonce) = encrypt_with_chacha(&sha_key, &content);
    let final_content = encrypted_base64 + "|||{}{}{}|||" + encrypted_nonce.as_str();
    write_file(final_content); //NEED TO ALSO ADD NONCE WHICH WILL LATER BE EXTRACTED
}

fn encrypt_with_chacha(key: &[u8; 32], content: &[u8]) -> (String, String) {
    let mut nonce = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);

    // Create the ChaCha20 cipher
    let mut cipher = ChaCha20::new(&Key::from_slice(key), &Nonce::from_slice(&nonce));

    // Encrypt the content by applying the keystream
    let mut encrypted_data = content.to_vec();
    cipher.apply_keystream(&mut encrypted_data);

    // Optionally: Encode the encrypted data as Base64 if you need to store or transmit it as text
    let encrypted_base64 = encode(&encrypted_data);
    let encrypted_nonce = encode(&nonce);
    // println!("Encrypted data (Base64): {}", encrypted_base64);

    (encrypted_base64, encrypted_nonce)
}

fn decrypt() {
    println!("decrypt!");
    let content = match read_file_() {
        Ok(data) => data,
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };
    println!("Enter Key");
    let key = read_input();
    let sha_key = string_to_sha256(key);
    let (encrypted_base64, encrypted_nonce) = extract_strings(&content);
    let decrypted_content = decrypt_from_chacha(encrypted_base64, encrypted_nonce, &sha_key);
    write_file(decrypted_content);
        
}

fn decrypt_from_chacha(
    encrypted_base64: String,
    encrypted_nonce: String,
    key: &[u8; 32],
) -> String {
    let encrypted_data =
        base64::decode(&encrypted_base64).expect("Failed to decode encrypted data");
    let nonce = base64::decode(&encrypted_nonce).expect("Failed to decode nonce");

    // Ensure the nonce is the correct length (12 bytes for ChaCha20)
    if nonce.len() != 12 {
        panic!("Invalid nonce length: expected 12 bytes");
    }

    // Create the ChaCha20 cipher with the same key and nonce
    let mut cipher = ChaCha20::new(&Key::from_slice(key), &Nonce::from_slice(&nonce));

    // Decrypt the data by applying the keystream to the encrypted data
    let mut decrypted_data = encrypted_data.to_vec();
    cipher.apply_keystream(&mut decrypted_data);

    // Convert the decrypted byte data back to a string (assuming the original content was UTF-8)

    String::from_utf8(decrypted_data).expect("Failed to convert decrypted data to string")
}

fn extract_strings(vec_content: &[u8]) -> (String,String) {
    let input = str::from_utf8(vec_content).ok().unwrap(); //handle better
    let separator = "|||{}{}{}|||";
    match input.find(separator) {
        Some(index) => {
            let (encrypted_content,encrypted_nonce) = input.split_at(index);
            (encrypted_content.to_string(),encrypted_nonce[separator.len()..].to_string())
        }
        None => ("NA".to_string(),"NA".to_string())
    }
}

fn string_to_sha256(input: String) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    let hash_bytes: [u8; 32] = result
        .as_slice()
        .try_into()
        .expect("hash should be 32 bytes");
    hash_bytes
}
