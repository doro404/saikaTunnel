use crate::state::UiState;
use colored::*;

pub fn draw_main_menu(state: &UiState) {
    println!("\n=== Menu Principal ===");

    let tunnel_status = if state.saika_tunnel { "ON" } else { "OFF" };
    println!("1 - saikaTunnel Menu",);
    println!("q - Sair");

    println!("======================\n");
}

pub fn draw_saika_menu(state: &UiState, msg: &str) {
    println!("\n === Menu SaikaTunnel ===");

    let status = if state.saika_tunnel { "ON" } else { "OFF" };
    println!("Status: {}", status);
    println!("1 - Alternar ON/OFF");
    println!("2 - Ver Estatísticas");
    println!("b - Voltar");
    println!();

    if !msg.is_empty() {
        if msg.to_lowercase().contains("erro") {
            println!("{}", msg.red())
        } else {
            println!("{}", msg.green())
        }
    }

    println!("============================\n");
}

pub fn draw_saika_statisticas(state: &UiState) {
    println!("\n === Menu Estatisticas ===");

    let status = if state.estatisticas { "ON" } else { "OFF" };
    println!("Status: {}", status);
    println!("1 - Alternar ON/OFF");
    println!("b - Voltar");

    println!("============================\n");
}
