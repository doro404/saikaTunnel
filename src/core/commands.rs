use serde::Deserialize; // Isso precisa estar no arquivo onde você usa o #[derive(Deserialize)]

#[derive(Debug)]
pub enum TunnelCommand  {
    Start,
    Stop,
    Status,
    Message(String),
}

#[derive(Deserialize, Debug)]
pub struct SshPayload {
    pub user: String,
    pub password: String,
}