use polocloud_sdk::events::{ServiceOnlineEvent, ServiceShutdownEvent};
use polocloud_sdk::Polocloud;
use std::io::stdin;
use tokio::main;

#[main]
async fn main() {
    let client_mutex = Polocloud::instance().await;

    let client = client_mutex.lock().unwrap();

    let event_provider_mutex = client.event_provider();
    let mut event_provider = event_provider_mutex.lock().unwrap();
    //event_provider.subscribe("ServiceOnlineEvent", callback_online).await.unwrap();
    event_provider.subscribe(callback_shutdown).await.unwrap();

    //groups
    let group_provider_mutex = client.group_provider();
    let mut group_provider = group_provider_mutex.lock().unwrap();
    let group = group_provider.find_by_name_async("test".to_string()).await.unwrap();
    let group_not_found = group_provider.find_by_name_async("asdkfjljldddddddddddddddddddddddd".to_string()).await.unwrap();
    let groups = group_provider.find_async().await.unwrap();

    println!("Test Group: {:?}", group);
    println!("Group not found: {:?}", group_not_found);
    println!("All Groups: {:?}", groups);

    let service_provider_mutex = client.service_provider();
    let mut service_provider = service_provider_mutex.lock().unwrap();
    let services = service_provider.find_async().await.unwrap();

    println!("All Services: {:?}", services);


    let mut s=String::new();
    let _ = stdin().read_line(&mut s);
}

fn callback_online(event: ServiceOnlineEvent) {
    println!("{:?}", event);
}

fn callback_shutdown(event: ServiceShutdownEvent) {
    println!("{:?}", event);
}
