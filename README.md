# File Watcher Basic Experiment
This is a simple Rust program that watches a specified directory for file system events such as file creation, modification, and deletion.

## Overview
The program uses the notify crate to watch a directory and handle file system events. It sets up a watcher on a specified directory and then enters a loop where it sleeps for one second at a time, checking each time if a Ctrl+C interrupt signal has been received. If such a signal is received, the program exits.

When a file system event occurs in the watched directory, the program handles the event by printing the paths of the affected files and the kind of event (create, modify, or remove).

## Usage
To run the program, use the following command:

```bash
cargo run
```
By default, the program watches the C:\\WatchDir directory. If this directory does not exist, the program will print an error message and exit. You can change the directory by modifying the path variable in the main function.

## Dependencies
This program depends on the following crates:
- ctrlc version 3.4.4: for handling Ctrl+C interrupt signals.
- notify version 6.1.1: for watching directories and handling file system events.

## Note
This program is an experiment and is not intended for production use. It may not handle all edge cases and error conditions.

## License
This project is licensed under the MIT License - see the LICENSE.md file for details.