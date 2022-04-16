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

    let dumpmdm_xml = get_dumpmdm_xml(ip, port, username, password).await?;
    let dumpmdm_xml_parsed = roxmltree::Document::parse(&dumpmdm_xml).unwrap();
    println!(
        "XML document has {} descendants",
        dumpmdm_xml_parsed.descendants().count()
    );

    let dumpmdm_dsl_interface_config = dumpmdm_xml_parsed
        .descendants()
        .find(|n| n.has_tag_name("WANDSLInterfaceConfig"))
        .unwrap();
    println!(
        "WANDSLInterfaceConfig node has {} descendants",
        dumpmdm_dsl_interface_config.descendants().count()
    );

    Ok(())
}

async fn get_dumpmdm_xml(
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

    // Fetch output of dumpmdm command, skipping the "Dump of Entire MDM, this is NOT the config
    // file" line at the beginning.
    let response = telnet
        .execute("dumpmdm")
        .await?
        .lines()
        .skip(1)
        .collect::<String>()
        // Workaround for extraneous newlines in mini-telnet output.
        .replace("\n\n", "\n");

    // Hack to fix extraneous newlines in mini-telnet output.
    Ok(response)
}
