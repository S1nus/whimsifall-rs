# whimsifall-rs

## Overview
Whimsifall is a based rollup on Celestia, written in Rust.

![whimsifall][flowchart.png "Flowchart"]
# Set up development environment

This page will go over setting up your development environment to run Celestia software. This environment can be used for development, building binaries, and running nodes.

## Install dependencies

1. If you are on Ubuntu, first update and upgrade your OS:
    
    ```bash
    sudo apt update && sudo apt upgrade -y
    
    ```
    
2. Install essential packages that are necessary to execute many tasks like downloading files, compiling, and monitoring the node:
    
    ```bash
    sudo apt install curl tar wget clang pkg-config libssl-dev jq build-essential \\
    git make ncdu -y
    
    ```
    

## Install Golang

celestia-node is written in Golang so we must install Golang to build and run our node.

1. We are using Mocha version for the desired network:
    
    ```bash
    ver="1.21.1"
    
    ```
    
2. Download and install Golang:
    
    ```bash
    cd $HOME
    wget "<https://golang.org/dl/go$ver.linux-amd64.tar.gz>"
    sudo rm -rf /usr/local/go
    sudo tar -C /usr/local -xzf "go$ver.linux-amd64.tar.gz"
    rm "go$ver.linux-amd64.tar.gz"
    
    ```
    
3. Add your `/usr/local/go/bin` directory to your `$PATH` if you have not already:
    
    ```bash
    echo "export PATH=$PATH:/usr/local/go/bin:$HOME/go/bin" >> $HOME/.bash_profile
    source $HOME/.bash_profile
    ```
    
4. To verify that the correct version of Go was installed correctly run:
    
    ```bash
    go version
    ```
    

The output will show the version installed.


# Install Rust

## Overview

Rust is a systems programming language that is known for its performance, reliability, and productivity. Follow the steps below to install Rust on your system.

## Supported Platforms

Rust supports a variety of platforms, including Windows, macOS, and Linux. Visit the [official Rust website](https://www.rust-lang.org/tools/install) for the most up-to-date information.

## Installation Steps

**1. Open a Terminal or Command Prompt**

**Windows**

Open the Command Prompt or PowerShell with administrator privileges.

**macOS / Linux**

Open a terminal.

**2. Download and Run the Installer**

Using `curl` (on macOS / Linux)

```bash
curl --proto '=https' --tlsv1.2 -sSf <https://sh.rustup.rs> | sh
```
