mod crypto_data;
mod dispatch;
mod settings;

use dispatch::data_service::sub_data_server::SubDataServer;
use dispatch::DataService;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let local_settings = settings::loads::load_server();
    println!("Settings: {:?}", local_settings.generate_url());

    let sub = DataService::default();
    let addr = local_settings.generate_url().parse().unwrap();

    Server::builder()
        .add_service(SubDataServer::new(sub))
        .serve(addr)
        .await
        .unwrap();
}
