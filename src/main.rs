mod _struct;

use structopt::StructOpt;
use _struct::Structure::*;

#[derive(StructOpt)] 
//use std::env;
struct Cli {
    city : String,
    country_code  : String
}

struct Forecast{
    coord : Coord,
    weather : Weather,
    base : String,
    visibility : i32,
    wind : Wind,
    clouds : Clouds,
    dt : i32,
    sys : Sys,
    timezone : i32,
    id : i32,
    name : String,
    cod : i32
}

fn main() {
    let args = Cli::from_args();
    //let args : Vec<String> = env::args().collect();
    //println!("Mes args : {:?}",args[1]);
    
    println!("Our city : {} , our country code : {}",args.city,args.country_code);
}
