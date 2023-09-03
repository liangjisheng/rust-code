pub mod voting {
    // 如果没有修改过OUT_DIR环境变量，则可以通过tonic::include_proto!("voting")
    // 宏直接导入build.rs编译后得到的voting.rs文件
    // tonic::include_proto!("voting");

    // 如果修改过OUT_DIR环境变量的值，或者tonic_build编译时明确指定了输出路径(稍后给代码)，
    // 则不能使用tonic::include_proto!宏来导入voting.rs。此时应该用Rust自身提供的宏
    // include!来明确指定导入的路径
    include!("../protos/voting.rs");
}

use tonic::{transport::Server, Request, Response, Status};
use voting::{
    voting_server::{Voting, VotingServer},
    VotingRequest, VotingResponse,
};

#[derive(Debug, Default)]
pub struct VotingService {}

#[tonic::async_trait]
impl Voting for VotingService {
    async fn vote(
        &self,
        request: Request<VotingRequest>,
    ) -> Result<Response<VotingResponse>, Status> {
        let r: &VotingRequest = request.get_ref();
        match r.vote {
            0 => Ok(Response::new(voting::VotingResponse {
                confirmation: { format!("upvoted for {}", r.url) },
            })),
            1 => Ok(Response::new(voting::VotingResponse {
                confirmation: { format!("downvoted for {}", r.url) },
            })),
            _ => Err(Status::new(
                tonic::Code::OutOfRange,
                "Invalid vote provided",
            )),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:8080".parse().unwrap();
    let voting_service = VotingService::default();
    println!("start server at {}", address);

    Server::builder()
        .add_service(VotingServer::new(voting_service))
        .serve(address)
        .await?;

    Ok(())
}
