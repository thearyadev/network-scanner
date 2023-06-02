use teloxide::prelude::*;
use std::process::Command;
use std::str;


#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Info) // Set log level to Info
        .init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::new("6266417972:AAGY4Qcq-1XoDZqofhtQ4qhRnOJdw8a-zmE");
    let chat_id = ChatId::from(UserId::from(teloxide::prelude::UserId(6283902754)));
    let result = bot.send_message(chat_id, "hello").send().await;
    match result {
        Ok(_) => log::info!("Message sent!"),
        Err(e) => log::error!("Error sending message: {}", e),
    }
    arp_scan();
}

fn arp_scan() -> Vec<(String, String)> {
    let output = Command::new("arp-scan")
        .args(&["-l", "-q"])
        .output()
        .expect("Failed to execute command");

    let stdout = str::from_utf8(&output.stdout).unwrap();

    stdout
        .lines()
        .skip(2)
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let ip = parts.next()?.to_string();
            let mac = parts.next()?.to_string();
            Some((ip, mac))
        })
        .collect()
}