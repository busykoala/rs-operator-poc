use crate::config;
use crate::k8s;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Service;
use kube::{Api, Client, config::Config};

pub struct K8sApis {
    pub deployment: Api<Deployment>,
    pub service: Api<Service>,
}

pub fn get_apis() -> K8sApis {
    let env = config::get_environment();
    return K8sApis {
        deployment: Api::namespaced(k8s::get_client(), &env.namespace),
        service: Api::namespaced(k8s::get_client(), &env.namespace)
    }
}

fn get_client() -> Client {
    let config = Config::incluster_env().expect(
        "Unable to load the incluster environment config.");
    let client = Client::try_from(config).expect(
        "Failed to init the k8s client with the given config.");
    return client
}
