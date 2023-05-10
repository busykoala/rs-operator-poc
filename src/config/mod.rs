use std::{fs, env};

pub struct OperatorConfig {
    pub namespace: String,
    pub deployment: String,
}

pub fn get_environment() -> OperatorConfig {
    let file_path = "/var/run/secrets/kubernetes.io/serviceaccount/namespace";
    let namespace = fs::read_to_string(file_path).expect(
        &format!("namespace not found in {}.", file_path));
    let deployment = env::var("DEPLOYMENT").expect(
        "$DEPLOYMENT env variable is not set.");
    return OperatorConfig {
        namespace,
        deployment,
    }
}
