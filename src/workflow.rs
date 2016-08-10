#[derive(RustcDecodable, Debug)]
pub struct Workflow {
    pub component_uri: String,
    pub component_id: String,
    pub endpoint_id: String,
    pub schedule: Schedule,
}

#[derive(RustcDecodable, Debug)]
pub struct Schedule {
    pub id: String,
    pub partitioning: String,
    pub docker_image: Option<String>,
    pub docker_args: Option<Vec<String>>,
    pub secret: Option<Secret>,
}

#[derive(RustcDecodable, Debug)]
pub struct Secret {
    pub name: String,
    pub mount_path: String,
}
