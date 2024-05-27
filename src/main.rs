use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener};
use crate::requisition::Requisition;

mod requisition;
mod ftp_server;
mod commands;

fn main() -> std::io::Result<()> {
    let mut file = File::create("teste.txt")?;
    file.write_all(b"cawercv")?;
    
    let listener = TcpListener::bind("127.0.0.1:0");
    
    
    for stream in listener.unwrap().incoming() {
        resolve_requisition(stream.unwrap().read(&mut [0u8; 1000])?);
    }
    
    Ok(())
}

fn resolve_requisition(data: usize) {
    let binding = [data as u8];
    let json = &*String::from_utf8_lossy(&binding);
    let requisition: Requisition = serde_json::from_str(json).expect("deu ruim no json");
    let command = requisition.command();
    
    match command { 
        "list" => commands::list(requisition),
        _ => println!("Método não reconhecido")
    }
}
