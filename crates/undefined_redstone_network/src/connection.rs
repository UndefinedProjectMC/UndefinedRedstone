use rak_rs::connection::Connection;
use rak_rs::Listener;
use crate::URNetworkSettings;

pub(crate) async fn accept_connection(settings: URNetworkSettings) {
    let mut listener = Listener::bind(format!("0.0.0.0:{}", settings.0.server_port)).await.unwrap();
    listener.id = settings.0.guid;
    listener.motd = settings.0.server_motd.clone();
    listener.start().await.unwrap();
    loop {
        if let Ok(connection) = listener.accept().await {
            tokio::spawn(handle_connection(connection));
        }
    }
}

pub(crate) async fn handle_connection(connection: Connection) {
    println!("handle connection ok!");
}