#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


use std::env;

use cur_converter::{ get_currency};

 fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: currency_converter <original currency> <target currency> <value>");
        return;
    }
    let original = &args[1];
    let target = &args[2];
    let value = &args[3];
    let clean_original =  match original.to_lowercase().as_str() {
        "usd" => "USD",
        "eur" => "EUR",
        "gbp" => "GBP",
        "jpy" => "JPY",
        "cad" => "CAD",
        "aud" => "AUD",
        _ => {
            println!("{} is not a recognized currency code", original);
            println!("Please use one of the following: USD, EUR, GBP, JPY, CAD, AUD");
            return;
        }
    
    };
    let clean_target =  match target.to_lowercase().as_str() {
        "usd" => "USD",
        "eur" => "EUR",
        "gbp" => "GBP",
        "jpy" => "JPY",
        "cad" => "CAD",
        "aud" => "AUD",
        _ => {
            println!("{} is not a recognized currency code", target);
            println!("Please use one of the following: USD, EUR, GBP, JPY, CAD, AUD");
            return;
        }
    
    };
    let clean_value = match value.parse::<f64>() {
        Ok(n) => n,
        Err(_) => {
            println!("{} is not a valid number", value);
            return;
        }
    };
    if clean_value <= 0.0 {
        println!("{} is not a valid number", value);
        return;
    }
    if clean_original == clean_target {
        println!("{} {} = {} {}", clean_value, clean_original, clean_value, clean_target);
        return;
    }



    let i = get_currency(clean_original, clean_target, clean_value);
    match i {
        Some(n) => println!("{} {} = {} {}", clean_value, clean_original, ((n * 100.0).round()) / 100.0, clean_target),
        None => println!("Error in getting data from api"),
    }
    // println!("Input Currency: {} , Output Currency: {} , Value to Convert : {}", clean_original, clean_target, clean_value);
}
