use std::{thread, time::Duration};

use anyhow::Result;
use config::ConfigConnection;
use tokio::{io::{self, AsyncWriteExt}, process::Command};

mod config;

#[tokio::main]
async fn main() {

    let connections_result: Result<Vec<ConfigConnection>> = config::load();

    if let Err(err) = connections_result {
        println!("Error: {}", err);
        return;
    }

    let connections = connections_result.unwrap();

    println!("Checking Runops authentication...");

    if !is_authenticated().await {
        println!("Not authenticated");
        println!("Try running 'runops login' on your terminal...");
        return;
    }

    println!("Authenticated!");

    for connection in connections {
        Command::new("rproxy")
            .arg(connection.r#type)
            .arg("-c")
            .arg(connection.target)
            .arg("-p")
            .arg(connection.port.to_string())
            .arg("&")
            .spawn()
            .expect("Could not start child process");
    }

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}

async fn is_authenticated() -> bool {
    let args = ["tasks", "list"];
    let output = Command::new("runops")
        .args(&args)
        .output()
        .await
        .expect("Failed to execute command");

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        !output_str.contains("Invalid credentials")
    }
    else {
        io::stderr().write_all(&output.stderr).await.unwrap();
        std::process::exit(1);
    }
}