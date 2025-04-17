// Em src/core/tunnel_core.rs
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::fs::OpenOptions;
use serde::{Deserialize};
use std::time::Duration;
use std::process::Command;
use crate::core::commands::{TunnelCommand, SshPayload}; // Certifique-se de que TunnelCommand está sendo importado
use crate::core::global::*;

fn log_message(msg: &str) {
    let mut file = OpenOptions::new()
        .append(true) // Adicionar ao arquivo existente
        .create(true) // Criar o arquivo se não existir
        .open("server_logs.txt") // Caminho do arquivo
        .unwrap();

    writeln!(file, "{}", msg).unwrap();
}


// Modificar para receber Sender<TunnelCommand>
pub fn start_tunnel(tx: Sender<TunnelCommand>, rx: Receiver<TunnelCommand>, tx_message: &Sender<String>) {
    let tcp_server_running = Arc::new(AtomicBool::new(false));

    loop {
        match rx.recv() {
            Ok(cmd) => match cmd {
                TunnelCommand::Stop => {
                    if tcp_server_running.load(Ordering::SeqCst) {
                        tcp_server_running.store(false, Ordering::SeqCst);
                        let _ = tx_message.send("Túnel foi encerrado.".to_string());
                    } else {
                        let _ = tx_message.send("Túnel já está parado.".to_string());
                    }
                }
                TunnelCommand::Status => {
                    let status = if tcp_server_running.load(Ordering::SeqCst) {
                        "Túnel ainda ativo"
                    } else {
                        "Túnel não está em execução"
                    };
                    let _ = tx_message.send(format!("Comando Status: {}", status));
                }
                TunnelCommand::Start => {
                    if !tcp_server_running.load(Ordering::SeqCst) {
                        tcp_server_running.store(true, Ordering::SeqCst);
                        let tx_message_inner = tx_message.clone();
                        let tcp_flag = tcp_server_running.clone();

                        std::thread::spawn(move || {
                            start_tcp_server(tx_message_inner, tcp_flag);
                        });

                        let _ = tx_message.send(format!("Servidor TCP iniciado na porta {}", SERVER_SAIKA_PORT));
                    } else {
                        let _ = tx_message.send("Servidor já está em execução.".to_string());
                    }
                }
                _ => {
                    let _ = tx_message.send(format!("Comando desconhecido: {:?}", cmd));
                }
            },
            Err(_) => {
                let _ = tx_message.send("Canal de controle fechado.".to_string());
                break;
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
}



// Servidor TCP que aceita JSON com dados SSH
pub fn start_tcp_server(tx_message: Sender<String>, server_running: Arc<AtomicBool>) {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", SERVER_SAIKA_PORT))
    .expect("Erro ao escutar porta TCP");

    listener.set_nonblocking(true).expect("Falha ao tornar listener não-bloqueante");

    while server_running.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((mut stream, _addr)) => {
                std::thread::spawn(move || {
                    let mut buffer = [0; 1024];
                    let bytes_read = match stream.read(&mut buffer) {
                        Ok(0) => return,
                        Ok(n) => n,
                        Err(_) => return,
                    };

                    let payload_json = String::from_utf8_lossy(&buffer[..bytes_read]);
                    log_message(&format!("Payload recebido: {}", payload_json));

                    let payload: SshPayload = match serde_json::from_str(&payload_json) {
                        Ok(data) => data,
                        Err(_) => {
                            let _ = stream.write_all("Payload inválido\n".as_bytes());
                            return;
                        }
                    };

                    let response = match authenticate_locally(&payload.user, &payload.password) {
                        Ok(_) => "Autenticação bem-sucedida\n".to_string(),
                        Err(e) => format!("Erro na autenticação: {}\n", e),
                    };

                    log_message(&format!("Resposta enviada ao cliente: {}", response));
                    let _ = stream.write_all(response.as_bytes());
                });
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(100));
                continue;
            }
            Err(e) => {
                log_message(&format!("Erro ao aceitar conexão: {}", e));
                break;
            }
        }
    }

    let _ = tx_message.send("Servidor TCP encerrado.".to_string());
    server_running.store(false, Ordering::SeqCst); // Libera para um próximo Start
}

fn authenticate_locally(user: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("sshpass")
        .args(&["-p", password, "ssh", "-o", "StrictHostKeyChecking=no", &format!("{}@localhost", user), "echo autenticado"])
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err("Falha na autenticação local".into())
    }
}