extern crate core;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;
use local_ip_address::local_ip;
use crate::requisition::Requisition;
mod requisition;
mod commands;
mod json_sender;

fn main() -> std::io::Result<()> {
    match local_ip() {
        Ok(ip) => {
            println!("Running on {}:11000", ip);
            let listener = TcpListener::bind(ip.to_string() + ":11000");

            for stream in listener.unwrap().incoming() {
                match stream {
                    Ok(stream) => {
                        thread::spawn(|| {
                            resolve_requisition(stream);
                        });
                    } Err(e) => println!("deu ruim")
                }
            }
        },
        Err(e) => eprintln!("Falha ao pegar ip local: {}", e),
    }
    
    Ok(())
}

fn resolve_requisition(mut connection: TcpStream) {
    
    let mut metadata_buffer = [0; 8];
    connection.read_exact(&mut metadata_buffer[..]).expect("TODO: panic message");
    let metadata_length = u64::from_le_bytes(metadata_buffer);

    let mut buffer = vec![0; metadata_length as usize];
    connection.read(&mut buffer).expect("TODO: panic message");
  
    let json = String::from_utf8_lossy(&buffer[..]);
    let cleaned_json = json.trim_end_matches('\0');
    let requisition = serde_json::from_str::<Requisition>(&cleaned_json).expect("cwe");
    
    return match requisition.command() {
        "list" => commands::list(connection).unwrap(),
        "put" => commands::put(requisition, connection),
        "get" => commands::get(requisition, connection).unwrap(),
        _ => println!("Método não reconhecido")
    }
}