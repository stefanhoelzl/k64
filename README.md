[![Crates.io](https://img.shields.io/crates/v/k64.svg)](https://crates.io/crates/k64)

# k64
Peripheral access API for Kinetis K64 microcontrollers

# memory.x
A memory.x file can be found on the repository, but it only uses one section of the SRAM memory, feel free to change the script to support both sections.
**Attention:** An un-aligned access across both sections can result in a hard fault!

# Issues
Due to [svd2rust Issue 16](https://github.com/japaric/svd2rust/issues/16) there are some registers missing.
see [warnings](WARNINGS.md)

# Development
The following `make` commands are available:
* `setup`: installs tools
* `generate`: generates new sources from svd file
* `package`: creates a local package
* `publish`: publishes to [crates.io](http://www.crates.io)
