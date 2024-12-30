#!/bin/bash

set -e

# GitHubリポジトリ情報
REPO="KAZUKI-SAKAMOTO/python-develop-generator"
BINARY_NAME="peg"
INSTALL_PATH="/usr/local/bin/$BINARY_NAME"

# デフォルトバージョン
VERSION=${1:-latest}
ASSET="linux-binary"

# インストール済みのバージョン確認
if command -v "$BINARY_NAME" &> /dev/null; then
  INSTALLED_VERSION=$($BINARY_NAME --version | awk '{print $NF}')
  echo "Installed version: $INSTALLED_VERSION"
else
  INSTALLED_VERSION=""
fi

# `jq`がインストールされているか確認
if ! command -v jq &> /dev/null; then
  echo "'jq' not found, using grep/awk as fallback."
  get_latest_version() {
    curl -s https://api.github.com/repos/$REPO/releases/latest | grep '"tag_name":' | awk -F'"' '{print $4}'
  }
else
  get_latest_version() {
    curl -s https://api.github.com/repos/$REPO/releases/latest | jq -r .tag_name
  }
fi

# バージョンが最新ならスキップ
if [ "$VERSION" == "latest" ]; then
  VERSION=$(get_latest_version)
fi

if [ "$INSTALLED_VERSION" == "$VERSION" ]; then
  echo "The latest version ($VERSION) is already installed."
  exit 0
fi

# 新しいバージョンをインストール
echo "Installing version $VERSION..."

# ダウンロードURL
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/$ASSET"

# バイナリのダウンロード
curl -L "$DOWNLOAD_URL" -o "$INSTALL_PATH"

# 実行権限を付与
chmod +x "$INSTALL_PATH"

echo "Installation complete! Version $VERSION is now installed."