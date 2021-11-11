use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

mod stat_data;
use stat_data::Stats;

#[wasm_bindgen]
pub async fn get_schedule(date: String) -> Result<JsValue, JsValue> {
    // Request options
    let mut options = RequestInit::new();
    options.method("GET");
    options.mode(RequestMode::Cors);

    // URL
    let url = format!("https://statsapi.web.nhl.com/api/v1/schedule?date={}", date);

    // Setup and send request
    let request = Request::new_with_str_and_init(&url, &options)?;
    let window = web_sys::window().unwrap();
    let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // Handle response
    let response: Response = response_value.dyn_into().unwrap();
    let json = JsFuture::from(response.json()?).await?;
    // Return javascript object as JsValue
    Ok(json)
}

#[wasm_bindgen]
pub async fn get_stats(team: String) -> Result<JsValue, JsValue> {
    let mut options = RequestInit::new();
    options.method("GET");
    options.mode(RequestMode::Cors);

    // Setup request
    let url = format!("https://statsapi.web.nhl.com/api/v1/teams/{}/stats", team);
    let request = Request::new_with_str_and_init(&url, &options)?;
    // Make request
    let window = web_sys::window().unwrap();
    let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    // Handle response
    let response: Response = response_value.dyn_into().unwrap();
    let json = JsFuture::from(response.json()?).await?;
    //Return js object
    Ok(json)
}

#[wasm_bindgen]
pub fn get_probability(home: String, away: String) -> Result<JsValue, JsValue> {
    // Deserialize json and perform probability calculation
    let home_stats: Stats = match serde_json::from_str(&home) {
        Ok(data) => data,
        Err(e) => {
            error(&format!("Error parsing json for home team: {}", e));
            return Err(JsValue::FALSE);
        }
    };
    let away_stats: Stats = match serde_json::from_str(&away) {
        Ok(data) => data,
        Err(e) => {
            error(&format!("Error parsing json for away team: {}", e));
            return Err(JsValue::FALSE);
        }
    };

    let mut home_score = 0.0;
    let mut away_score = 0.0;

    // Calculate home team score
    let home_ppp: f64 = match home_stats.power_play_percentage.parse() {
        Ok(number) => number,
        Err(e) => {
            error(&format!("Error parsing home team PP%: {}", e));
            return Err(JsValue::FALSE);
        }
    };
    let home_pkp: f64 = match home_stats.penalty_kill_percentage.parse() {
        Ok(number) => number,
        Err(e) => {
            error(&format!("Error parsing home team PK%: {}", e));
            return Err(JsValue::FALSE);
        }
    };
    let home_fop: f64 = match home_stats.face_off_win_percentage.parse() {
        Ok(number) => number,
        Err(e) => {
            error(&format!("Error parsing home team FO%: {}", e));
            return Err(JsValue::FALSE);
        }
    };
    home_score += home_fop;
    home_score += home_pkp;
    home_score += home_ppp;
    home_score += home_stats.save_pctg;
    home_score += home_stats.shooting_pctg;
    home_score += 21.0 * (home_stats.goals_per_game - home_stats.goals_against_per_game);

    // Calculate away team score
    let away_ppp: f64 = match away_stats.power_play_percentage.parse() {
        Ok(number) => number,
        Err(e) => {
            error(&format!("Error parsing away team PP%: {}", e));
            return Err(JsValue::FALSE);
        }
    };
    let away_pkp: f64 = match away_stats.penalty_kill_percentage.parse() {
        Ok(number) => number,
        Err(e) => {
            error(&format!("Error parsing away team PK%: {}", e));
            return Err(JsValue::FALSE);
        }
    };
    let away_fop: f64 = match away_stats.face_off_win_percentage.parse() {
        Ok(number) => number,
        Err(e) => {
            error(&format!("Error parsing away team FO%: {}", e));
            return Err(JsValue::FALSE);
        }
    };

    away_score += away_fop;
    away_score += away_pkp;
    away_score += away_ppp;
    away_score += away_stats.shooting_pctg;
    away_score += away_stats.save_pctg;
    away_score += 21.0 * (away_stats.goals_per_game - away_stats.goals_against_per_game);

    // Final calculations
    let mut total_score = home_score + away_score;
    let home_advantage = total_score * 0.044;
    total_score += home_advantage;
    home_score += home_advantage;

    let home_prob = format!("{:.2}%", (home_score / total_score) * 100.0);
    let away_prob = format!("{:.2}%", (away_score / total_score) * 100.0);

    let prob_result = &format!(
        "{{ \"home\": \"{}\", \"away\": \"{}\" }}",
        home_prob, away_prob
    );
    Ok(JsValue::from_str(prob_result))
}

// Console.log in JS
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// console.error in JS
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(e: &str);
}
