mod _struct;

use exitfailure::ExitFailure;
use structopt::StructOpt;
use _struct::Structure::*;

#[derive(StructOpt)] 
//use std::env;
struct Cli {
    city : String,
    country_code  : String
}



impl Forecast{
    async fn get(city : String , country_code : String) -> Result<Self,ExitFailure> {
            todo!()
    }   
}

fn main() {
    let args = Cli::from_args();
    //let args : Vec<String> = env::args().collect();
    //println!("Mes args : {:?}",args[1]);
    
    println!("Our city : {} , our country code : {}",args.city,args.country_code);
}
