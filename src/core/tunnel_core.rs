// Em src/core/tunnel_core.rs
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::io::{Read, Write};
use std::net::TcpListener;
use serde::{Deserialize};
use std::time::Duration;
use crate::core::commands::{TunnelCommand, SshPayload}; // Certifique-se de que TunnelCommand está sendo importado

// Modificar para receber Sender<TunnelCommand>
pub fn start_tunnel(tx: Sender<TunnelCommand>, rx: Receiver<TunnelCommand>, tx_message: Sender<String>) {
    // Loop para escutar comandos do main
    loop {
        match rx.recv() {
            Ok(cmd) => {
                match cmd {
                    TunnelCommand::Start => {
                        let _ = tx_message.send("Túnel recebendo comando START".to_string());
                        start_saika_tunnel();
                    }
                    TunnelCommand::Stop => {
                        let _ = tx_message.send("Túnel foi encerrado.".to_string());
                        break; // Encerra o túnel
                    }
                    TunnelCommand::Status => {
                        let _ = tx_message.send("Comando Status: Túnel ainda ativo".to_string());
                    }
                    _ => {
                        let _ = tx_message.send(format!("Comando desconhecido: {:?}", cmd));
                    }
                }
            }
            Err(_) => {
                let _ = tx_message.send("Canal de controle fechado.".to_string());
                break;
            }
        }
        thread::sleep(Duration::from_secs(1)); // Simula trabalho do túnel
    }
}

fn start_saika_tunnel() {
    start_tcp_server();
}

// Servidor TCP que aceita JSON com dados SSH
fn start_tcp_server() {
    let listener = TcpListener::bind("0.0.0.0:4000").expect("Erro ao escutar porta TCP");

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            std::thread::spawn(move || {
                let mut buffer = [0; 1024];

                let bytes_read = match stream.read(&mut buffer) {
                    Ok(0) => {
                        let _ = stream.write_all(b"conexao fechada pelo cliente\n");
                        return;
                    }
                    Ok(n) => n,
                    Err(e) => {
                        if let Err(write_error) = stream.write_all(format!("Erro ao ler dados: {}\n", e).as_bytes()) {
                            eprintln!("Erro ao escrever resposta no stream: {}", write_error);
                        }
                        return;
                    }
                };

                let payload_json = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Payload recebido: {}", payload_json);

                let payload: SshPayload = match serde_json::from_str(&payload_json) {
                    Ok(data) => data,
                    Err(_) => {
                        let _ = stream.write_all("Payload inválido\n".as_bytes());
                        return;
                    }
                };

                let response = match connect_to_ssh(&payload.host, &payload.user, &payload.password) {
                    Ok(_) => "Conectado com sucesso\n".to_string(),
                    Err(e) => format!("Erro ao conectar: {}\n", e),
                };

                let _ = stream.write_all(response.as_bytes());
            });
        }
    }
}

fn connect_to_ssh(host: &str, user: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::process::Command;

    let output = Command::new("sshpass")
       .args(&["-p", password, "ssh", "-o", "StrictHostKeyChecking=no", &format!("{}@{}", user, host), "echo conectado"])
       .output()?;

    if output.status.success() {
       Ok(())
    } else {
       Err("Falha ao conectar via SSH".into())
    }

}