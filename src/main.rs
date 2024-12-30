mod install_uv;
mod options_select;
mod template_gen;
mod types;

use dialoguer::Confirm;
use install_uv::is_installed_uv;
use types::tera_options::DevEnv;

use crate::types::tera_options::TeraOptions;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

fn main() {
  let options = options_select::options_select().unwrap();

  // ローカルが選択された場合、uvがインストールされているか確認し、インストールされていない場合はインストール
  if let DevEnv::Local = options.dev_env {
    if !is_installed_uv() {
      let confirm = Confirm::new()
        .with_prompt("uv is not installed. This Python environment assumes uv: https://docs.astral.sh/uv/. Do you want to install it?")
        .interact()
        .expect("Failed to get user input");

      if confirm {
        install_uv::install_uv().expect("Failed to install uv");
      }
    }
  }

  let templates = load_templates();
  template_gen::create_project_files(&templates, &options).expect("Failed to create project files");
}
