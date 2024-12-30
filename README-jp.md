# Python 開発環境ジェネレーター

## Overview

このプロジェクトは、Pythonの開発環境をセットアップするためのジェネレーターを提供します。生成されるプロジェクトには、リントツール（[flake8](https://pypi.org/project/flake8/)）、フォーマッター（[pep8](https://peps.python.org/pep-0008/)）、静的型チェッカー（[mypy](https://mypy-lang.org/)）が事前に設定されています。

## Features

- [uv](https://docs.astral.sh/uv/)を使用したパッケージ管理。
- Docker環境とローカル環境の選択肢を提供。
- ローカル環境を選択した場合、自動でuvをインストール。

## Installation

次のコマンドを実行してください：

```bash
curl -fsSL https://raw.githubusercontent.com/KAZUKI-SAKAMOTO/python-development-env-generator/master/install.sh | /bin/bash
```

## Usage

インストール後、以下のコマンドを実行することで、新しいPython開発環境をコマンド実行ディレクトリに作成できます：

```sh
$ peg
Python Project Name?: Sample
Which Python version do you use?: 3.12
Select Development Environment. If you select Docker, a .devcontainer file will be created.
Generate file: "Sample/.vscode/settings.json"
Generate file: "Sample/.python-version"
Generate file: "Sample/.mypy.ini"
Generate file: "Sample/.flake8"
Generate file: "Sample/.gitignore"
Generate file: "Sample/pyproject.toml"
Generate file: "Sample/src/hello.py"
Generate file: "Sample/.pep8"
Generate file: "Sample/cspell.json"
Generate file: "Sample/README.md"
Generate file: "Sample/.devcontainer/docker-compose.yml"
Generate file: "Sample/Docker/Dockerfile"
Generate file: "Sample/.devcontainer/devcontainer.json"
```

## License

このプロジェクトはMITライセンスの下で提供されています。