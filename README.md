# RustAsyncBook's Project

This repository contains some example codes and the final project of the [async-book](https://rust-lang.github.io/async-book/), which I've implemented for learning purpose. The presented codes are very simple and only lead to a basic familiarity with the concepts of async in the Rust programming language.

## Final Project
The final project of async-book is very simple async implementation of the single threaded server example of the [Rust book](https://doc.rust-lang.org/book/).

## Repository Structure

- **src folder**
    - **html_codes foler** _(Contains simple html codes to use as server's response)_
        - **hello.html** _(to use as success response)_
        - **404.html** _(to use as unsuccess response)_
    - **executor_util** _(a module contains a sample of a very simple future executor)_
    - **rimer_future** _(a module contains a sample of a very simple time future)_
    - **handle_connection_mod** _(a module contains a simple async handle_connection function (a simple listener for a server) wich use in final project example and it's unit test)_
    - **mock_tcp_stream** _(a simple structure which implements Read, Write, and Unpin traits to use in unit test)_
    - **main** _(an async main function)_
    - **lib** _(a normal lib file to handle modules)_
