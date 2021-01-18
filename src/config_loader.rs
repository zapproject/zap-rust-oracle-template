use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryInto;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;
use web3::types::{H160, H256, U256};

pub struct event_config {
    pub name: String,
    pub event_hash: String,
    pub address: String,
    pub abi_path: String,
    pub response_type: String,
    pub response_data:String,
    
}
impl event_config {
    fn new(
        _name: String,
        _event_hash: String,
        _address: String,
        _abi_path: String,
        _response_type: String,
        _response_data:String,
       
    ) -> event_config {
        event_config {
            name: _name,
            event_hash: _event_hash,
            address: _address,
            abi_path: _abi_path,
            response_type: _response_type,
            response_data:_response_data,
           
        }
    }
}
fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<Value, Box<Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as a SERDE Value type.
    let config = serde_json::from_reader(reader).unwrap();

    // Return the Value`.
    Ok(config)
}
//&'static String
pub fn convert_value_to_event_config(json: Value) -> (Vec<event_config>, Vec<H256>, Vec<H160>) {
    let config_array = &json["events"];
    let mut address = Vec::new();
    let mut event_sigs = Vec::new();
    let mut event_configs = Vec::new();
    let value_array_string = |args: &Value| -> Vec<String> {
        println!("{:?}", args);
        args.as_array()
            .unwrap()
            .into_iter()
            .map(|arg| String::from(arg.as_str().unwrap()))
            .collect()
    };

    config_array
        .as_array()
        .unwrap()
        .into_iter()
        .for_each(|event| {
            println!("{:?}", &event);
            event_configs.push(event_config::new(
                String::from(event["name"].as_str().unwrap()),
                String::from(event["event_hash"].as_str().unwrap()),
                String::from(event["address"].as_str().unwrap()),
                String::from(event["abi_path"].as_str().unwrap()),
                String::from(event["response_type"].as_str().unwrap()),
                String::from(event["response_data"].as_str().unwrap()),
               
            ));
            println!("{:?}", event["address"].to_string());

            let temp: H160 = event["address"].as_str().unwrap().parse().unwrap();

            let temp2: H256 = event["event_hash"].as_str().unwrap().parse().unwrap();

            address.push(H160::from(temp));
            event_sigs.push(H256::from(temp2));
        });
    (event_configs, event_sigs, address)
}
pub fn load_config() -> (Vec<event_config>, Vec<H256>, Vec<H160>) {
    let raw_values = read_config_from_file("./config.json").unwrap();
    convert_value_to_event_config(raw_values)
}
