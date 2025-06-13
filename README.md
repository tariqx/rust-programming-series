# rust-programming-series


# threads.md
Tutorial on threads and concurrency in Rust.

# vector-data-structure.md
Tutorial on vector data structure in Rust.

# echocli 
A simple version of the echo command-line utility

It takes a string as input and prints it back to the user, followed by a newline character
Example usage:
echocli.exe -e "Happy learning...!\nThis is Rust."

# cat cli
A simple implementation of the Unix cat command in Rust.
It takes a file paths as input and prints the contents of the file to the output.
Example usage:
cat.exe [options] [filenames]

options: 
-n : show line numbers in the output
-h : display help
-v : show nonprinting characters in the output
-s : Removes all the blank spaces from the output

# Axum + SQLx CRUD REST API 
Database-driven CRUD (Create, Read, Update, Delete) backend API using the Rust ecosystem. Covers building simple Todo list that allows to create a new task, get the list of all tasks, look up a task by id, delete a task by its id, and finally update an existing task.

Tech stack used for building this project: 
- postgres: Open-source database
- axum: API framework 
- SQLx: Very fast compile time db library. 
- tokio: Async event-driven library  
- serde: Library for serializing and deserializing data structures

# Smart Contract + Web3 
This project demonstrates how to interact with a smart contract using Rust. Contract was deployed to sepolia testnet using solidity, see ***ethereum-smart-contract-tut*** repo.
