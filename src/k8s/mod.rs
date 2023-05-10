use crate::config;
use crate::k8s;
use k8s_openapi::api::apps::v1::{Deployment, StatefulSet};
use k8s_openapi::api::core::v1::{
    Service, ConfigMap, Secret, PersistentVolumeClaim
};
use k8s_openapi::api::networking::v1::Ingress;
use k8s_openapi::api::batch::v1::CronJob;
use kube::{Api, Client, config::Config};

pub struct K8sApis {
    pub config_map: Api<ConfigMap>,
    pub cron_job: Api<CronJob>,
    pub deployment: Api<Deployment>,
    pub ingress: Api<Ingress>,
    pub pvc: Api<PersistentVolumeClaim>,
    pub secret: Api<Secret>,
    pub service: Api<Service>,
    pub stateful_set: Api<StatefulSet>,
}

pub fn get_apis() -> K8sApis {
    let env = config::get_environment();
    return K8sApis {
        config_map: Api::namespaced(k8s::get_client(), &env.namespace),
        cron_job: Api::namespaced(k8s::get_client(), &env.namespace),
        deployment: Api::namespaced(k8s::get_client(), &env.namespace),
        ingress: Api::namespaced(k8s::get_client(), &env.namespace),
        pvc: Api::namespaced(k8s::get_client(), &env.namespace),
        secret: Api::namespaced(k8s::get_client(), &env.namespace),
        service: Api::namespaced(k8s::get_client(), &env.namespace),
        stateful_set: Api::namespaced(k8s::get_client(), &env.namespace),
    }
}

fn get_client() -> Client {
    let config = Config::incluster_env().expect(
        "Unable to load the incluster environment config.");
    let client = Client::try_from(config).expect(
        "Failed to init the k8s client with the given config.");
    return client
}
