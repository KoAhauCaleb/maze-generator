# Overview

This is a CLI application written in Rust to create mazes and export them as an SVG file.

I have started to hear a lot more about Rust recently. Because of that I decided to do this project to introduce myself to it.

Coming from Python, Rust was a bit complex, and I had many troubles at first. The majority of this project was reading documentation to figure out why a single line of code wasn't working. My experience with Java did lessen the struggle fortunately.

A video of my code running and an overview of how it works can be found at:

[Rust Maze Generator Project](https://youtu.be/fkDYrdmyr1Y)

# Development Environment

I installed Rust and Cargo using rustup, an official tool to manage rust installations. Cargo was useful with testing by compiling and running my application with a single command. Rust also requires Visual Studio C++ Build Tools which are used for compiling.

The equivalent to modules/libraries in Rust are crates. I used the SVG crate to export the maze as an SVG file. 

# Useful Websites

* [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)
* [Maze Generation Algorithm](https://en.wikipedia.org/wiki/Maze_generation_algorithm)
* [SVG Crate Docs](https://crates.io/crates/svg)

# Future Work

* Find best spot to be the finish of the maze maze.
* Keep track of maze solution.
* Turn it into a web application to get experience with WebAssembly