use kube::{Client, config::Config};

pub fn get_client() -> Client {
    let config = Config::incluster_env().expect(
        "Unable to load the incluster environment config.");
    let client = Client::try_from(config).expect(
        "Failed to init the k8s client with the given config.");
    return client
}
