use super::MissionRpc;
use stubs::r#static::v0::{static_service_server::StaticService, DestroyRequest, DestroyResponse};
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl StaticService for MissionRpc {
    async fn destroy(
        &self,
        request: Request<DestroyRequest>,
    ) -> Result<Response<DestroyResponse>, Status> {
        let res = self.request("staticDestroy", request).await?;
        Ok(Response::new(res))
    }
}
