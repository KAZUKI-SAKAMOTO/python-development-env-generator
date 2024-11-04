use core::fmt;
use std::path::Path;

use convert_case::{Case, Casing};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
  static ref PYTHON_VERSION_REGEX: Regex = Regex::new(r"^[2-3]+(\.\d+\.\d+)|(\.\d+)$").unwrap();
  static ref PROJECT_NAME_REGEX: Regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9-_]*$").unwrap();
}

pub fn validate_project_name(project_name: &String) -> Result<(), String> {
  if project_name.len() < 3 || project_name.len() > 50 {
    return Err("Project name must be between 3 and 50 characters".to_string());
  }

  if !PROJECT_NAME_REGEX.is_match(project_name) {
    return Err("Project must be alphanumeric and start with a letter".to_string());
  }

  let project_dir = format!("./{}", project_name.to_case(Case::Pascal));
  let path = Path::new(&project_dir);

  if path.exists() {
    return Err("Project already exists".to_string());
  }

  Ok(())
}

pub fn validate_python_version(python_version: &String) -> Result<(), String> {
  if !PYTHON_VERSION_REGEX.is_match(python_version) {
    return Err("Invalid python version".to_string());
  }
  Ok(())
}

#[derive(Debug, Copy, Clone)]
pub enum DevEnv {
  Local,
  Docker,
}

impl fmt::Display for DevEnv {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      DevEnv::Local => write!(f, "Local"),
      DevEnv::Docker => write!(f, "Docker"),
    }
  }
}

pub struct TeraOptions {
  pub project_dir_name: String,
  pub project_title: String,
  pub python_version: String,
  pub dev_env: DevEnv,
}
