# wasi-networking

This is a rough sketch of low-level networking API in WASI. It is
designed to resemble the POSIX networking API as much as possible, but
also aiming to:

- ensure capability-based security
- allow connections/streams to be dynamically created by a Wasm module itself
- support TLS and proxies transparently
- support multiplexing natively for protocols like QUIC

The rendered documentation is [here](docs.md#-networking).

## API usage

Below are the typical flows of the API calls.

For a server:

1. Create `control_fd` and bind it to a listening address: `open(protocol_fd, "[::1]:8888", socket_stream, ...) -> control_fd`
2. Listen on `control_fd`: `listen(control_fd, 0) -> errno`
3. possible polling on `control_fd`
4. Accept an incoming connection: `accept(control_fd) -> connection_fd`
5. Accept a stream: `stream_accept(connection_fd) -> stream_fd`
6. read/write on `stream_fd`

and for the client:

1. Create `control_fd` and bind it to an address (any): `open(protocol_fd, "[::]:0", socket_stream, ...) -> control_fd`
2. Connect to a remote peer through `control_fd`: `connect(control_fd, "[::1]:8888", ...) -> connection_fd`
3. Create a stream: `stream_create(connection_fd) -> stream_fd`
5. read/write on `stream_fd`

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

While some of the API functions (`listen`, `accept`, and `connect`;
`bind` is merged into `open`) are borrowed from [Berkeley sockets] and
POSIX, the multiplexing model is conceptually similar to
[STREAMS]. [CloudABI]'s socket API is also relevant to
capability-based security, while it imposes the host to statically
provide the pre-opened connections with the guest.

[Berkeley sockets]: https://en.wikipedia.org/wiki/Berkeley_sockets
[STREAMS]: https://en.wikipedia.org/wiki/STREAMS
[CloudABI]: https://github.com/NuxiNL/cloudabi#capability-based-security
