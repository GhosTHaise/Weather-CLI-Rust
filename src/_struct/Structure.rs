use serde_derive::{Deserialize,Serialize};
#[derive(Serialize,Deserialize,Debug)]
pub struct Coord{
    lon : f64,
    lat : f64
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Weather{
    id : i32,
    main : String,
    description : String,
    icon : String
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Details {
    id : i32,
    main : String,
    description : String,
    icon : String
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Temps {
    temp : f64,
    feels_like : f64,
    temp_min : f64,
    temp_max : f64,
    pressure : i32,
    pub humidity : i32,
    sea_level : i32,
    grnd_level : i32
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Wind {
    speed : f64,
    deg : i32,
    gust : f64
} 
#[derive(Serialize,Deserialize,Debug)]
pub struct Clouds {
    all : i32
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Sys {
    //r#type : i32,
    //id : u32,
    country : String,
    sunrise : i32,
    sunset : i32
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Forecast{
    pub coord : Coord,
    pub weather : Vec<Weather>,
    pub base : String,
    pub main : Temps,
    pub visibility : i32,
    pub wind : Wind,
    pub clouds : Clouds,
    pub dt : i32,
    pub sys : Sys,
    pub timezone : i32,
    pub id : i32,
    pub name : String,
    pub cod : i32
}