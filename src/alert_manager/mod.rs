use dotenv::dotenv;
use reqwest::{blocking::Client, Error, StatusCode};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct SMSResponse {
    account_sid: Option<String>,
    api_version: String,
    body: String,
    date_created: String,
    date_sent: String,
    date_updated: String,
    direction: String,
    error_code: String,
    error_message: String,
    from: String,
    messaging_service_sid: String,
    num_media: String,
    num_segments: String,
    price: String,
    price_unit: String,
    sid: String,
    status: String,
    subresource_uris: SubresourceUris,
    to: String,
    uri: String,
}

#[derive(Serialize, Deserialize)]
struct SubresourceUris {
    all_time: String,
    today: String,
    yesterday: String,
    this_month: String,
    last_month: String,
    daily: String,
    monthly: String,
    yearly: String,
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    code: u16,
    message: String,
    more_info: String,
    status: u16
}

#[derive(Deserialize, Debug)]
pub struct TwilioMessage {
    sid: String,
    body: String,
    from: String,
    to: String,
    date_sent: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TwilioResponse {
    messages: Vec<TwilioMessage>,
}

pub struct AlertManager {
    twilio_account_sid: String,
    twilio_auth_token: String,
    twilio_phone_number: String,
    recipient_phone_number: String,
}

impl AlertManager {
    pub fn new() -> Self {
        dotenv().ok();

        let twilio_account_sid =
            env::var("TWILIO_ACCOUNT_SID").expect("Twilio Account SID could not be retrieved.");
        let twilio_auth_token =
            env::var("TWILIO_AUTH_TOKEN").expect("Twilio Auth Token could not be retrieved.");
        let twilio_phone_number =
            env::var("TWILIO_PHONE_NUMBER").expect("The Twilio phone number could not be retrieved.");
        let recipient_phone_number = env::var("RECIPIENT_PHONE_NUMBER")
            .expect("The recipient's phone number could not be retrieved.");

        Self {
            twilio_account_sid,
            twilio_auth_token,
            twilio_phone_number,
            recipient_phone_number,
        }
    }

    fn handle_error(&self, body: String) {
        let error_response: ErrorResponse = serde_json::from_str(&body).expect("Unable to deserialise JSON error response.");
        println!("SMS was not able to be sent because: {:?}.", error_response.message);
    }

    fn handle_success(&self, body: String) {
        let sms_response: SMSResponse = serde_json::from_str(&body).expect("Unable to deserialise JSON success response.");
        println!("Your SMS with the body \"{:?}\".", sms_response.body);
    }

    pub fn send_alert(&self, message: &str) -> Result<(), Error> {
        let request_url = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            self.twilio_account_sid
        );
    
        let client = Client::new();
        let request_params = [
            ("To", &self.recipient_phone_number),
            ("From", &self.twilio_phone_number),
            ("Body", &message.to_string()),
        ];
        let response = client
            .post(&request_url)
            .basic_auth(&self.twilio_account_sid, Some(&self.twilio_auth_token))
            .form(&request_params)
            .send()?;

        let status = response.status();
        let body = match response.text() {
            Ok(result) => result,
            Err(error) => panic!(
                "Problem extracting the JSON body content. Reason: {:?}",
                error
            ),
        };

        match status {
            StatusCode::BAD_REQUEST => self.handle_error(body),
            StatusCode::OK => self.handle_success(body),
            _ => println!("Received status code: {}", status),
        }

        Ok(())
    }

    pub fn fetch_twilio_messages(&self) -> Result<(), reqwest::Error> {
        let url = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            self.twilio_account_sid
        );

        let client = Client::new();
        let response: TwilioResponse = client
            .get(&url)
            .basic_auth(&self.twilio_account_sid, Some(&self.twilio_auth_token))
            .send()?
            .json()?;

        println!("Received messages:");
        for message in response.messages {
            println!(
                "From: {}, To: {}, Date: {:?}, Body: {}",
                message.from, message.to, message.date_sent, message.body
            );
        }

        Ok(())
    }


}
