# AtRoot

**AtRoot** is a lightweight command-line tool written in Rust that allows you to 
define and execute custom commands based on a project-specific configuration file.

It automatically searches for an `atroot` file in the current working directory and its 
parent directories, stopping at the closest one it finds.

## How it works

AtRoot uses a file named `atroot`, when parsed onto lowercase, to define commands.

Each line consists of an identifier, followed by `==`, and the command that should be executed.

### Configuration

Create a file named `atroot` in your project root:

```text
run == cargo run

install == cargo install --path .

build == cargo build

run and build == cargo run && cargo build
```

The syntax is:

```text
identifier == command
```

- **Identifier:** The name used to invoke the command through AtRoot.
- **Command:** The command AtRoot should execute when the corresponding identifier is invoked.
### Usage

Once your configuration file is ready, you can execute its commands using `atroot`:

```bash
atroot run
```

This executes:

```bash
cargo run
```

```bash
atroot run and build
```

This executes:

```bash
cargo run && cargo build
```
AtRoot finds the corresponding identifier in the nearest `atroot` configuration file and 
executes its associated command.

## Configuration discovery

AtRoot searches for the `atroot` file starting from the current working directory.

If it cannot find one, it moves to the parent directory and repeats the process until it finds 
a matching file.

This allows you to define commands at the project level without having to specify the 
configuration file's location manually.

## Features

- **Lightweight:** A small command-line utility written in Rust.
- **Project-aware:** Automatically discovers the nearest `atroot` configuration file.
- **Custom commands:** Associate identifiers with commands of your choice.
- **Minimal configuration:** A simple, readable syntax.

