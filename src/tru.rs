use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_connect(mut client: TcpStream, request_line: String) {
    if let Some(dest) = request_line.split_whitespace().nth(1) {
        println!("[HTTPS] Conectando a {}", dest);

        // Tenta conectar ao destino (ex: google.com:443)
        if let Ok(mut server) = TcpStream::connect(dest) {
            // Envia ao cliente uma resposta de sucesso
            let _ = client.write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n");

            let mut client_clone = client.try_clone().unwrap();
            let mut server_clone = server.try_clone().unwrap();

            // Encaminha dados client -> server
            let t1 = thread::spawn(move || {
                let mut buf = [0; 1024];
                while let Ok(n) = client.read(&mut buf) {
                    if n == 0 || server.write_all(&buf[..n]).is_err() {
                        break;
                    }
                }
                let _ = server.shutdown(Shutdown::Write);
            });

            // Encaminha dados server -> client
            let t2 = thread::spawn(move || {
                let mut buf = [0; 1024];
                while let Ok(n) = server_clone.read(&mut buf) {
                    if n == 0 || client_clone.write_all(&buf[..n]).is_err() {
                        break;
                    }
                }
                let _ = client_clone.shutdown(Shutdown::Write);
            });

            let _ = t1.join();
            let _ = t2.join();
        } else {
            let _ = client.write_all(b"HTTP/1.1 502 Bad Gateway\r\n\r\n");
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if let Ok(n) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..n]);

        if request.starts_with("CONNECT") {
            let line = request.lines().next().unwrap_or_default().to_string();
            handle_connect(stream, line);
        } else {
            println!("[HTTP] {}", request.lines().next().unwrap_or_default());

            // Aqui você pode fazer proxy HTTP (GET/POST), se quiser.
            // Só para testar:
            let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nProxy HTTP OK!";
            let _ = stream.write_all(response);
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").expect("Não conseguiu escutar na porta");

    println!("Proxy rodando em 0.0.0.0:7878");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(move || handle_client(stream));
        }
    }
}
