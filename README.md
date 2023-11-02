# CLI Tasks Manager 

## Build

```bash
cargo build --release
```

## Usage

### Create new Task
```bash
cargo run -- tasks create "hello world" -t "task description a bit longer" -s doing
```
or
```bash
./target/release/cli-todo-app tasks create "hello world" -t "task description a bit longer" -s doing
```

### List Tasks
```bash
cargo run -- tasks list
```
or
```bash
./target/release/cli-todo-app tasks list
```

### Delete Task
```bash
cargo run -- tasks delete 0
```
or
```bash
./target/release/cli-todo-app tasks delete 0
```

### Update Task
```bash
cargo run -- tasks update 0 -n "hello world" -t "task description a bit longer" -s doing
```
or
```bash
./target/release/cli-todo-app tasks update 0 -n "hello world" -t "task description a bit longer" -s doing
```

## TODO:
- add more tests
- add anyhow pretty errors
- load/save tasks from/to sqlite
- create a "project" -> "tasks" structure.

