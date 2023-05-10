use std::time::Duration;

mod k8s;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = config::get_environment();
    let apis = k8s::get_apis();
    loop {
        match apis.deployment.get(&env.deployment).await {
            Ok(depl_info) => {
                if let Some(spec) = depl_info.spec {
                    println!("{:?}", spec);
                }
            },
            Err(e) => {
                eprintln!("Error getting deployment: {}", e);
            },
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
