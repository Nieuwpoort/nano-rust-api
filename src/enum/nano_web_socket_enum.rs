#[derive(Debug)]
pub enum NanoNodeWebSocketCommand {
    AddAccount(String),
    RemoveAccount(String),
}