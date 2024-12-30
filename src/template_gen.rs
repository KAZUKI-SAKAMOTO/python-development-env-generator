use std::{
  collections::HashMap,
  fs::{self, File},
  io::{self, Write},
  path::PathBuf,
};
use tera::{Context, Tera};

use crate::{types::tera_options::DevEnv, TeraOptions};

pub fn create_project_files(templates: &HashMap<&str, &str>, option: &TeraOptions) -> std::io::Result<()> {
  let mut tera = Tera::default();
  for (name, content) in templates.iter() {
    tera.add_raw_template(name, content).expect("Failed to add template");
  }

  create_standard_files(&tera, &option).expect("Failed to create standard files");

  if let DevEnv::Docker = option.dev_env {
    create_docker_files(&tera, &option).expect("Failed to create docker files");
  }

  Ok(())
}

fn create_docker_files(tera: &Tera, option: &TeraOptions) -> std::io::Result<()> {
  let ctx = Context::new();

  for file_name in tera.get_template_names() {
    if file_name.contains("docker/") {
      let rendered = tera.render(file_name, &ctx).expect("Failed to render template");
      let path = replace_tera_file_path("docker/", &option.project_dir_name, &file_name);

      write_to_file_with_dirs(&path, &rendered).expect("Failed to write file");
    }
  }

  return Ok(());
}

fn create_standard_files(tera: &Tera, option: &TeraOptions) -> std::io::Result<()> {
  let mut ctx = Context::new();
  ctx.insert("project_name", &option.project_dir_name);
  ctx.insert("project_name_title", &option.project_title);
  ctx.insert("python_version", &option.python_version);

  for file_name in tera.get_template_names() {
    if file_name.contains("standard/") {
      let rendered = tera.render(file_name, &ctx).expect("Failed to render template");
      let path = replace_tera_file_path("standard/", &option.project_dir_name, &file_name);

      write_to_file_with_dirs(&path, &rendered).expect("Failed to write file");
    }
  }

  return Ok(());
}

fn replace_tera_file_path(prefix: &str, project_dir_name: &str, file_path: &str) -> PathBuf {
  let path_name = file_path.replace(prefix, &format!("{}/", project_dir_name)).replace(".tera", "");
  return PathBuf::from(path_name);
}

fn write_to_file_with_dirs(path: &PathBuf, content: &str) -> io::Result<()> {
  if let Some(parent) = path.parent() {
    fs::create_dir_all(parent).expect("Failed to create directories");
  }

  println!("Generate file: {:?}", path);

  let mut file = File::create(path).expect("Failed to create file");
  file.write_all(content.as_bytes()).expect("Failed to write file");

  Ok(())
}
