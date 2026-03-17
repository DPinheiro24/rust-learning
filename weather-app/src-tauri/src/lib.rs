use serde::{Deserialize, Serialize};
use std::{fs};

#[derive(Serialize, Deserialize)]
struct Preferences {
    city: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CurrentWeather {
    time: String,
    temperature_2m: f32,
    is_day: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct DailyWeather {
    time: Vec<String>,
    temperature_2m_max: Vec<f32>,
    temperature_2m_min: Vec<f32>,
    sunrise: Vec<String>,
    sunset: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct DayForecast {
    time: String,
    temperature_2m_max: f32,
    temperature_2m_min: f32,
    sunrise: String,
    sunset: String
}

#[derive(Serialize, Deserialize, Debug)]
struct DataWeather {
    current: CurrentWeather,
    daily: DailyWeather
}

#[derive(Serialize, Deserialize)]
struct WeatherReply {
    is_day: i32,
    forecast: Vec<DayForecast>,
}

#[derive(Serialize, Deserialize)]
struct Geocoding {
    results: Option<Vec<CityResponse>>
}

#[derive(Serialize, Deserialize)]
struct CityResponse {
    name: String,
    latitude: f32,
    longitude: f32,
    country: Option<String>,
    admin1: Option<String>
}

#[tauri::command]
async fn fetch_place(place: String) -> Result<WeatherReply, String> {
    let url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1",
        place
    );

    let response = reqwest::get(url)
        .await
        .map_err(|e|e.to_string())?;

    let reply: Geocoding = response.json().await.map_err(|e|format!("{e:?}"))?;

    let city = reply.results
        .and_then(|r| r.into_iter().next())
        .ok_or_else(|| format!("City '{}' not found", place))?;

    let weather = fetch_weather(city.latitude.to_string(), city.longitude.to_string()).await?;

  Ok(weather)

}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn fetch_weather(lat: String, longi: String) -> Result<WeatherReply, String> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&daily=weather_code,temperature_2m_max,temperature_2m_min,sunrise,sunset&current=temperature_2m,weathercode,windspeed_10m,relative_humidity_2m,is_day",
        lat, longi
    );

    let response = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?;

    let text: DataWeather = response.json().await.map_err(|e| format!("{e:?}"))?;

    let mut reply = Vec::new();

    for (i, date) in text.daily.time.iter().enumerate() {
        reply.push(DayForecast {
            time: date.clone(),
            temperature_2m_max: text.daily.temperature_2m_max[i],
            temperature_2m_min: text.daily.temperature_2m_min[i],
            sunrise: text.daily.sunrise[i].clone(),
            sunset: text.daily.sunset[i].clone()
        });
    }

    Ok(WeatherReply {
        is_day: text.current.is_day,
        forecast: reply,
    })
}

#[tauri::command]
fn save_preferences(prefs: Preferences) -> Result<(), String>{
    let json = serde_json::to_string(&prefs).unwrap();
    fs::write("preferences.json", json)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn load_preferences() -> Preferences {
    match fs::read_to_string("preferences.json") {
        Ok(json) => serde_json::from_str(&json).unwrap_or(Preferences { city: None }),
        Err(_) => Preferences { city: None },
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            fetch_weather, 
            fetch_place, 
            save_preferences, 
            load_preferences
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
