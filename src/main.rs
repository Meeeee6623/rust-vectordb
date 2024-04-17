use anyhow::Result;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{
    Condition, CreateCollection, Filter, SearchPoints, VectorParams, VectorsConfig,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let client = QdrantClient::from_url("http://localhost:6334").build()?;

    let collections_list = client.list_collections().await?;
    dbg!(collections_list);

    let collection_name = "test";
    client.delete_collection(collection_name).await?;

    client
        .create_collection(&CreateCollection {
            collection_name: collection_name.into(),
            vectors_config: Some(VectorsConfig {
                config: Some(Config::Params(VectorParams {
                    size: 10,
                    distance: Distance::Cosine.into(),
                    ..Default::default()
                })),
            }),
            ..Default::default()
        })
        .await?;

    // upload 10 new points with the values a: 1, b: 2, ... i: 9, j: 10

let points = (0..10)
    .map(|i| {
        let key = (b'a' + i as u8) as char;
        let payload: Payload = json!({ key.to_string(): i + 1 })
            .try_into()
            .unwrap();
        PointStruct::new(i, vec![i as f32; 10], payload)
    })
    .collect::<Vec<_>>();

client
    .upsert_points_blocking(collection_name, None, points, None)
    .await?;

    // search for the point with the value of "b" equal to 2

    let search_result = client
        .search_points(&SearchPoints {
            collection_name: collection_name.into(),
            vector: vec![11.; 10],
            filter: Some(Filter::all([Condition::matches("b", 2)])),
            limit: 10,
            with_payload: Some(true.into()),
            ..Default::default()
        })
        .await?;
    dbg!(&search_result);


    Ok(())
}