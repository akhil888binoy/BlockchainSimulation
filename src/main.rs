use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest};
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use std::collections::LinkedList;
use chrono::prelude::*;


#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
struct Transaction {
    sender: String ,
    receiver : String ,
    amount : u32 ,
    timestamp : String ,
    signature : String ,
    transactionId : String 
}

impl Transaction{
    fn new()->Self{
        Transaction{
            sender : String::new(),
            receiver: String::new(),
            amount : 0,
            timestamp : String::new(),
            signature : String::new(),
            transactionId: String::new()
        }   
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
struct Block{
    index : u32,
    timestamp: String,
    listOftransactions : Vec<Transaction>,
    hash : String ,
    prevhash: String ,
    nonce : u32
}

impl Block{
    fn new(indexCount : u32 , utc : String)->Self{
        Block { 
            index: indexCount
            , timestamp: utc
            , listOftransactions: Vec::<Transaction>::new()
            , hash: String::new(), 
              prevhash: String::new()
            , nonce: 0 
        }
    }
}

fn main() {
    let mut indexCount : u32 = 0 ;
    let mut list: LinkedList<Block> = LinkedList::new();
    let mut utc: DateTime<Utc> = Utc::now();

    loop{
            let mut block = Block::new(indexCount , utc.to_string());
            list.push_back(block.clone());
            let serialized = serde_json::to_string(&block).unwrap();
            let mut hasher = Sha256::new();
            hasher.update(serialized.as_bytes());
            indexCount += 1 ;
            let result = hasher.finalize();
            let hash_hex = format!("{:x}", result);
            println!("Block hash: {}", hash_hex);
            println!(" Lissssttttt : {:?}", list);
    }
    
}
