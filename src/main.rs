use NexusCore_MC::raknet::server::RakNetServer;
use NexusCore_MC::server::Server;
use NexusCore_MC::log_t;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    
    // Language selection
    NexusCore_MC::lang::prompt_language();
    
    let args: Vec<String> = std::env::args().collect();
    let port: u16 = if args.len() > 1 {
        args[1].parse().unwrap_or(19132)
    } else {
        19132
    };
    
    let bind_addr = format!("0.0.0.0:{}", port);
    log_t!(info, SERVER_STARTING, bind_addr);
    
    let (event_tx, event_rx) = mpsc::channel(100);
    
    let (server_impl, cmd_tx) = RakNetServer::new(&bind_addr, event_tx).await?;
    
    // Spawn server thread
    tokio::spawn(async move {
        server_impl.run().await;
    });
    
    log_t!(info, LISTENING);
    
    let server = Server::new(cmd_tx, event_rx);
    server.run().await?;
    
    Ok(())
}
