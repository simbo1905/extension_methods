# Rust Extension Methods

An example of using Extention Methods to pimp my library with a json generating method. The idea is that you have a clean create with the library code and a seperate create with the extention methods. This avoids having 3rd party marcos like `serde` as a dependency on your clean library code. 

This project is a workspace containing two libraries and a binary project. 

The libraries are: 

* `some_library` which contains an enum `Shape`. This library has no external dependencies. 
* `some_extention` contains a trait `JSON` which adds an Extention Method `to_json` onto `Shape` by wrapping it in `Shaper` to get around the Orphan rules. The `Shaper` class uses the serde remote derive technique [Deriving De/Serialize for type in a different crate](https://serde.rs/remote-derive.html)


