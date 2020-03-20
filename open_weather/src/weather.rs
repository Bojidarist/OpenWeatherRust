extern crate hyper;

use hyper::Client;
use hyper::body::HttpBody as _;

pub struct OpenWeatherApi{
    pub api_key: String
}

impl OpenWeatherApi {
    pub async fn get_json(&self, city : String) -> String {
        let client = Client::new();
        let url: String = String::from(format!("http://api.openweathermap.org/data/2.5/weather?q={}&APPID={}&units=metric", city, self.api_key));
        let parse_to_uri = url.parse().unwrap();
        let mut res = client.get(parse_to_uri).await.unwrap();

        return String::from(format!("{:?}", res.body_mut().data().await));
    }

    pub fn print_data(&self, value : serde_json::Value){
        println!("Name: {}", value["name"].to_string());
        println!("Country: {}", value["sys"]["country"].to_string());
        println!("Lon: {}", value["coord"]["lon"].to_string());
        println!("Lat: {}", value["coord"]["lat"].to_string());
        println!("Weather");
        println!("Main: {}", value["weather"][0]["main"].to_string().replace("\\", "").replace("\"", ""));
        println!("Description: {}", value["weather"][0]["description"].to_string().replace("\\", "").replace("\"", ""));
        println!("Temp: {}", value["main"]["temp"].to_string());
        println!("Feels like: {}", value["main"]["feels_like"].to_string());
        println!("Temp min: {}", value["main"]["temp_min"].to_string());
        println!("Temp max: {}", value["main"]["temp_max"].to_string());
        println!("Pressure: {}", value["main"]["pressure"].to_string());
        println!("Humidity: {}", value["main"]["humidity"].to_string());
        println!("Sea level: {}", value["main"]["sea_level"].to_string());
        println!("Ground level: {}", value["main"]["grnd_level"].to_string());
    }
}