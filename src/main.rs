mod config_loader;
mod event_decoder;
use config_loader::{event_config, load_config};
use dotenv::dotenv;
use event_decoder::{decode_log, decode_params};
use serde_json::Value;
use std::collections::HashMap;

use tiny_keccak::Keccak;
use web3::contract::tokens::{Detokenize, Tokenize};
use web3::contract::{Contract, Options};
use web3::futures::{future, StreamExt};
use web3::types::{Address, BlockNumber, FilterBuilder, Log, H256, U256};
extern crate rustc_hex;
use run_script::ScriptOptions;
use rustc_hex::ToHex;
use tokio::task;
use async_std::{
   
    net::{TcpListener, TcpStream, ToSocketAddrs},
    prelude::*,    
};
use io_arc::IoArc;
#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("{:?}",currprice);
    dotenv().ok();
    //println!("{}", dotenv!("PRIVATE_KEY"));
    let provider = dotenv!("WEB3_PROVIDER");
    let socket_port = dotenv!("PORT");
    println!("{:?}",&provider);
    let mut event_mapping = HashMap::new();

    let web3 = web3::Web3::new(web3::transports::WebSocket::new(provider).await?);

    let (events, signatures, addresses) = load_config();

    events.into_iter().enumerate().for_each(|(key, item)| {
        println!("{:?}", &item.event_hash);
        println!("{:?}", &item.address);
        let hash_key = event_key(&item.event_hash, &item.address);
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
    let mut stream = TcpStream::connect(socket_port).await;
    
    let  mut arc_stream = IoArc::new(stream.unwrap());

    sub.for_each(|log| async {
        let _log=log;
        let (post_body,response_type,response_data)=process_log(_log.unwrap(),&event_mapping).await;
        let _response_data=response_data.clone();
        let _post=post_body.clone();
        
        match response_type.as_str(){
            "http_post"=>{
                tokio::spawn(async move{
                    make_post( &_response_data, _post).await;
                });

            },
            "web_socket"=>{
                
                let mut  c=arc_stream.clone();
                tokio::spawn(async move{
                    
                    let post_string=serde_json::to_vec(&_post).unwrap();
                    println!("{:?}",&c);
                    c.write(&post_string).await;
                   //
                 }); 
            },
            "shell_script"=>println!("socket selected"),

            _=>println!("ERROR"),
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
    log:   Log,
    map:  &'a  HashMap<String,event_config>,
   
) ->(  Value,&'a String,&'a String){
    let Event = log;
    let event_copy = Event.clone();
    println!("got log: {:?}", &Event);
    let mut topics: Vec<String> = event_copy
        .topics
        .into_iter()
        .map(|topic| format!("{:x}", topic))
        .collect();
    println!("{:?}", &topics[0]);
    println!("{:?}", &format!("{:x}", &Event.address));

    let key = event_key( &topics[0], &format!("{:x}", &Event.address));
    println!("the key is {:?}", key);


    /**let web3 = web3::Web3::new((web3::transports::WebSocket::new(dotenv!("INFURA")).await).unwrap());
    let private_key: &[u8] = dotenv!("PRIVATE_KEY").as_bytes();

    let secret_key:SecretKey = SecretKey::from_slice(&hex::decode(private_key).unwrap()).unwrap();
    let signed_event=web3.accounts().sign(String::from("hello").into_bytes(),&secret_key  );
    **/
    let config = map.get(&key).unwrap();
    let decoded_log = decode_log(
        &config.abi_path,
        &config.name,
        topics,
        &Event.data.0.to_hex::<String>(),
    )
    .unwrap();
  /**  let body=auth_post{
        hash_signature:signed_event,
        event_values:decoded_log
    };
    **/
    let post_body = serde_json::json!(&decoded_log);
  //  println!("{:?}", &post_body);
    

   (post_body,&config.response_type,&config.response_data)
    
}
fn run_process(p:String,data:String){
    let options = ScriptOptions::new();
    let (code, output, error) = run_script::run_script!(
        p,
        &options
    )
    .unwrap();

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
