
pub struct Coord{
    lon : f64,
    lat : f64
}

pub struct Weather{
    details : Details
}

pub struct Details {
    id : u32,
    main : String,
    description : String,
    icon : String
}

pub struct Temps {
    temp : f64,
    feels_like : f64,
    temp_min : f64,
    temp_max : f64,
    pressure : i32,
    humidity : i32
}
pub struct Wind {
    speed : f64,
    deg : i32
} 

pub struct Clouds {
    all : i32
}

pub struct Sys {
    r#type : i32,
    id : u32,
    country : String,
    sunrise : i32,
    sunset : i32
}