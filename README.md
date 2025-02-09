# Multiple Mutable Borrows in Rust

This repository demonstrates a common error in Rust: attempting to have multiple mutable borrows of the same variable.  Rust's ownership and borrowing system prevents data races by enforcing strict rules about how variables can be borrowed.  This example shows how to reproduce the error and how to fix it using techniques like cloning or refactoring.