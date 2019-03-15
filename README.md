[![Crates.io](https://img.shields.io/crates/v/k64.svg)](https://crates.io/crates/k64)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

# k64
Peripheral access API for Kinetis K64 microcontrollers

## Linker File
A [linker file](memory.x) can be found in the repository.
The smaller `SRAM_L` section (`64K`) is used for the stack 
and the larger `SRAM_U` section (`192K`) is used for the ram.

**Attention:** An un-aligned access across both sections can result in a hard fault!

## Issues
Due to [svd2rust Issue 16](https://github.com/japaric/svd2rust/issues/16) there are some registers missing.
see [warnings](WARNINGS.md)

## Development
The following `make` commands are available:
* `setup`: installs tools
* `generate`: generates new sources from svd file
* `package`: creates a local package
* `publish`: publishes to [crates.io](http://www.crates.io)
