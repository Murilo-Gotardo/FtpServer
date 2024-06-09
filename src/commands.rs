use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io;
use std::path::Path;

use base64::{Engine as _, engine::general_purpose};

use crate::requisition::Requisition;

const FILES: &str = "src/files";

pub fn list(requisition: Requisition) -> io::Result<()> {
    println!("Mostrando lista de arquivos ({})", requisition.command());
    
    for entry in fs::read_dir(FILES)? {
        let entry = entry?;
        let path = entry.path();
        println!("{:?}", path);
    }
    
    Ok(())
}

pub fn put(requisition: Requisition) {
    let file_quality = verify_file_integrity(requisition.file_data().unwrap(), requisition.hash().clone().unwrap());
    
    if !file_quality {
        println!("Arquivo corrompido, suspendendo gravacao");
        return;
    }
    
    let decoded_file = general_purpose::STANDARD
        .decode(&requisition.file_data().unwrap()).unwrap();
    
    let new_file_path = format!("{}{}{}", FILES.to_owned(), "/", requisition.file_name().clone().unwrap());
    
    let output_path = Path::new(&new_file_path);
    fs::write(output_path, decoded_file).expect("falha");
}

pub fn get() {

}

fn verify_file_integrity(file_data: &[u8], hash_to_verify: String) -> bool {
    let mut hasher = DefaultHasher::new();
    file_data.hash(&mut hasher);
    let local_hash = hasher.finish();

    return if hash_to_verify == local_hash.to_string() {
        true
    } else {
        false
    }
}