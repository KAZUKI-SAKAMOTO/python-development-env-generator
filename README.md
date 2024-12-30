# Python Development Environment Generator

## Overview

This project provides a generator for setting up a Python development environment. The generated project comes preconfigured with a linter ([flake8](https://pypi.org/project/flake8/)), a formatter ([pep8](https://peps.python.org/pep-0008/)), and a static type checker ([mypy](https://mypy-lang.org/)).

## Features

- Package management using [uv](https://docs.astral.sh/uv/).
- Ability to choose between Docker and local environments for development.
- Automatic installation of uv if a local environment is selected.

## Installation

Run the following command to install:

```bash
curl -fsSL https://raw.githubusercontent.com/KAZUKI-SAKAMOTO/python-development-env-generator/master/install.sh | /bin/bash
```

## Usage

After installation, you can create a new Python development environment in the command execution directory by running:
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

This project is licensed under the MIT License.