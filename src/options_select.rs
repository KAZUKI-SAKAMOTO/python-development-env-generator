use crate::{
  types::tera_options::{validate_project_name, validate_python_version, DevEnv},
  TeraOptions,
};
use convert_case::{Case, Casing};
use dialoguer::{Input, Select};

pub fn options_select<'a>() -> Result<TeraOptions, Box<dyn std::error::Error>> {
  // プロジェクト名を入力
  let project_name: String =
    Input::new().with_prompt("Python Project Name?").validate_with(validate_project_name).interact_text().expect("Failed to get project name");

  // Pythonバージョンを入力
  let python_version: String =
    Input::new().with_prompt("Which Python version do you use?").validate_with(validate_python_version).interact_text().expect("Failed to get python version");

  // 開発環境を選択
  println!("Select Development Environment. If you select Docker, a .devcontainer file will be created.");
  let items = vec![DevEnv::Local, DevEnv::Docker];
  let items_vec = items.iter().map(|m| m.to_string()).collect::<Vec<_>>();
  let selected_index = Select::new().items(&items_vec).default(0).interact().expect("Failed to select");
  let selected = items[selected_index];

  return Ok(TeraOptions {
    project_dir_name: project_name.to_case(Case::Pascal),
    project_title: project_name.to_case(Case::Title),
    python_version,
    dev_env: selected,
  });
}
