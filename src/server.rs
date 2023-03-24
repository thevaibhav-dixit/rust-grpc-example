use super::todo::task_service_server::TaskService;
use super::todo::{Name, Task, TaskChangeResponse, TaskUpdateRequest, TaskUpdateResponse};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tonic::{Request, Response, Status};

pub struct TodoStruct {
    map: Arc<Mutex<HashMap<String, Task>>>,
}
impl Default for TodoStruct {
    fn default() -> Self {
        return Self {
            map: Arc::new(Mutex::new(HashMap::<String, Task>::new())),
        };
    }
}
#[tonic::async_trait]
impl TaskService for TodoStruct {
    async fn add(&self, request: Request<Task>) -> Result<Response<TaskChangeResponse>, Status> {
        // we check for the request
        let task = request.into_inner();
        let name = match task.name.as_ref() {
            Some(name) => name.name.clone(),
            None => return Err(Status::invalid_argument("name is required")),
        };
        let mut task_map = self.map.lock().await;
        task_map.insert(name, task);
        return Ok(Response::new({
            TaskChangeResponse {
                status: "success".to_string(),
            }
        }));
    }
    async fn delete(&self, request: Request<Name>) -> Result<Response<TaskChangeResponse>, Status> {
        let mut task_map = self.map.lock().await;
        let name = request.into_inner();
        let key = name.name.clone();
        if let Some(k) = task_map.get(&key) {
            task_map.remove(&key);
        } else {
            return Err(Status::not_found("Name_NOT_FOUND"));
        }
        return Ok(Response::new(TaskChangeResponse {
            status: "deleted".to_string(),
        }));
    }
    async fn update(
        &self,
        request: Request<TaskUpdateRequest>,
    ) -> Result<Response<TaskUpdateResponse>, Status> {
        return Ok(Response::new(TaskUpdateResponse {
            status: "updated".to_string(),
            task: Some(Task {
                name: Some(Name {
                    name: "test".to_string(),
                }),
                desc: "some_desc".to_string(),
            }),
        }));
    }
    async fn get(&self, request: Request<Name>) -> Result<Response<Task>, Status> {
        println!("{:?}", request);
        let req = request.into_inner();
        let name = req.name;
        let map = self.map.lock().await;
        if let Some(task) = map.get(&name) {
            return Ok(Response::new(task.clone()));
        }
        return Err(Status::not_found("NAME_NOT_FOUND"));
    }
}
