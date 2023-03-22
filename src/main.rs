mod server;
mod todo;
use tonic::transport::Server;

use server::TodoStruct;
use todo::task_service_server::TaskServiceServer;

mod store_proto {
   include!("todo.rs");

   pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
      tonic::include_file_descriptor_set!("todo_descriptor");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   let addr = "127.0.0.1:9001".parse()?;
   let todo = TodoStruct::default();

   let reflection_service = tonic_reflection::server::Builder::configure()
           .register_encoded_file_descriptor_set(store_proto::FILE_DESCRIPTOR_SET)
           .build()
           .unwrap();

   Server::builder()
           .add_service(TaskServiceServer::new(todo))
           .add_service(reflection_service)
           .serve(addr)
           .await?;
   Ok(())
}
