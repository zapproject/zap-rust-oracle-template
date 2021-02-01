mod config_loader;
mod event_decoder;
use config_loader::{event_config, load_config};
use dotenv::dotenv;
use event_decoder::decode_log;
use serde_json::Value;
use std::collections::HashMap;

use tiny_keccak::Keccak;

use web3::futures::{future, StreamExt};
use web3::types::{FilterBuilder, Log, H256,H160};
use std::str::FromStr;
extern crate rustc_hex;
use run_script::ScriptOptions;
use rustc_hex::ToHex;
use std::env;
extern crate eth_checksum;
use async_std::{net::TcpStream, prelude::*};
use io_arc::IoArc;

extern crate dotenv_codegen;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();

    println!("{:?}", &args);

    let provider = &args[2];

    let socket_port = &args[3];
    println!("{:?}", &provider);
    let mut event_mapping = HashMap::new();

    let web3 = web3::Web3::new(web3::transports::WebSocket::new(provider).await?);

    let (events, signatures, addresses) = load_config(&args[1]);

    events.into_iter().enumerate().for_each(|(_key, item)| {
        println!("{:?}", &item.event_hash);
        println!("{:?}", &item.address);
        println!("{:?}", eth_checksum::checksum( &item.address));
        let address_string=H160::from_str(&item.address).unwrap();
        println!("{:?}",eth_checksum::checksum(&format!("{:x}", address_string)).as_str());
        let hash_key = event_key(&item.event_hash,  eth_checksum::checksum(&format!("{:x}", address_string)).as_str() );
        println!("the hash key is {:?}", hash_key);
        event_mapping.insert(hash_key, item);
    });
    //println!("{:?}",&addresses);
    // println!("{:?}",&signatures);
    let filter = FilterBuilder::default()
        .address(addresses)
        .topics(Some(signatures), None, None, None)
        .build();
    println!("{:?}", &filter);
    let sub = web3.eth_subscribe().subscribe_logs(filter).await?;
    let stream = async_std::net::TcpStream::connect(socket_port).await;

    let arc_stream = IoArc::new(stream.unwrap());

    sub.for_each(|log| async {
        let _log = log;
        let (post_body, response_type, response_data) =
            process_log(_log.unwrap(), &event_mapping).await;
        let _response_data = response_data.clone();
        let _post = post_body.clone();

        match response_type.as_str() {
            "http_post" => {
                tokio::spawn(async move {
                    make_post(&_response_data, _post).await;
                });
            }
            "web_socket" => {
                let mut c = arc_stream.clone();
                tokio::spawn(async move {
                    let post_string = serde_json::to_vec(&_post).unwrap();
                    println!("{:?}", &c);
                    c.write(&post_string).await;
                    //
                });
            }
            "shell_script" => println!("socket selected"),

            _ => println!("ERROR"),
        }

        future::ready(()).await
    })
    .await;

    Ok(())
}

/*fn writeData(socket:IoArc<async_std::net::TcpStream>,data:[u8]){

}
*/
fn event_key(sig: &String, address: &str) -> String {
    let mut result = [0u8; 32];
    let mut n = String::new();
    n.push_str(address);
    n.push_str(sig);
    let data = n.replace(" ", "").into_bytes();
    let mut sponge = Keccak::new_keccak256();
    sponge.update(&data);
    sponge.finalize(&mut result);
    format!("{:x}", H256::from_slice(&result))
}

