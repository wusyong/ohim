#!/bin/sh

set -ex

sudo apt-get update 
sudo apt-get install build-essential musl-dev musl-tools python3-pygments python3-markupsafe -y

rustup target add wasm32-unknown-unknown wasm32-wasip1 wasm32-wasip2

mkdir -p $HOME/.local/bin
cd $HOME/.local/bin

if [ X"${GH_TOKEN}" = X"" ];then 
    read -p "Paste here your Github personal access token: " GH_TOKEN
    export GH_TOKEN
fi

## Install Conventional Commits - Cocogitto (`cog`)
CURRENT_REPO="cocogitto/cocogitto"
CURRENT_VERSION=$(gh --repo $CURRENT_REPO release view --json tagName --jq .tagName)
DOWNLOADED_FILE=$(gh --repo $CURRENT_REPO release view --json assets --jq '.assets[] | select(.name | contains("x86_64") and contains("linux") and contains("musl")) .name')
gh --repo $CURRENT_REPO --pattern "$DOWNLOADED_FILE" release download $CURRENT_VERSION
tar -zxvf $DOWNLOADED_FILE x86_64-unknown-linux-musl
cp -v x86_64-unknown-linux-musl/cog ./
rm -rfv $DOWNLOADED_FILE x86_64-unknown-linux-musl

## Install wasmtime (`wasmtime`)
CURRENT_REPO="bytecodealliance/wasmtime"
CURRENT_VERSION=$(gh --repo $CURRENT_REPO release view --json tagName --jq .tagName)
DOWNLOADED_FILE=$(gh --repo $CURRENT_REPO release view --json assets --jq '.assets[] | select(.name | contains("x86_64") and contains("linux") and (contains("c-api") | not)  and endswith(".tar.xz")) .name')
DOWNLOADED_FILE_WITHOUT_EXTENSION=$(basename $DOWNLOADED_FILE .tar.xz)
gh --repo $CURRENT_REPO --pattern "$DOWNLOADED_FILE" release download $CURRENT_VERSION
tar -xvf $DOWNLOADED_FILE $DOWNLOADED_FILE_WITHOUT_EXTENSION/wasmtime $DOWNLOADED_FILE_WITHOUT_EXTENSION/wasmtime-min
mv -v ${DOWNLOADED_FILE_WITHOUT_EXTENSION}/* ./
rm -rfv $DOWNLOADED_FILE $DOWNLOADED_FILE_WITHOUT_EXTENSION

## Install wasm-pack (`wasm-pack`)
CURRENT_REPO="rustwasm/wasm-pack"
CURRENT_VERSION=$(gh --repo $CURRENT_REPO release view --json tagName --jq .tagName)
DOWNLOADED_FILE=$(gh --repo $CURRENT_REPO release view --json assets --jq '.assets[] | select(.name | contains("x86_64") and contains("linux") and contains("musl") and endswith(".tar.gz")) .name')
TMP_FOLDER=$(basename $DOWNLOADED_FILE .tar.gz)
gh --repo $CURRENT_REPO --pattern "$DOWNLOADED_FILE" release download $CURRENT_VERSION
tar -xvf $DOWNLOADED_FILE
mv ${TMP_FOLDER}/wasm-pack ./
rm -rf $DOWNLOADED_FILE $TMP_FOLDER

## Install wasm-tools (`wasm-tools`)
CURRENT_REPO="bytecodealliance/wasm-tools"
CURRENT_VERSION=$(gh --repo $CURRENT_REPO release view --json tagName --jq .tagName)
DOWNLOADED_FILE=$(gh --repo $CURRENT_REPO release view --json assets --jq '.assets[] | select(.name | contains("x86_64") and contains("linux") and endswith(".tar.gz")) .name')
TMP_FOLDER=$(basename $DOWNLOADED_FILE .tar.gz)
gh --repo $CURRENT_REPO --pattern "$DOWNLOADED_FILE" release download $CURRENT_VERSION
tar -xvf $DOWNLOADED_FILE
mv ${TMP_FOLDER}/wasm-tools ./
rm -rf $DOWNLOADED_FILE $TMP_FOLDER

## Install wasm-bindgen (`wasm-bindgen`)
CURRENT_REPO="rustwasm/wasm-bindgen"
CURRENT_VERSION=$(gh --repo $CURRENT_REPO release view --json tagName --jq .tagName)
DOWNLOADED_FILE=$(gh --repo $CURRENT_REPO release view --json assets --jq '.assets[] | select(.name | contains("x86_64") and contains("linux") and contains("musl") and endswith(".tar.gz")) .name')
TMP_FOLDER=$(basename $DOWNLOADED_FILE .tar.gz)
gh --repo $CURRENT_REPO --pattern "$DOWNLOADED_FILE" release download $CURRENT_VERSION
tar -xvf $DOWNLOADED_FILE
mv ${TMP_FOLDER}/wasm-bindgen ./
mv ${TMP_FOLDER}/wasm-bindgen-test-runner ./
mv ${TMP_FOLDER}/wasm2es6js ./
rm -rf $DOWNLOADED_FILE $TMP_FOLDER

## Install wit-bindgen (`wit-bindgen`)
CURRENT_REPO="bytecodealliance/wit-bindgen"
CURRENT_VERSION=$(gh --repo $CURRENT_REPO release view --json tagName --jq .tagName)
DOWNLOADED_FILE=$(gh --repo $CURRENT_REPO release view --json assets --jq '.assets[] | select(.name | contains("x86_64") and contains("linux") and endswith(".tar.gz")) .name')
TMP_FOLDER=$(basename $DOWNLOADED_FILE .tar.gz)
gh --repo $CURRENT_REPO --pattern "$DOWNLOADED_FILE" release download $CURRENT_VERSION
tar -xvf $DOWNLOADED_FILE
mv ${TMP_FOLDER}/wit-bindgen ./
rm -rf $DOWNLOADED_FILE $TMP_FOLDER

CURRENT_REPO="bytecodealliance/wac"
CURRENT_VERSION=$(gh --repo $CURRENT_REPO release view --json tagName --jq .tagName)
DOWNLOADED_FILE=$(gh --repo $CURRENT_REPO release view --json assets --jq '.assets[] | select(.name | contains("x86_64") and contains("linux") and endswith("musl")) .name')
TMP_FOLDER=$(basename $DOWNLOADED_FILE .tar.gz)
gh --repo $CURRENT_REPO --pattern "$DOWNLOADED_FILE" release download $CURRENT_VERSION
chmod +x $DOWNLOADED_FILE
mv ${DOWNLOADED_FILE} wac

# Install only the latest cargo-component compiled version
CURRENT_REPO="JADSN1894/carginstall"
CURRENT_VERSION=$(gh --repo "$CURRENT_REPO" release view --json tagName --jq .tagName)
# Download only the file named 'cago-component'
gh --repo "$CURRENT_REPO" release download "$CURRENT_VERSION" --pattern "cargo-component"
chmod +x cargo-component
