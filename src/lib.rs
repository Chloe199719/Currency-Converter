#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


use std::{collections::HashMap, env};
use serde::{Deserialize, Serialize};
use reqwest::{self, header::HeaderName};
use dotenv::dotenv;
#[derive(Debug, Deserialize)]
struct Meta {
    last_updated_at: String,
}

#[derive(Debug, Deserialize)]
struct Currency {
    code: String,
    value: f64,
}

#[derive(Debug, Deserialize)]
struct Response {
    meta: Meta,
    data: HashMap<String, Currency>,
}




// Bad implementation of api call client can get Key from api call
pub fn get_currency (input_currency: &str , output_currency: &str , value:f64) -> Option<f64> {
    dotenv().ok();
    let secret = env::var("API_KEY").expect("API_KEY not set");
    let client = reqwest::blocking::Client::new();
    let  req = client.get("https://api.currencyapi.com/v3/latest ").header("apikey",secret).query(&[("base_currency", input_currency), ("currencies", output_currency)]).build().unwrap();
    let req = client.execute(req);
    
    let  res: Result<Response, reqwest::Error> = match req {
        Ok(n) => {
            n.json()},
        Err(_) => {
            println!("Error in getting data from api 1");
            return None;
        }
    
        
    };
    let data = match res {
        Ok(n) => n,
        Err(_) => {
            println!("Error in getting data from api 2");
            return None;
        }
    };
    // println!("{:?}", data);
    return  Some(value * data.data.get(output_currency).unwrap().value);
 
}