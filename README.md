This repository is a Rust crate defining the schemata for Wordweaver.

If you are making a client for Wordweaver in Rust, you can just use this crate. Otherwise, you'll have to build your own. Please make sure it precisely conforms to the data structures encoded here.

In the future, this crate will provide a trait `Cipher` which each version will implement to allow for our cryptography to be hardened over time without breaking compatibility.

This library is compatible with `no_std` 🎉! It does need a heap allocator, though. If all goes well you might be able to run wordweaver on bare-metal 😎
