use reqwest::Client;
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct FormKeyValue {
    key: String,
    value: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct FormKey {
    key: String,
}

#[tokio::main]
async fn main(){
    let client = Client::new();
    let addr = "http://127.0.0.1:3000";
    let resp = client.get(format!("{}/ping_http", addr)).send().await;
    println!("ping_http {:?}", resp.unwrap().text().await);
    let resp = client.get(format!("{}/ping_rpc", addr)).send().await;
    println!("ping_rpc {:?}", resp.unwrap().text().await);
    let resp = client.post(format!("{}/set_rpc", addr)).form(&FormKeyValue{
        key: "key".to_string(),
        value: "value".to_string(),
    }).send().await;
    println!("set_rpc {:?}", resp.unwrap().text().await);
    let resp = client.post(format!("{}/get_rpc", addr)).form(&FormKey{
        key: "key".to_string(),
    }).send().await;
    println!("get_rpc {:?}", resp.unwrap().text().await);
    let resp = client.post(format!("{}/del_rpc", addr)).form(&FormKey{
        key: "key".to_string(),
    }).send().await;
    println!("del_rpc {:?}", resp.unwrap().text().await);
    let resp = client.post(format!("{}/get_rpc", addr)).form(&FormKey{
        key: "key".to_string(),
    }).send().await;
    println!("get_rpc again {:?}", resp.unwrap().text().await);
}