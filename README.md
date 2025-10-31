# study-rust-async-cycle-iterator

This repository study for async cycle iterator in Rust.

## Question

How to create an async cycle iterator in Rust that can repeatedly iterate over a collection of items asynchronously?

## Answer

To create an async cycle iterator in Rust, you can use the `futures` crate, which provides utilities for working with asynchronous programming. Below is an example of how to implement an async cycle iterator that repeatedly iterates over a collection of items asynchronously.

## Feature

- `select!` always execute all branches even if one branch is ready.
  - be careful each branch task. e.g. insert db
