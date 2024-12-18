use todo::todo_client::TodoClient;
use todo::CreateTodoRequest;
use tokio::time::{sleep, Duration};

pub mod todo {
    tonic::include_proto!("todo");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_addr = "http://127.0.0.1:8080";
    let mut client = TodoClient::connect(server_addr).await?;
    loop {
        let create_request = tonic::Request::new(CreateTodoRequest {
            name: String::from("coding"),
            description: String::from("jolly"),
            priority: 1,
        });

        let create_response = client.create_todo(create_request).await?;

        println!("{:?}", create_response.into_inner().todo);

        let request = tonic::Request::new(());

        let response = client.get_todos(request).await?;

        println!("{:?}", response.into_inner().todos);

        sleep(Duration::from_secs(5)).await;
    }
}
