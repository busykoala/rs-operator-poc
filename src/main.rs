use std::thread::sleep;
use k8s_openapi::api::apps::v1::Deployment;
use kube::{Api, Client, config::Config};
use std::{fs, env, time::Duration};


struct Env {
    namespace: String,
    deployment: String,
}


fn get_environment() -> Env {
    let file_path = "/var/run/secrets/kubernetes.io/serviceaccount/namespace";
    let namespace = fs::read_to_string(file_path).expect(
        "namespace file should have been read.");
    let deployment = env::var("DEPLOYMENT").expect("$DEPLOYMENT is not set");
    Env {
        namespace: namespace,
        deployment: deployment,
    }
}


fn get_client() -> Client {
    let config = Config::incluster_env().unwrap();
    let client = Client::try_from(config).unwrap();
    return client
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = get_environment();
    loop {
        let client= get_client();
        let deployments: Api<Deployment> = Api::namespaced(client, &env.namespace);
        let pg_depl = deployments.get(&env.deployment).await?;
        println!("{:?}", pg_depl.spec.unwrap());
        sleep(Duration::from_secs(5));
    }
}
