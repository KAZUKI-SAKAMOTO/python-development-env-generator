# Python Develop Environment Generator

## Overview

This project provides a generator for setting up a Python development environment. It automates the creation of virtual environments, installation of dependencies, and configuration of development tools.

## Features

- Automatic creation of virtual environments
- Dependency management with `requirements.txt`
- Configuration of development tools like linters and formatters
- Easy setup with a single command

## Installation

TODO: Add installation instructions

## Usage

After installation, you can use the generator to create a new Python development environment by running:
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