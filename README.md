# Rusty-SLM

This is a program for controlling SLMs with rust and python. The server is implemented in rust with grpc, allowing for control from any programming language that can connect to a web endpoint.

## Binaries

There's binaries in the releases section to download, which should run on windows and linux without extra stuff.

## Server Compilation
### Windows
For compiling on windows:
1) Install rust (using [rustup](https://rustup.rs/). You'll probably need to install the msvc c++ build-tools as well)
2) Install the [Vulkan SDK](https://www.lunarg.com/vulkan-sdk/)
3) Download this repository, and go into the root directory
4) Run `cargo build --release`
5) The executable should be created as: `target/release/rusty-slm-server.exe`
6) Run this executable from the terminal with a port number like: `rusty-slm-server.exe 9000`

### Linux
For compiling on linux:
Follow the same instructions as windows, but remove the `.exe` from the file name.

### Mac
Probably the same as linux?

## Python Client Installation

This repo also contains a simple python client for communicating with the server. You can install this through pip with:

    pip install git+https://github.com/maxastyler/rusty-slm

### Client Usage

Import the `rusty_slm` package into python, and create an `SLMController` object.
```python
import numpy as np
from rusty_slm import SLMController

slm = SLMController(SERVER_PORT_NUMBER)
slm.set_screen(0)
slm.set_image(np.random.randint(0, 256, (1920, 1080), dtype=np.uint8))
```

You can use `set_screen` to change which monitor the image is displayed on, and `set_image` to set the image. 
`set_image` takes a `uint8` numpy array as an argument, which can have a shape `(W, H)` for greyscale images, or `(W, H, 3)` for colour images.

## Usage from another language

The server communicates with clients using the GRPC system, which has implementations in loads of different languages.
You can find the protobuf definition in `protos/slm.proto`, which is pretty simple.

## Todo

- Implement a function to return a full list of monitors, and their properties
- Option to set a monitor by name, rather than number
