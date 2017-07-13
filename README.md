# Rust Extension Methods

An example of using Extention Methods to pimp my library with a json generating method. 

This project is a workspace containing two libraries and a binary project. 

The libraries are: 

* `some_library` which contains an enum `Shape`. This library has no external dependencies. 
* `some_extention` contains a trait `JSON` which adds an Extention Method `to_json` onto `Shape` by wrapping it in `Shaper` to get around the Orphan rules. 

