# Presto Parser

This is a parser for the NER annotated _Presto_ corpus distributed [here](https://github.com/e-ditiones/LEM17). For the moment the parser function **is NOT implemented**.

## Usage

To cut a presto file into subfiles do:

```bash
presto-parser cut [path/to/presto_file] [path/to/folder]
```

To correct a presto file do:

```bash
presto-parser correct [path/to/presto_file]
```

To if there are many mistakes try with

```bash
presto-parser correct [path/to/presto_file] > error_file.txt
```

## Installation

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Complete istallation guide [here](https://www.rust-lang.org/tools/install).

### Clone the repo

```bash
git clone
```

### cd Into the the folder

```bash
cd presto-parser
```

### Install the package

```bash
cargo install --path ./
```
