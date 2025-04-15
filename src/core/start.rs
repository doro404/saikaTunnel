use std::process::{Command, Stdio};


pub fn toggle_tunnel(state_on: bool) -> String {
    if state_on {
        let _ = Command::new("bash")
            .arg("-c")
            .arg("nohup ./tunel-core &")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();

        return "Túnel iniciado!".to_string();
    } else {
        let output = Command::new("bash")
            .arg("-c")
            .arg("pkill tunel-core") // Verifique o nome do processo corretamente
            .output()
            .expect("Erro ao Parar o Tunnel!");

            if !output.status.success() {
                return "Erro ao tentar parar o túnel".to_string()
            } else {
                return "Túnel parado!".to_string()
            }
    }
}