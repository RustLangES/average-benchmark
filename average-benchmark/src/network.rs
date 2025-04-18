use chrono::DateTime;
use reqwest::header::HeaderMap;
use serde_json::Value;

pub async fn send_data(system_info: Value) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let request = client
        .request(
            reqwest::Method::POST,
            concat!(env!("BACKEND_URL"), "/submit-tests"),
        )
        .headers(headers)
        .json(&system_info);

    let response = request.send().await?;
    let body = response.text().await?;
    let parsed: Value = serde_json::from_str(&body)?;

    let success = parsed["success"].as_bool() == Some(true);
    let message_type = if success { "Message" } else { "Error" };
    let color_code = if success { "32" } else { "31" };
    let content = if success {
        parsed["message"].as_str().unwrap_or("Success")
    } else {
        parsed["error"].as_str().unwrap_or("Unknown error")
    };

    let timestamp = parsed["timestamp"]
        .as_str()
        .and_then(|ts_str| DateTime::parse_from_rfc3339(ts_str).ok())
        .map(|ts| ts.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .unwrap_or_else(|| "N/A".to_string());

    println!("\x1B[{}m{}: {}\x1B[0m", color_code, message_type, content);
    println!("\x1B[{}mTimestamp: {}\x1B[0m", color_code, timestamp);

    Ok(())
}
