pub mod data_service {
    tonic::include_proto!("data");
}
use data_service::sub_data_server::SubData;
use data_service::{SubDataRequest, SubDataResponse};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct DataService {}

#[tonic::async_trait]
impl SubData for DataService {
    async fn sub_kline_data(
        &self,
        request: Request<SubDataRequest>,
    ) -> Result<Response<SubDataResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = SubDataResponse {
            successful: true,
            message: "Hello, world!".into(),
        };
        Ok(Response::new(reply))
    }
}
