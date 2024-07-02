use std::io::Write;
use std::net::TcpStream;
use serde_json::json;

pub struct JsonSender {
    
}

impl JsonSender {

    pub fn make_response_json_to_list(operation: &str, status: &str, files_list: &Vec<String>) -> String {
        let data = json!({
         "files_list": files_list,
         "operation": operation,
         "status": status,
        });

        let json_string = serde_json::to_string(&data).expect("Falha ao serializar JSON");

        return json_string
    }

    pub fn make_response_json_with_reason(operation: &str, status: &str, reason: &str) -> String {
        let data = json!({
         "operation": operation,
         "status": status,
         "reason": reason
        });

        let json_string = serde_json::to_string(&data).expect("Falha ao serializar JSON");

        return json_string
    }
    
    pub fn make_response_json(file_name: &String, operation: &str, status: &str) -> String {
        let data = json!({
         "file_name": file_name,
         "operation": operation,
         "status": status
        });

        let json_string = serde_json::to_string(&data).expect("Falha ao serializar JSON");

        return json_string
    }

    pub fn make_response_json_to_get(file_name: &String, operation: &str, status: &str, local_hash: String) -> String {
        let data = json!({
         "file_name": file_name,
         "operation": operation,
         "status": status,
         "hash": local_hash
        });

        let json_string = serde_json::to_string(&data).expect("Falha ao serializar JSON");

        return json_string
    }

    pub fn send_json_to_client(json: String, connection: &mut TcpStream) {
        let metadata = json.len() as u64;
        let bytes = json.as_bytes();

        connection.write_all(&metadata.to_le_bytes()).expect("TODO: panic message");
        connection.write_all(bytes).unwrap();
        connection.flush().unwrap()
    }
}
