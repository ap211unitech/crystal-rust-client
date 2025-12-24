use rust_client::{
    self,
    client::{Block200Response, GetBlocksRequest, GetBlocksResponse, SubmergeCrystalApiV1Client},
};

#[tokio::main]
async fn main() {
    let client = SubmergeCrystalApiV1Client::new();
    let response: rust_client::client::GetBlocksResponse = client
        .get_blocks(GetBlocksRequest {
            ..Default::default()
        })
        .await
        .unwrap();

    if let GetBlocksResponse::Ok(Block200Response {
        data,
        pagination: _,
    }) = response
    {
        println!("{:#?}", data.iter().next());
    }
}
