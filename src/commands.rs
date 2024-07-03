use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use crate::json_sender::JsonSender;
use crate::requisition::Requisition;

const FILES: &str = "src/files";

pub fn list(mut connection: TcpStream) -> io::Result<()> {
    let mut files: Vec<String> = Vec::new();
    
    match fs::read_dir(FILES) {
        Ok(mut entries) => {
            if !entries.next().is_none() {
                for entry in entries {
                    let entry = entry?;
                    let path = entry.path();

                    files.push(path.file_name().unwrap().to_str().unwrap().parse().unwrap());
                }

                let json = JsonSender::make_response_json_to_list("list", "success", &files);
                JsonSender::send_json_to_client(json, &mut connection);
            } else {
                let json = JsonSender::make_response_json_with_reason("list", "fail", "No files in the system");
                JsonSender::send_json_to_client(json, &mut connection);
            }
            
        } Err(..) => {}
    }
    
    Ok(())
}

pub fn put(requisition: Requisition, mut connection: TcpStream) {
    let mut file_size_buf = [0; 8];
    connection.read_exact(&mut file_size_buf[..]).expect("TODO: panic message");
    let file_size = u64::from_le_bytes(file_size_buf);
    let mut file_buffer = vec![0; file_size as usize];
    connection.read_exact(&mut file_buffer).expect("TODO: panic message");
    
    let file_quality = verify_file_integrity(&file_buffer, requisition.hash().clone().unwrap());
    if !file_quality {
        let json = JsonSender::make_response_json(requisition.file_name().as_ref().unwrap(), "put", "fail");
        JsonSender::send_json_to_client(json, &mut connection);
        return;
    }

    let new_file_path = format!("{}{}{}", FILES.to_owned(), "/", requisition.file_name().clone().unwrap());
    let output_path = Path::new(&new_file_path);
    fs::write(output_path, file_buffer).expect("falha");

    let json = JsonSender::make_response_json(requisition.file_name().as_ref().unwrap(), "put", "success");
    JsonSender::send_json_to_client(json, &mut connection);
}

pub fn get(requisition: Requisition, mut connection: TcpStream) -> io::Result<()> {
    let file_path = FILES.to_owned() + "/" + &*requisition.file_name().as_ref().unwrap();
    
    match fs::read(file_path.clone()) {
        Ok(bytes) => {
            let local_hash = sha256::digest(&bytes);
            let json = JsonSender::make_response_json_to_get(requisition.file_name().as_ref().unwrap(), "get", local_hash);
            JsonSender::send_json_to_client(json, &mut connection);
            send_file_to_client(file_path, connection);
        } Err(..) => {
            let json = JsonSender::make_response_json(requisition.file_name().as_ref().unwrap(), "get", "fail");
            JsonSender::send_json_to_client(json, &mut connection);
        }
    }

    Ok(())
}

fn verify_file_integrity(file_data: &Vec<u8>, hash_to_verify: String) -> bool {
    let local_hash = sha256::digest(file_data);

    return if hash_to_verify == local_hash {
        true
    } else {
        false
    }
}

fn send_file_to_client(file_path: String, mut connection: TcpStream) {
    let file = File::open(file_path);

    let file_size = file.as_ref().unwrap().metadata().unwrap().len();
    let mut file_buffer = vec![0; file_size as usize];
    let bytes_read = file.as_ref().unwrap().read(&mut file_buffer).expect("REASON");

    connection.write_all(file_size.to_le_bytes().as_slice()).unwrap();
    connection.write_all(&file_buffer[..bytes_read]).unwrap();
}