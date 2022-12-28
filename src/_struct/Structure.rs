use serde_derive::{Deserialize,Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Coord{
    lon : f64,
    lat : f64
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Weather{
    details : Details
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Details {
    id : u32,
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
    humidity : i32
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Wind {
    speed : f64,
    deg : i32
} 
#[derive(Serialize,Deserialize,Debug)]
pub struct Clouds {
    all : i32
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Sys {
    r#type : i32,
    id : u32,
    country : String,
    sunrise : i32,
    sunset : i32
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Forecast{
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