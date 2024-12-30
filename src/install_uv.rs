#[cfg(target_os = "linux")]
use std::process::{Command, Stdio};

#[cfg(target_os = "windows")]
pub fn is_installed_uv() -> bool {
  // Windows specific code
  return true;
}

#[cfg(target_os = "windows")]
pub fn install_uv() -> Result<(), Box<dyn std::error::Error>> {
  // Windows specific code

  return Ok(());
}

#[cfg(target_os = "linux")]
pub fn is_installed_uv() -> bool {
  // Linux specific code

  let output = Command::new("which").arg("uv").output().unwrap();

  if output.status.success() {
    return true;
  } else {
    return false;
  }
}

#[cfg(target_os = "linux")]
pub fn install_uv() -> Result<(), Box<dyn std::error::Error>> {
  // Linux specific code

  // curlがインストールされているか確認
  let output = Command::new("curl").arg("--version").output()?;
  if !output.status.success() {
    return Err("curl is not installed".into());
  }

  // curlでuvをダウンロードしインストール
  let curl = Command::new("curl").arg("-LsSf").arg("https://astral.sh/uv/install.sh").stdout(Stdio::piped()).spawn().expect("Failed to execute command");

  let shell = Command::new("sh").stdin(curl.stdout.expect("failed to open echo stdout")).stdout(Stdio::piped()).spawn().expect("failed to execute sh command");

  let output = shell.wait_with_output().expect("failed to wait on grep");

  if output.status.success() {
    println!("{}", String::from_utf8_lossy(&output.stdout));
    return Ok(());
  } else {
    eprintln!("Command failed with error:\n{}", String::from_utf8_lossy(&output.stderr));
    return Err("Failed to install uv".into());
  }
}
