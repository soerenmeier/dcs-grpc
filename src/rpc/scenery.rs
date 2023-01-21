use super::MissionRpc;
use stubs::scenery::v0::scenery_service_server::SceneryService;
use stubs::*;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl SceneryService for MissionRpc {
    async fn destroy(
        &self,
        request: Request<scenery::v0::DestroyRequest>,
    ) -> Result<Response<scenery::v0::DestroyResponse>, Status> {
        let res = self.request("sceneryDestroy", request).await?;
        Ok(Response::new(res))
    }
}
