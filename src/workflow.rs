#[derive(Serialize, Deserialize, Debug)]
pub struct Workflow {
    pub component_id: String,
    pub workflow_id: String,
    pub configuration: Configuration,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub id: String,
    pub schedule: String,
    pub offset: Option<String>,
    pub docker_termination_logging: bool,
    pub docker_image: Option<String>,
    pub docker_args: Option<Vec<String>>,
    pub commit_sha: Option<String>,
    pub service_account: Option<String>,
    pub resources: Vec<String>,
}
