mod _struct;

use exitfailure::ExitFailure;
use structopt::StructOpt;
use _struct::Structure::*;
use reqwest::Url;
use tokio;

#[derive(StructOpt)] 
//use std::env;
struct Cli {
    city : String,
    country_code  : String
}



impl Forecast{
    async fn get(city : &String , country_code : &String) -> Result<Self,ExitFailure> {
        let url = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&appid=0cd2598c06685a9e3497c04dccea7395",city,country_code);
        let url = Url::parse(&*url)?;

        let response = reqwest::get(url)
                        .await?
                        .json::<Forecast>()
                        .await?
                        ;
        
        Ok(response)
    }   
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();
    //let args : Vec<String> = env::args().collect();
    //println!("Mes args : {:?}",args[1]);
    let response = Forecast::get(&args.city,&args.country_code)
        .await.unwrap();
    println!("Our city : {} , our country code : {}, humidity : {}%",
                args.city,args.country_code,response.main.humidity);
}
