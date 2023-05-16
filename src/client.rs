use data_service::sub_data_client::SubDataClient;
use data_service::SubDataRequest;

pub mod data_service {
    tonic::include_proto!("data");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SubDataClient::connect("http://127.0.0.1:50051").await?;

    let request = tonic::Request::new(SubDataRequest {
        symbol: "btcusdt".into(),
        exchange: 0,
        market_type: 0,
    });

    let response = client.sub_kline_data(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
