# extension_methods
An Example of Extention Methods adding json formatting to a clean library

This project is a workspace containing two libraries and a binary project. 

The libraries are: 

* `some_library` which contains an enum `Shape`. This library has no external dependencies. 
* `some_extention` contains a trait `JSON` which adds an Extention Method `to_json` onto `Shape`.

The extention library has to wrap `Shape` inside of `Shaper` using the newtype pattern to get around the Orphan rules. 
