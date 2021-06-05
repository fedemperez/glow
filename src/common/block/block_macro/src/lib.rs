extern crate proc_macro;
use serde_json::Value;
use proc_macro::{TokenTree, TokenStream};
use std::collections::HashMap;
use lazy_static::lazy_static;

const BLOCKS_JSON: &str = include_str!("blocks.json");

lazy_static! {
    static ref BLOCK_ID_MAP: HashMap<String, u16> = gen_block_id_map();
}

#[proc_macro]
pub fn block_id(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    if let TokenTree::Ident(ident) = tokens.next().expect("No identifier found :(") {
        let block_name = format!("minecraft:{}", ident.to_string());
        let id = BLOCK_ID_MAP.get(&block_name.to_string()).expect(
            format!("No block with id {} :(", block_name).as_str());
        format!("{}", id).parse().unwrap()
    } else {
        panic!("Block name should be an identifier (remove these if you used them \" \")");
    }
}

fn gen_block_id_map() -> HashMap<String, u16> {
    let mut result = HashMap::new();
    let json: HashMap<String, HashMap<String, Value>> = 
        serde_json::from_str(&BLOCKS_JSON).unwrap();
    for (name, block) in json {
        let states = block["states"].as_array().unwrap();
        for state in states {
            if let Some(Value::Bool(true)) = state.get("default") {
                let id = state["id"].as_u64().unwrap() as u16;
                result.insert(name, id);
                break;
            }
        }
    }
    result
}
