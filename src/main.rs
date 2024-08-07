extern crate core;
use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};
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
                    Ok(mut stream) => {
                        thread::spawn(move || {
                            resolve_requisition(&mut stream);
                        });
                    } Err(e) => println!("deu ruim")
                }
            }
        },
        Err(e) => eprintln!("Falha ao pegar ip local: {}", e),
    }
    
    Ok(())
}

fn resolve_requisition(connection: &mut TcpStream) {
    loop {
        let mut metadata_buffer = [0; 8];
        match connection.read(&mut metadata_buffer[..]) {
            Ok(0) => {
                break
            }
            Ok(_) => {
                let metadata_length = u64::from_le_bytes(metadata_buffer);

                let mut buffer = vec![0; metadata_length as usize];
                connection.read(&mut buffer).expect("TODO: panic message");

                let json = String::from_utf8_lossy(&buffer[..]);
                let cleaned_json = json.trim_end_matches('\0');
                let requisition = serde_json::from_str::<Requisition>(&cleaned_json).expect("cwe");

                match requisition.command() {
                    "list" => commands::list(connection).unwrap(),
                    "put" => commands::put(requisition, connection),
                    "get" => commands::get(requisition, connection).unwrap(),
                    _ => println!("Método não reconhecido")
                }

            } Err(e) => {
                println!("Erro ao ler da conexão: {}", e);
                break
            }
        }
    }

    if let Err(e) = connection.shutdown(Shutdown::Both) {
        println!("Erro ao fechar a conexão: {}", e);
    }
}