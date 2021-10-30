use super::MissionRpc;
use stubs::group::group_service_server::GroupService;
use stubs::*;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl GroupService for MissionRpc {
    async fn get_units(
        &self,
        request: Request<group::GetUnitsRequest>,
    ) -> Result<Response<group::GetUnitsResponse>, Status> {
        let res: group::GetUnitsResponse = self.request("getUnits", request).await?;
        Ok(Response::new(res))
    }
}