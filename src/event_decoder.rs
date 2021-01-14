//use crate::error::Error;
use ethabi::param_type::{ParamType, Reader};
use ethabi::token::{LenientTokenizer, StrictTokenizer, Token, Tokenizer};
use ethabi::{decode, encode, Contract, Event, Function, Hash};
use rustc_hex::{FromHex, ToHex};
use serde::{Deserialize, Serialize};
use std::fs::File;
//use std::fs::File;
use tiny_keccak::Keccak;

#[derive(Serialize, Deserialize, Debug)]
pub struct event_value {
    name: String,
    value: String,
}
impl event_value {
    fn new(_name: String, _value: String) -> event_value {
        event_value {
            name: _name,
            value: _value,
        }
    }
}
fn hash_signature(sig: &str) -> Hash {
    let mut result = [0u8; 32];
    let data = sig.replace(" ", "").into_bytes();
    let mut sponge = Keccak::new_keccak256();
    sponge.update(&data);
    sponge.finalize(&mut result);
    Hash::from_slice(&result)
}

fn load_event(path: &str, name_or_signature: &str) -> Result<Event, &'static str> {
    let file = File::open(path);
    let contract = Contract::load(file.unwrap()).unwrap();
    let params_start = name_or_signature.find('(');

    match params_start {
        // It's a signature.
        Some(params_start) => {
            let name = &name_or_signature[..params_start];
            let signature = hash_signature(name_or_signature);
            contract
                .events_by_name(name)
                .unwrap()
                .iter()
                .find(|event| (*event).signature() == signature)
                .cloned()
                .ok_or("fail")
        }

        // It's a name.
        None => {
            let events = contract.events_by_name(name_or_signature).unwrap();
            match events.len() {
                0 => unreachable!(),
                1 => Ok(events[0].clone()),
                _ => Err("fail"),
            }
        }
    }
}
pub fn decode_params(
    types: Vec<&str>,
    data: &str,
) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    let types: Vec<ParamType> = types
        .iter()
        .map(|s| Reader::read(s))
        .collect::<Result<_, _>>()?;

    let data: Vec<u8> = data.from_hex()?;

    let tokens = decode(&types, &data)?;
    //println!("{:?}",&tokens);
    assert_eq!(types.len(), tokens.len());

    Ok(tokens)
}

pub fn decode_log(
    abi_path: &str,
    name_or_signature: &str,
    topics: Vec<String>,
    data: &str,
) -> Result<Vec<event_value>, Box<dyn std::error::Error>> {
    let event = load_event(&abi_path, name_or_signature)?;
    let topics: Vec<Hash> = topics
        .into_iter()
        .map(|t| t.parse())
        .collect::<Result<_, _>>()?;
    let data = data.from_hex()?;
    let decoded = event.parse_log((topics, data).into())?;

    let result = decoded
        .params
        .into_iter()
        .map(|log_param| {
            event_value::new(
                format!("{}", log_param.name),
                format!("{}", log_param.value),
            )
        })
        .collect::<Vec<event_value>>();

    Ok(result)
}
#[test]
fn log_decode() {
    //let command = "ethabi decode log ../res/event.abi Event -l 0000000000000000000000000000000000000000000000000000000000000001 0000000000000000000000004444444444444444444444444444444444444444".split(" ");
    let abi = "event.abi";
    let name = "Event";
    let topic = [String::from(
        "0000000000000000000000000000000000000000000000000000000000000001",
    )];
    let data = "0000000000000000000000004444444444444444444444444444444444444444";
    let expected = ["a true", "b 4444444444444444444444444444444444444444"];
    let decoded = decode_log(abi, name, topic.to_vec(), data).unwrap();
    assert_eq!(decoded, expected);
}
