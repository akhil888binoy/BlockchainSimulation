use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest};
use std::{hash::{Hash, Hasher}, io::{stdin, Read}, sync::mpsc::Receiver};
use serde::{Serialize, Deserialize};
use std::collections::LinkedList;
use chrono::prelude::*;
use std::hash::{DefaultHasher};


#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
struct Transaction {
    sender: String ,
    receiver : String ,
    amount : u32 ,
    timestamp : String ,
    signature : String ,
}

impl Transaction{
    fn new(sender : String , receiver : String , amount : u32 , timestamp : String, signature: String)->Self{
        Transaction{
           sender,
            receiver,
            amount ,
            timestamp,
            signature ,
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
    fn new(indexCount : u32 , utc : String ,transaction:Transaction , hash :String , newprevhash:String)->Self{
        let mut listOfTxn = Vec::<Transaction>::new();
        listOfTxn.push(transaction);
        Block { 
            index: indexCount
            , timestamp: utc
            , listOftransactions: listOfTxn
            , hash, 
              prevhash: newprevhash
            , nonce: 0 
        }
    }
}

fn getinput(p:&str)->String{
    println!("{}", p);
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn getNuminput(p:&str)-> u32{
    println!("{}", p);
    let mut input = String::new();
    stdin().read_line(&mut input).expect("cannot readline");
    let numinput : u32 = input.trim().parse().unwrap();
    return numinput;
}


fn initiateTxn(timestamp : String ,signature : String )->Transaction{
    println!("Enter the transaction details");
    let mut sender = getinput("Enter sender");
    let mut receiver = getinput("Enter receiver");
    let mut amount =  getNuminput("Enter the amount");
    let transaction = Transaction::new(sender , receiver , amount , timestamp , signature);
    return transaction;

}


fn getHash()->String{
   let mut hasher = DefaultHasher::new();
   7920.hash(&mut hasher);
   let hash = hasher.finish();
   hash.to_string()
}


fn main() {
    let mut indexCount : u32 = 0 ;
    let mut list: LinkedList<Block> = LinkedList::new();
    let mut prevhash = String::from("0x000000000000000000000000000");
    loop{    
            let mut utc: DateTime<Utc> = Utc::now();
            let hash = getHash();
            let sign = getHash();
            let transaction = initiateTxn(utc.to_string() , sign);
            let mut block = Block::new(indexCount , utc.to_string(), transaction , hash , prevhash);
            list.push_back(block.clone());
            let serialized = serde_json::to_string(&block).unwrap();
            let mut hasher = Sha256::new();
            hasher.update(serialized.as_bytes());


            indexCount += 1 ;
            let result = hasher.finalize();
            let hash_hex = format!("{:x}", result);
            let clonedhex = hash_hex.clone();
            prevhash = hash_hex;
            println!("Block hash: {}", clonedhex);
            println!(" Lissssttttt : {:?}", list);
    }
    
}