async fn process_log<'a>(
    log: Log,
    map: &'a HashMap<String, event_config>,
) -> (Value, &'a String, &'a String) {
    let event = log;
    let event_copy = event.clone();
    println!("got log: {:?}", &event);
    let topics: Vec<String> = event_copy
        .topics
        .into_iter()
        .map(|topic| format!("{:x}", topic))
        .collect();
    println!("{:?}", &topics[0]);
    println!("{:?}", eth_checksum::checksum(&format!("{:x}", &event.address)).as_str());

    let key = event_key(&topics[0],  eth_checksum::checksum(&format!("{:x}", &event.address)).as_str() );
    println!("the key is {:?}", key);

    let config = map.get(&key).unwrap();
    let decoded_log = decode_log(
        &config.abi_path,
        &config.name,
        topics,
        &event.data.0.to_hex::<String>(),
    )
    .unwrap();

    let post_body = serde_json::json!(&decoded_log);

    (post_body, &config.response_type, &config.response_data)
}
fn run_process(p: String, data: String) {
    let options = ScriptOptions::new();
    let (code, output, error) = run_script::run_script!(p, &options).unwrap();
    println!("{:?}", data);
    println!("Exit Code: {}", code);
    println!("Output: {}", output);
    println!("Error: {}", error);
}
async fn make_post(
    url: &String,
    body: Value,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let echo_json: serde_json::Value = reqwest::Client::new()
        .post(url)
        .json(&body)
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", &echo_json);

    Ok(echo_json)
}
#[test]
fn Incoming_log_decode_stringify() {
    let abi = "./eventsABI/Incoming.abi";
    let name = "Incoming";
    let topic = [
        String::from("69741cc3ec0270f258feb6b53b42ef1e7d2251a3c8eea4f6ba1f72bd4b7beba7"),
        String::from("dd01e0d1e313c493bd8dcb841088d6d6bcbca3b0c3cfe6d0c76df566f0b2577d"),
        String::from("000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266"),
        String::from("0000000000000000000000009a9f2ccfde556a7e9ff0848998aa4a0cfd8863ae"),
    ];
    let data = "00000000000000000000000000000000000000000000000000000000000000800da72197e898ebe1814471a76048ed137a089f595c1e7038f2b70e98645e765200000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000571756572790000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002273c58c66704975459b45d17d8d262d7b6de59f5e57291ea5af890e2cf11a2b808455b9a244e3f6e53a4d620c3d1261bb351f203ccdf11324cfed2696b108913";
    //let expected = [event_value::new(String::from("id"),String::from("dd01e0d1e313c493bd8dcb841088d6d6bcbca3b0c3cfe6d0c76df566f0b2577d")),event_value::new_from_str("provider","f39fd6e51aad88f6f4ce6ab8827279cfffb92266"),event_value::new_from_str("query","query"),event_value::new_from_str("endpoint","0da72197e898ebe1814471a76048ed137a089f595c1e7038f2b70e98645e7652" ),event_value::new_from_str("endpointParams","[273c58c66704975459b45d17d8d262d7b6de59f5e57291ea5af890e2cf11a2b8,08455b9a244e3f6e53a4d620c3d1261bb351f203ccdf11324cfed2696b108913]")];

    let decoded = decode_log(abi, name, topic.to_vec(), data).unwrap();
    println!("{:?}", decoded);
    let post_body = serde_json::json!(&decoded);
    let post_string = serde_json::to_vec(&post_body).unwrap();
    println!("{:?}", post_body.to_string());
    println!("{:?}", post_string);
}
#[test]

fn Send_Decoded_Event_websocket() {
    use std::io::prelude::*;
    use std::net::TcpStream as syncTcpStream;
    let abi = "./eventsABI/Incoming.abi";
    let name = "Incoming";
    let topic = [
        String::from("69741cc3ec0270f258feb6b53b42ef1e7d2251a3c8eea4f6ba1f72bd4b7beba7"),
        String::from("dd01e0d1e313c493bd8dcb841088d6d6bcbca3b0c3cfe6d0c76df566f0b2577d"),
        String::from("000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266"),
        String::from("0000000000000000000000009a9f2ccfde556a7e9ff0848998aa4a0cfd8863ae"),
    ];
    let data = "00000000000000000000000000000000000000000000000000000000000000800da72197e898ebe1814471a76048ed137a089f595c1e7038f2b70e98645e765200000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000571756572790000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002273c58c66704975459b45d17d8d262d7b6de59f5e57291ea5af890e2cf11a2b808455b9a244e3f6e53a4d620c3d1261bb351f203ccdf11324cfed2696b108913";

    let decoded = decode_log(abi, name, topic.to_vec(), data).unwrap();
    let mut stream = syncTcpStream::connect("127.0.0.1:3007").unwrap();
    let post_vec = serde_json::to_vec(&decoded).unwrap();
    //println!("{:?}",&c);
    stream.write(&post_vec);
}
