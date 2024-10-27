use std::fs;
use tera::{Context, Tera};

use crate::TeraOptions;

pub fn create_project_files(option: &TeraOptions) -> std::io::Result<()> {
  let tera = match Tera::new("templates/**/*.tera") {
    Ok(t) => t,
    Err(e) => {
      println!("Parsing error(s): {}", e.to_string());
      std::process::exit(1);
    }
  };

  let mut context = Context::new();
  context.insert("project_name", &option.project_name);
  context.insert("content", "my-content");

  let rendered = tera.render("temp.md.tera", &context).expect("Failed to render template");

  fs::write("./out/README.md", rendered)?;

  Ok(())
}
