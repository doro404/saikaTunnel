pub mod layout;
use layout::{draw_main_menu, draw_saika_menu, draw_saika_statisticas};
use crate::state::{UiState, Menu}; // ajuste de acordo com onde UiState e Menu estão definidos
use std::io::{self, Write};
use ctrlc;
use std::io::stdout;
use std::process;
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};
use crate::core::start::toggle_tunnel;


pub fn run_ui() {
    let mut option_selected: Option<u8> = None;

    let mut state = UiState {
        saika_tunnel: false,
        estatisticas: false,
        current_menu: Menu::Principal,
        tunnel_message: "".to_string(),
    };

    ctrlc::set_handler(move || {
        process::exit(0);
    }).expect("Erro ao configurar Ctrl+C");

    loop {
        clear_terminal();

        let tunnel_message = &state.tunnel_message;

        match state.current_menu {
            Menu::Principal => draw_main_menu(&state),
            Menu::SaikaTunnel => draw_saika_menu(&state, &tunnel_message),
            Menu::Estatisticas => draw_saika_statisticas(&state),
        }

        let mut input = String::new();
        println!("Escolha uma opção: ");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        
        let input = input.trim();

        match state.current_menu {
            Menu::Principal => match input {
                "1" => state.current_menu = Menu::SaikaTunnel,
                "q" => break,
                _ => {}
            },
            Menu::SaikaTunnel => match input {
                "1" => {
                    state.saika_tunnel = !state.saika_tunnel;
                    // Aqui, alterne o túnel e capture a mensagem retornada
                    state.tunnel_message = toggle_tunnel(state.saika_tunnel);
                }
                
                "2" => {
                    state.current_menu = Menu::Estatisticas;
                }
                "b" => state.current_menu = Menu::Principal,
                _ => {}
            },
            Menu::Estatisticas => match input {
                "1" => state.estatisticas = !state.estatisticas,
                "b" => state.current_menu = Menu::SaikaTunnel,
                _ => {}

            }
        }

        if let Some(ref option) = option_selected {
            println!("Você selecionou: {}", option);
        }
    }
}

fn clear_terminal() {
    let mut stdout = stdout();
    execute!(
        stdout,
        Clear(ClearType::All),
        MoveTo(0, 0)
    ).unwrap();
}