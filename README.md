
```
- Operating Systems Coursework
Students:
- Juan David Valencia Torres, jdvalencit@eafit.edu.co

Professor:
- Juan Guillermo Lalinde, jlalinde@eafit.edu.co
```
# Introduction

The following project is the implementation of a task manager in Rust programming language. 

Rust is a compiled systems programming language that provides fine control over memory management and shares the principle of zero-cost abstraction. With the concept of ownership, moves, and borrows, Rust ensures memory safety and concurrency, which makes it a robust option for implementing systems-level software such as a task manager. The task manager aims to provide a user-friendly interface to manage running processes on a Linux operating system. The implementation leverages the procfs library to obtain information about the running processes and provides functionality such as viewing process details, terminating processes, and sorting processes by various criteria.

The project's dependencies include procfs, a Rust library that provides an interface to read the /proc file system in Linux, and tokio, a library for writing asynchronous code with Rust. The task manager is developed to work on Linux operating systems and does not support other platforms.

The system has been tested on Ubuntu 22.04.

#  Implementation

The project offers additional functionality beyond basic process management. It includes features such as the ability to change process priority, a user interface inspired by the designs of HTop and Bottom, and displays information about the system resources being used.

The task manager interacts with the procfs library to obtain system resource information such as CPU usage, memory usage, disk usage, and network usage. It uses this information to populate the user interface with real-time statistics that allow users to monitor their system's performance.

One of the unique features of the task manager is its ability to modify the priority of running processes. This functionality enables users to manage their system resources effectively by giving them greater control over the allocation of system resources. This can be done through the program's interface.

To create the user interface for the task manager, we used Rust's TUI (Terminal User Interface) library, which provides a set of widgets and event handlers that allow us to create interactive console-based applications. The TUI library also includes an event-driven model that allows us to react to user input and update the interface accordingly. The library abstracts the complexity of terminal I/O and makes it easy to handle input events and refresh the screen in real-time.

# Usage

### Clone the repository

First, you need to clone the repository from GitHub. To do this, you can open a terminal and run the following command:

```
git clone https://github.com/jdvalencit/system-monitor.git
```

### Running the program

To run the program, navigate to the `system-monitor` directory in the terminal and run by using the following commands:

```
cd system-monitor
cargo run
```

### Building the program for production

To build the program for production, navigate to the `system-monitor` directory in the terminal and run by using the following commands:

```
cd system-monitor
cargo build --release
```

This will compile the program with optimizations, and the resulting binary will be located in the `target/release/` directory. You can then copy this binary to your desired location for deployment.

#  Conclusions

Overall, this task manager implementation has achieved its main objectives. The program provides users with a powerful system management tool that allows them to monitor and modify various aspects of the system's processes, such as their priority, CPU usage, memory usage, disk usage, and network activity.

The use of Rust's TUI library has allowed us to create an intuitive and visually appealing user interface, which makes it easy for users to interact with the system management tool.

There are some limitations that could be addressed in future implementations. The main goal is to introduce more flexible balancing policies that would allow users to specify the priority for each process or other quantitative criteria for determining the queue ordering (e.g. Ordering by last hour network usage).

#  References
- [1] [procfs - crates.io: Rust Package Registry](https://crates.io/crates/procfs)
- [2] [sysinfo - crates.io: Rust Package Registry](https://crates.io/crates/sysinfo)
- [3] [tui - crates.io: Rust Package Registry](https://crates.io/crates/tui)
- [4] [log - crates.io: Rust Package Registry](https://crates.io/crates/log)
- [5] [tokio - crates.io: Rust Package Registry](https://crates.io/crates/tokio)
- [6] [tui-logger - crates.io: Rust Package Registry](https://crates.io/crates/tui-logger)
- [7] [Rust Programming Language (rust-lang.org)](https://www.rust-lang.org/)
