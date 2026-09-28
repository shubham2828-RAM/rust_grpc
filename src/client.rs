use tonic::Request;

pub mod user {
    tonic::include_proto!("user");
}

use user::user_service_client::UserServiceClient;
use user::GetUserRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client =
        UserServiceClient::connect("http://127.0.0.1:50051").await?;

    let request = Request::new(GetUserRequest {
        user_id: 101,
    });

    let response = client.get_user(request).await?;

    println!("Response: {:?}", response.into_inner());

    Ok(())
}