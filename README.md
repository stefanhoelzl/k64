[![Build Status](https://dev.azure.com/stefanhoelzl/k64/_apis/build/status/stefanhoelzl.k64?branchName=master)](https://dev.azure.com/stefanhoelzl/k64/_build/latest?definitionId=1&branchName=master)
[![Crates.io](https://img.shields.io/crates/v/k64.svg)](https://crates.io/crates/k64)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

# k64
Peripheral access API for Kinetis K64 microcontroller

## Getting Started
check out the following ressources to get started with embedded in Rust:
* [Embedded Rust documentation](https://docs.rust-embedded.org)
* [Embedded in Rust](http://blog.japaric.io)

Examples can be found [here](examples).

## Linker File
A [linker file](memory.x) can be found in the repository.
The smaller `SRAM_L` section (`64K`) is used for the stack 
and the larger `SRAM_U` section (`192K`) is used for the ram.

**Attention:** An un-aligned access across both sections can result in a hard fault!

## Development
The following `make` commands are available:
* `setup`: installs rust targets
* `publish`: publishes to [crates.io](http://www.crates.io)
* `examples`: build all examples
