// src/state.rs

#[derive(Debug)]
pub enum Menu {
    Principal,
    SaikaTunnel,
    Estatisticas,
}

#[derive(Debug)]
pub struct UiState {
    pub saika_tunnel: bool,
    pub estatisticas: bool,
    pub current_menu: Menu,
    pub tunnel_message: String,
}
