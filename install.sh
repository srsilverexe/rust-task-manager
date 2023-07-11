#!/usr/bin/env bash

cargo build --release
sudo mv target/release/task_manager /bin/rust_task_manager
echo "task manager instaled!!"
echo 'run "rust_task_manager" to open the aplication'
