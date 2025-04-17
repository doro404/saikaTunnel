use std::sync::{mpsc::{Sender, Receiver, channel}, Mutex};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::core::commands::TunnelCommand;
use crate::core::tunnel_core;
use std::sync::mpsc; // Ensure this is included
use once_cell::sync::Lazy;

static TX_CMD: Lazy<Mutex<Option<Sender<TunnelCommand>>>> = Lazy::new(|| Mutex::new(None));
static RX_MSG: Lazy<Mutex<Option<Receiver<String>>>> = Lazy::new(|| Mutex::new(None));
static IS_TUNNEL_RUNNING: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

pub fn toggle_tunnel(state_on: bool) -> String {
    // Se o túnel estiver em execução e a opção for para "desligar", enviar "Stop"
    if !state_on {
        if !IS_TUNNEL_RUNNING.load(Ordering::SeqCst) {
            return "Túnel já está parado.".to_string();
        }

        // Definir o comando de parada
        if let Some(tx) = &*TX_CMD.lock().unwrap() {
            if tx.send(TunnelCommand::Stop).is_ok() {
                if let Some(rx) = &*RX_MSG.lock().unwrap() {
                    if let Ok(msg) = rx.recv() {
                        IS_TUNNEL_RUNNING.store(false, Ordering::SeqCst); // Garantir que o estado do túnel seja alterado
                        return msg; // Retorna a mensagem do comando "Stop"
                    }
                }
            }
        }

        return "Erro ao parar túnel".to_string(); // Caso o envio do comando de parada falhe
    }

    // Se o túnel não estiver em execução e a opção for para "ligar", enviar "Start"
    if IS_TUNNEL_RUNNING.load(Ordering::SeqCst) {
        return "Túnel já está em execução.".to_string();
    }

    // Criar canais para comunicação do túnel
    let (tx_cmd, rx_cmd) = channel::<TunnelCommand>();
    let (tx_msg, rx_msg) = channel::<String>();

    {
        *TX_CMD.lock().unwrap() = Some(tx_cmd.clone());
        *RX_MSG.lock().unwrap() = Some(rx_msg);
    }

    IS_TUNNEL_RUNNING.store(true, Ordering::SeqCst);

    // Iniciar o túnel em uma thread separada
    thread::spawn(move || {
        tunnel_core::start_tunnel(tx_cmd.clone(), rx_cmd, &tx_msg);
    });

    // Enviar o comando de Start para o túnel
    if let Some(tx) = &*TX_CMD.lock().unwrap() {
        if tx.send(TunnelCommand::Start).is_ok() {
            if let Some(rx) = &*RX_MSG.lock().unwrap() {
                if let Ok(msg) = rx.recv() {
                    return msg; // Retorna a mensagem após o comando "Start"
                }
            }
        }
    }

    "Erro ao iniciar túnel".to_string() // Caso o envio do comando "Start" falhe
}

fn main() {
    println!("{}", toggle_tunnel(true));  // Inicia o túnel
    println!("{}", toggle_tunnel(false)); // Para o túnel
}
