use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    id: u8,
    userId: u8,
    title: String,
    completed: bool
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://jsonplaceholder.typicode.com/todos/1")
        .await?.text().await?;

    let mapped: Person = serde_json::from_str(&resp).unwrap();
    println!("{:?}", mapped);
    Ok(())
}
