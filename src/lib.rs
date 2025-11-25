use std::{collections::HashMap, error::Error, fmt, sync::OnceLock};

pub static TELEFYBOT: OnceLock<TelefyBot> = OnceLock::new();

#[macro_export]
macro_rules! message {
    ($message:expr) => {
        async {
            if let Some(telefy_bot) = $crate::TELEFYBOT.get() {
                let message = $message;
                if let Err(e) = telefy_bot.send_message(&message).await {
                    eprintln!("Couldn't send message: {}", e)
                } else {
                    println!("Sent message")
                }
            } else {
                println!("Bot not created")
            }
        }
        .await
    };
}

#[derive(Debug)]
pub enum TelefyError {
    NonSuccessStatus(u16),
    RequestError(reqwest::Error),
}

pub struct TelefyBot {
    token: String,
    chat_id: String,
}

impl std::fmt::Display for TelefyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonSuccessStatus(status) => write!(f, "Non success status: {}", status),
            Self::RequestError(err) => write!(f, "Request error: {}", err),
        }
    }
}

impl Error for TelefyError {}

impl TelefyBot {
    pub fn new(token: String, chat_id: String) -> Result<&'static TelefyBot, &'static str> {
        let telefy_bot = TelefyBot { token, chat_id };
        TELEFYBOT
            .set(telefy_bot)
            .map_err(|_| "TelefyBot already created")?;
        Ok(TELEFYBOT.get().unwrap())
    }
    pub async fn send_message(&self, message: &str) -> Result<(), TelefyError> {
        let client = reqwest::Client::new();
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.token);
        let mut map = HashMap::new();
        map.insert("chat_id", self.chat_id.clone());
        map.insert("text", message.to_string());
        let res = client
            .post(url)
            .json(&map)
            .send()
            .await
            .map_err(|e| TelefyError::RequestError(e))?;
        if res.status() != 200 {
            return Err(TelefyError::NonSuccessStatus(res.status().as_u16()));
        }
        Ok(())
    }
}
