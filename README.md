# telefy
A library for sending logging messages to telegram.


## Example
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "TOKEN".to_string();
    let chat_id = "CHAT_ID".to_string();

    //Create bot
    TelefyBot::new(token, chat_id)?;

    //Create and send message
    let message = "Hello I am a bot".to_string();
    telefy::message!(message);
}
```
