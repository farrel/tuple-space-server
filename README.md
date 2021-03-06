# tuple-space-server - HTTP based Tuple Space server

## What is a Tuple Space

A Tuple Space is a method for coordinating data between different processes in an asynchronous manner. Processes write tuples of data to the Tuple Space and then read or remove data from the Tuple Space using a tuple as template to match against.

This server is based on the [tuple-space](https://github.com/farrel/tuple-space) crate. The corresponding [tuple-space-client](https://github.com/farrel/tuple-space-client) crate can be used to communicate with the server from a Rust program.

## Building and running the server

The source code of the tuple-space-server can be obtained from it's Github repository. The server is written in Rust, so building it requires a Rust compiler. Use the cargo build manager (installed as part of Rust) to build the executable.

    % git clone https://github.com/farrel/tuple-space-server.git
    % cd tuple-space-server
    % cargo build --release

The server can be started from the comand line, passing in the path of the configuration file.

    % ./target/release/tuple-space-server --config-file tuple_space_server.toml

## Configuration

The configuration of the server is set by a .toml file with the following values set

```toml
ip_address = "0.0.0.0"
port = 8000
queue_size = 100
```

## License (3-Clause BSD License)

Copyright 2021 Farrel Lifson

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
