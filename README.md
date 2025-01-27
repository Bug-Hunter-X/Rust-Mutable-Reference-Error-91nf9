# Rust Mutable Reference Bug

This repository demonstrates a common error in Rust related to mutable references.  The `bug.rs` file contains code that attempts to create multiple mutable references to the same variable.  This will result in a compile-time error, as Rust's borrow checker prevents data races. The `bugSolution.rs` file contains a corrected version.