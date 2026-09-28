use tonic::{transport::Server, Request, Response, Status};

pub mod user {
    tonic::include_proto!("user");
}

use user::user_service_server::{
    UserService,
    UserServiceServer,
};
use user::{GetUserRequest, GetUserResponse};

#[derive(Default)]
struct MyUserService;

#[tonic::async_trait]
impl UserService for MyUserService {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let user_id = request.into_inner().user_id;

        println!("Received user_id: {}", user_id);

        let response = GetUserResponse {
            user_id,
            name: "Shubham Sharma".to_string(),
            email: "shubham@example.com".to_string(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;

    let user_service = MyUserService::default();

    println!("gRPC server listening on {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}