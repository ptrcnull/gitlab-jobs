#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::Mutex;

use gitlab::{api::{Query, AsyncQuery, projects::jobs::RetryJob}, Runner, Gitlab, AsyncGitlab};
use gitlab_api::Job;

pub mod gitlab_api;

struct AppState {
  gl: Option<AsyncGitlab>
}

impl AppState {
  fn default() -> Self {
      Self {
        gl: None
      }
  }

  fn get_client(&self) -> Result<AsyncGitlab, String> {
    match &self.gl {
      Some(client) => Ok(client.clone()),
      None => {
        Err("not logged in".to_string())
      }
    }
  }
}

type State<'a> = tauri::State<'a, Mutex<AppState>>;

#[tauri::command]
fn is_logged_in(state: State) -> bool {
  state.lock().unwrap().gl.is_some()
}

#[tauri::command]
async fn log_in(token: String, state: State<'_>) -> Result<(), String> {
  let gitlab_client = gitlab::GitlabBuilder::new("gitlab.com", token).build_async().await;
  match gitlab_client {
    Ok(client) => {
      state.lock().unwrap().gl.replace(client);
      Ok(())
    },
    Err(error) => Err(error.to_string())
  }
}

#[tauri::command]
async fn list_runners(state: State<'_>) -> Result<Vec<Runner>, String> {
  let client = {
    let appstate = state.lock().unwrap();
    appstate.get_client()
  }?;

  let endpoint = gitlab_api::Runners{};
  let response: Result<Vec<Runner>, _> = endpoint.query_async(&client).await;
  response.map_err(|err| err.to_string())
}

#[tauri::command]
async fn list_jobs_for_runner(runner_id: String, state: State<'_>) -> Result<Vec<Job>, String> {
  let client = {
    let appstate = state.lock().unwrap();
    appstate.get_client()
  }?;

  let endpoint = gitlab_api::RunnerJobs{runner_id};
  let response: Result<Vec<Job>, _> = endpoint.query_async(&client).await;
  response.map_err(|err| err.to_string())
}

#[tauri::command]
async fn retry_job(project_id: String, job_id: u64, state: State<'_>) -> Result<Job, String> {
  let client = {
    let appstate = state.lock().unwrap();
    appstate.get_client()
  }?;

  let endpoint = RetryJob::builder().project(project_id).job(job_id).build().unwrap();
  let response: Result<Job, _> = endpoint.query_async(&client).await;
  response.map_err(|err| err.to_string())
}

fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .manage(Mutex::new(AppState::default()))
    .invoke_handler(tauri::generate_handler![
      is_logged_in,
      log_in,
      list_runners,
      list_jobs_for_runner,
      retry_job
    ])
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .run(context)
    .expect("error while running tauri application");
}
