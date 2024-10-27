use crate::TeraOptions;
use dialoguer::Input;

pub fn options_select<'a>() -> Result<TeraOptions, Box<dyn std::error::Error>> {
  let project_name: String = Input::new().with_prompt("Python Project Name?").interact_text()?;

  return Ok(TeraOptions { project_name });
}
