# wasi-networking

This is a rough sketch of low-level networking API in WASI. It is
designed to resemble the POSIX networking API as much as possible, but
also aiming at:

- ensuring capability-based security
- allowing connections/streams to be dynamically created by a Wasm module
- native support for multiplexing for protocols like QUIC

The rendered documentation is [here](docs.md#-networking).

## API usage

Below are the typical flows of the API calls.

For a server:

1. Create a `control_fd`: `open(protocol_fd, "[::1]:8888", socket_stream, ...)`
2. Listen: `listen(control_fd, 0)`
3. [possible polling on `control_fd`]
4. Accept a connection: `accept(control_fd)`
5. Accept a stream: `stream_accept(connection_fd)`
6. [read/write on `stream_fd`]

and for the client:

1. Create a `control_fd`: `open(protocol_fd, "[::]:0", socket_stream, ...)`
2. Connect: `connect(control_fd, "[::1]:8888", ...)`
3. Create a stream: `stream_create(connection_fd)`
5. [read/write on `stream_fd`]

A protocol object (`protocol_fd`), which the host provides with the
guest, serves as a factory of connections/streams as well as a sandbox
in capability-based security. Protocol objects represent the actual
protocol implementation, such as IPv4, IPv6, Unix domain socket, or
QUIC, and handle any transport specific parameters, such as address
resolution and TLS certificates setup.

For multiplexing, the program can create any number of streams with
`stream_create` and `stream_accept` on a single connection (`connetion_fd`), if the
underlying protocol object supports it.

## Prior art

The multiplexing model is conceptually similar to
[STREAMS]. [CloudABI]'s socket API is also relevant to the
capability-oriented API, while it pre-opens the connections
themselves.

[CloudABI]: https://github.com/NuxiNL/cloudabi#capability-based-security
[STREAMS]: https://en.wikipedia.org/wiki/STREAMS
