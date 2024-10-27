mod options_select;
mod template_gen;
mod types;

use crate::types::tera_options::TeraOptions;

fn main() {
  let options = options_select::options_select().unwrap();
  template_gen::create_project_files(&options).expect("Failed to create project files");
}
