use std::time::Duration;

use clap::{arg, command};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = command!()
        .arg(
            arg!(--ip <VALUE>)
                .required(false)
                .default_value("192.168.1.254"),
        )
        .arg(arg!(--port <VALUE>).required(false).default_value("23"))
        .arg(
            arg!(--username <VALUE>)
                .required(false)
                .default_value("admin"),
        )
        .arg(
            arg!(--password <VALUE>)
                .required(false)
                .default_value("admin"),
        )
        .get_matches();

    let ip = matches.value_of("ip").unwrap();
    let port = matches.value_of("port").unwrap();
    let username = matches.value_of("username").unwrap();
    let password = matches.value_of("password").unwrap();

    let diagnostics = get_diagnostics(ip, port, username, password).await?;
    println!("{}", diagnostics);

    Ok(())
}

async fn get_diagnostics(
    ip: &str,
    port: &str,
    username: &str,
    password: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut telnet = mini_telnet::Telnet::builder()
        .prompt("> ")
        .login_prompt("Login: ", "Password: ")
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(5))
        .connect(&format!("{}:{}", ip, port))
        .await?;
    telnet.login(username, password).await?;

    let response = telnet.execute("dumpmdm").await?;

    // Hack to fix extraneous newlines in mini-telnet output.
    Ok(response.replace("\n\n", "\n"))
}
