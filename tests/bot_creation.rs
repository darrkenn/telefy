use telefy::TelefyBot;

#[test]
fn create_new_bot_successful() {
    let token = String::from("111111:aaaaaaaaaaaaaaaaaa");
    let chat_id = String::from("12312313");

    match TelefyBot::new(token, chat_id) {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    };
}

#[test]
fn creating_bot_twice_fails() {
    let token = String::from("111111:aaaaaaaaaaaaaaaaaa");
    let chat_id = String::from("12312313");

    match TelefyBot::new(token, chat_id) {
        Ok(_) => {
            panic!("Should have failed")
        }
        Err(e) => assert_eq!(e, "TelefyBot already created"),
    }
}
