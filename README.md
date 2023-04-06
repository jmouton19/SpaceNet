# SpaceNet
A rust library for distributed virtual environments using spatial partitioning.
### This name sucks...

# Instructions
Tested on Fedora 37 Workstation and Zorin OS 16.2.
### Install rust
https://www.rust-lang.org/tools/install
```console
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Prereq system libraries (Ubuntu/Debian)
libfontconfig - Needed for plotters drawing.\
build-essential - GCC
```console
sudo apt-get install libfontconfig libfontconfig1-dev
sudo apt install build-essential  
```
### Running examples
#### Compiling examples
```console
cargo build --examples
```

#### Running boot node
```console
cd target/debug/examples
./example_boot
```

#### Running node
```console
cd target/debug/examples
./example_node
```

### Running tests
```console
cargo test
```

* Output pngs saved in SpaceNet/images. Create this folder manually :)
* To remove node from overlay enter q into its terminal session.

To test multiple nodes simultaneously use grouped terminal sessions. 
This can be done in Terminator.  
```console
sudo apt-get install terminator
```




