use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
  let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable not set");
  let templates_out_dir = Path::new(&out_dir).join("templates");

  // テンプレートフォルダを`OUT_DIR/templates`にコピー
  fs::create_dir_all(&templates_out_dir).expect("Failed to create templates directory in OUT_DIR");
  copy_templates_to_out(Path::new("templates"), &templates_out_dir).expect("Failed to copy templates");

  // `OUT_DIR`内の`templates`フォルダを基に`templates.rs`を生成
  let dest_path = Path::new(&out_dir).join("templates.rs");
  let mut f = File::create(&dest_path).expect("Failed to create templates.rs file");

  writeln!(f, "use std::collections::HashMap;").expect("Failed to write to templates.rs");
  writeln!(f, "pub fn load_templates() -> HashMap<&'static str, &'static str> {{").expect("Failed to write to templates.rs");
  writeln!(f, "    let mut templates = HashMap::new();").expect("Failed to write to templates.rs");

  // コピーされた`OUT_DIR/templates`フォルダ内のファイルを読み込んで`templates.rs`に書き込む
  add_templates(&mut f, &templates_out_dir, "templates");

  writeln!(f, "    templates").expect("Failed to write to templates.rs");
  writeln!(f, "}}").expect("Failed to write to templates.rs");
}

// 再帰的にテンプレートフォルダをコピーする関数
fn copy_templates_to_out(src: &Path, dest: &Path) -> std::io::Result<()> {
  for entry in fs::read_dir(src)? {
    let entry = entry?;
    let path = entry.path();
    let dest_path = dest.join(path.file_name().unwrap());

    if path.is_dir() {
      fs::create_dir_all(&dest_path)?; // ディレクトリの再帰的コピー
      copy_templates_to_out(&path, &dest_path)?;
    } else {
      fs::copy(&path, &dest_path)?; // ファイルのコピー
    }
  }
  Ok(())
}

// 再帰的にテンプレートファイルを追加する関数
fn add_templates(f: &mut File, dir: &Path, base_path: &str) {
  for entry in fs::read_dir(dir).expect("Failed to read directory") {
    let entry = entry.expect("Failed to read entry");
    let path = entry.path();

    if path.is_dir() {
      // base_pathはそのままに、再帰呼び出しの際はパスを更新しない
      add_templates(f, &path, base_path);
    } else if let Some(ext) = path.extension() {
      if ext == "tera" {
        // `base_path/`以降を取得
        if let Some(base_path_index) = path.iter().position(|component| component == base_path) {
          // キーとしてファイル名、値としてファイルの中身を`templates.rs`に書き込む

          // キーを作成
          let key = path.iter().skip(base_path_index + 1).collect::<PathBuf>();
          let key_str = key.to_str().expect("Failed to convert path to string");

          // ファイル名を取得
          let relative_path = path.iter().skip(base_path_index).collect::<PathBuf>();
          let relative_path_str = relative_path.to_str().expect("Failed to convert path to string");

          writeln!(f, "    templates.insert(\"{}\", include_str!(\"{}\"));", key_str, relative_path_str).expect("Failed to write template to file");
        } else {
          panic!("Failed to get relative path");
        }
      }
    }
  }
}
