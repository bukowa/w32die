# w32die

A simple binary that wraps any executable and runs it in a 
Windows Job Object to ensure it (and all its children) are 
terminated when the parent process dies.

## Installation
```shell
cargo install w32die
```

## Usage
```shell
w32die [OPTIONS] -- <command> [ARGS]
```

## Example
```shell
$ w32die -- echo "Hello, World!"
```
