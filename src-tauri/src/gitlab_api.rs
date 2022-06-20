// additional queries not defined in the gitlab crate

use gitlab::{api::{endpoint_prelude::Method, Endpoint}, *};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

pub struct RunnerJobs {
    pub runner_id: String,
}

impl Endpoint for RunnerJobs {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        format!("runners/{}/jobs?order_by=id", self.runner_id).into()
    }
}

pub struct Runners {}
impl Endpoint for Runners {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "runners".into()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job {
    /// The ID of the job.
    pub id: JobId,
    /// The status of the job.
    pub status: StatusState,
    pub stage: String,
    /// The name of the job.
    pub name: String,
    #[serde(rename = "ref")]
    /// The name of the reference that was tested.
    pub ref_: Option<String>,
    pub tag: bool,
    pub coverage: Option<f64>,
    /// When the job was created or marked as pending.
    pub created_at: DateTime<Utc>,
    /// When the job was started.
    pub started_at: Option<DateTime<Utc>>,
    /// When the job completed.
    pub finished_at: Option<DateTime<Utc>>,
    /// The user which ran the job.
    pub user: Option<User>,
    /// The artifact file uploaded from the job.
    pub artifacts_file: Option<JobArtifactFile>,
    /// The commit the job tested.
    pub commit: RepoCommit,
    /// The runner which ran the job.
    pub runner: Option<Runner>,
    /// The pipeline the job belongs to.
    pub pipeline: PipelineBasic,
    pub allow_failure: bool,
    pub duration: Option<f64>,
    pub artifacts: Option<Vec<JobArtifact>>,
    pub artifacts_expire_at: Option<DateTime<Utc>>,
    pub web_url: String,
    pub project: ProjectInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectInfo {
    pub id: ProjectId,
    pub name: String,
    pub name_with_namespace: String,
    pub path: String,
    pub path_with_namespace: String,
}
