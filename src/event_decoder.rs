//use crate::error::Error;
use ethabi::param_type::{ParamType, Reader};
use ethabi::token::Token;
use ethabi::{decode, Contract, Event, Hash};
use rustc_hex::FromHex;
use serde::{Deserialize, Serialize};
use std::fs::File;
//use std::fs::File;
use tiny_keccak::Keccak;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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
    fn new_from_str(_name: &str, _value: &str) -> event_value {
        event_value {
            name: String::from(_name),
            value: String::from(_value),
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
fn Incoming_log_decode() {
    let abi = "./eventsABI/Incoming.abi";
    let name = "Incoming";
    let topic = [
        String::from("69741cc3ec0270f258feb6b53b42ef1e7d2251a3c8eea4f6ba1f72bd4b7beba7"),
        String::from("dd01e0d1e313c493bd8dcb841088d6d6bcbca3b0c3cfe6d0c76df566f0b2577d"),
        String::from("000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266"),
        String::from("0000000000000000000000009a9f2ccfde556a7e9ff0848998aa4a0cfd8863ae"),
    ];
    let data = "00000000000000000000000000000000000000000000000000000000000000800da72197e898ebe1814471a76048ed137a089f595c1e7038f2b70e98645e765200000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000571756572790000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002273c58c66704975459b45d17d8d262d7b6de59f5e57291ea5af890e2cf11a2b808455b9a244e3f6e53a4d620c3d1261bb351f203ccdf11324cfed2696b108913";
    let expected = [event_value::new(String::from("id"),String::from("dd01e0d1e313c493bd8dcb841088d6d6bcbca3b0c3cfe6d0c76df566f0b2577d")),event_value::new_from_str("provider","f39fd6e51aad88f6f4ce6ab8827279cfffb92266"),event_value::new_from_str("subscriber","9a9f2ccfde556a7e9ff0848998aa4a0cfd8863ae"),event_value::new_from_str("query","query"),event_value::new_from_str("endpoint","0da72197e898ebe1814471a76048ed137a089f595c1e7038f2b70e98645e7652" ),event_value::new_from_str("endpointParams","[273c58c66704975459b45d17d8d262d7b6de59f5e57291ea5af890e2cf11a2b8,08455b9a244e3f6e53a4d620c3d1261bb351f203ccdf11324cfed2696b108913]")];

    let decoded = decode_log(abi, name, topic.to_vec(), data).unwrap();
    println!("{:?}", decoded);
    assert_eq!(decoded[0], expected[0]);
    assert_eq!(decoded[1], expected[1]);
    assert_eq!(decoded[2], expected[2]);
    assert_eq!(decoded[3], expected[3]);
    assert_eq!(decoded[4], expected[4]);
}
