use super::todo::task_service_server::TaskService;
use super::todo::{
    Task, TaskAddReq, TaskChangeResponse, TaskId, TaskUpdateRequest, TaskUpdateResponse,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tonic::{Request, Response, Status};

pub struct TodoStruct {
    map: Arc<Mutex<HashMap<i32, Task>>>,
}
impl Default for TodoStruct {
    fn default() -> Self {
        return Self {
            map: Arc::new(Mutex::new(HashMap::<i32, Task>::new())),
        };
    }
}
#[tonic::async_trait]
impl TaskService for TodoStruct {
    async fn add(
        &self,
        request: Request<TaskAddReq>,
    ) -> Result<Response<TaskChangeResponse>, Status> {
        // we check for the request
        let task = request.into_inner();
        if task.name == "" || task.desc == "" {
            return Err(Status::invalid_argument("EMPTY_ARGUMENT_PROVIDED"));
        }
        let task_builder = Task {
            id: Some(TaskId { id: 12 }),
            name: task.name,
            desc: task.desc,
        };
        let mut data_map = self.map.lock().await;
        data_map.insert(12, task_builder);
        return Ok(Response::new(TaskChangeResponse {
            status: "Success".to_string(),
        }));
    }
    async fn delete(
        &self,
        request: Request<TaskId>,
    ) -> Result<Response<TaskChangeResponse>, Status> {
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
                id: Some(TaskId { id: 1 }),
                name: "some task".to_string(),
                desc: "some desc".to_string(),
            }),
        }));
    }
    async fn get(&self, request: Request<TaskId>) -> Result<Response<Task>, Status> {
        let req = request.into_inner();
        let map = self.map.lock().await;
        if let Some(task) = map.get(&req.id) {
            return Ok(Response::new(task.clone()));
        }
        return Err(Status::not_found("ID_NOT_FOUND"));
    }
}
