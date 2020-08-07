# Types
## <a href="#size" name="size"></a> `size`: `usize`
An array size.

Note: This is similar to `size_t` in POSIX.

Size: 4

Alignment: 4

## <a href="#filesize" name="filesize"></a> `filesize`: `u64`
Non-negative file size or length of a region within a file.

Size: 8

Alignment: 8

## <a href="#timestamp" name="timestamp"></a> `timestamp`: `u64`
Timestamp in nanoseconds.

Size: 8

Alignment: 8

## <a href="#clockid" name="clockid"></a> `clockid`: Enum(`u32`)
Identifiers for clocks.

Size: 4

Alignment: 4

### Variants
- <a href="#clockid.realtime" name="clockid.realtime"></a> `realtime`
The clock measuring real time. Time value zero corresponds with
1970-01-01T00:00:00Z.

- <a href="#clockid.monotonic" name="clockid.monotonic"></a> `monotonic`
The store-wide monotonic clock, which is defined as a clock measuring
real time, whose value cannot be adjusted and which cannot have negative
clock jumps. The epoch of this clock is undefined. The absolute time
value of this clock therefore has no meaning.

## <a href="#errno" name="errno"></a> `errno`: Enum(`u16`)
Error codes returned by functions.
Not all of these error codes are returned by the functions provided by this
API; some are used in higher-level library layers, and others are provided
merely for alignment with POSIX.

Size: 2

Alignment: 2

### Variants
- <a href="#errno.success" name="errno.success"></a> `success`
No error occurred. System call completed successfully.

- <a href="#errno.2big" name="errno.2big"></a> `2big`
Argument list too long.

- <a href="#errno.access" name="errno.access"></a> `access`
Permission denied.

- <a href="#errno.addrinuse" name="errno.addrinuse"></a> `addrinuse`
Address in use.

- <a href="#errno.addrnotavail" name="errno.addrnotavail"></a> `addrnotavail`
Address not available.

- <a href="#errno.afnosupport" name="errno.afnosupport"></a> `afnosupport`
Address family not supported.

- <a href="#errno.again" name="errno.again"></a> `again`
Resource unavailable, or operation would block.

- <a href="#errno.already" name="errno.already"></a> `already`
Connection already in progress.

- <a href="#errno.badf" name="errno.badf"></a> `badf`
Bad file descriptor.

- <a href="#errno.badmsg" name="errno.badmsg"></a> `badmsg`
Bad message.

- <a href="#errno.busy" name="errno.busy"></a> `busy`
Device or resource busy.

- <a href="#errno.canceled" name="errno.canceled"></a> `canceled`
Operation canceled.

- <a href="#errno.child" name="errno.child"></a> `child`
No child processes.

- <a href="#errno.connaborted" name="errno.connaborted"></a> `connaborted`
Connection aborted.

- <a href="#errno.connrefused" name="errno.connrefused"></a> `connrefused`
Connection refused.

- <a href="#errno.connreset" name="errno.connreset"></a> `connreset`
Connection reset.

- <a href="#errno.deadlk" name="errno.deadlk"></a> `deadlk`
Resource deadlock would occur.

- <a href="#errno.destaddrreq" name="errno.destaddrreq"></a> `destaddrreq`
Destination address required.

- <a href="#errno.dom" name="errno.dom"></a> `dom`
Mathematics argument out of domain of function.

- <a href="#errno.dquot" name="errno.dquot"></a> `dquot`
Reserved.

- <a href="#errno.exist" name="errno.exist"></a> `exist`
File exists.

- <a href="#errno.fault" name="errno.fault"></a> `fault`
Bad address.

- <a href="#errno.fbig" name="errno.fbig"></a> `fbig`
File too large.

- <a href="#errno.hostunreach" name="errno.hostunreach"></a> `hostunreach`
Host is unreachable.

- <a href="#errno.idrm" name="errno.idrm"></a> `idrm`
Identifier removed.

- <a href="#errno.ilseq" name="errno.ilseq"></a> `ilseq`
Illegal byte sequence.

- <a href="#errno.inprogress" name="errno.inprogress"></a> `inprogress`
Operation in progress.

- <a href="#errno.intr" name="errno.intr"></a> `intr`
Interrupted function.

- <a href="#errno.inval" name="errno.inval"></a> `inval`
Invalid argument.

- <a href="#errno.io" name="errno.io"></a> `io`
I/O error.

- <a href="#errno.isconn" name="errno.isconn"></a> `isconn`
Socket is connected.

- <a href="#errno.isdir" name="errno.isdir"></a> `isdir`
Is a directory.

- <a href="#errno.loop" name="errno.loop"></a> `loop`
Too many levels of symbolic links.

- <a href="#errno.mfile" name="errno.mfile"></a> `mfile`
File descriptor value too large.

- <a href="#errno.mlink" name="errno.mlink"></a> `mlink`
Too many links.

- <a href="#errno.msgsize" name="errno.msgsize"></a> `msgsize`
Message too large.

- <a href="#errno.multihop" name="errno.multihop"></a> `multihop`
Reserved.

- <a href="#errno.nametoolong" name="errno.nametoolong"></a> `nametoolong`
Filename too long.

- <a href="#errno.netdown" name="errno.netdown"></a> `netdown`
Network is down.

- <a href="#errno.netreset" name="errno.netreset"></a> `netreset`
Connection aborted by network.

- <a href="#errno.netunreach" name="errno.netunreach"></a> `netunreach`
Network unreachable.

- <a href="#errno.nfile" name="errno.nfile"></a> `nfile`
Too many files open in system.

- <a href="#errno.nobufs" name="errno.nobufs"></a> `nobufs`
No buffer space available.

- <a href="#errno.nodev" name="errno.nodev"></a> `nodev`
No such device.

- <a href="#errno.noent" name="errno.noent"></a> `noent`
No such file or directory.

- <a href="#errno.noexec" name="errno.noexec"></a> `noexec`
Executable file format error.

- <a href="#errno.nolck" name="errno.nolck"></a> `nolck`
No locks available.

- <a href="#errno.nolink" name="errno.nolink"></a> `nolink`
Reserved.

- <a href="#errno.nomem" name="errno.nomem"></a> `nomem`
Not enough space.

- <a href="#errno.nomsg" name="errno.nomsg"></a> `nomsg`
No message of the desired type.

- <a href="#errno.noprotoopt" name="errno.noprotoopt"></a> `noprotoopt`
Protocol not available.

- <a href="#errno.nospc" name="errno.nospc"></a> `nospc`
No space left on device.

- <a href="#errno.nosys" name="errno.nosys"></a> `nosys`
Function not supported.

- <a href="#errno.notconn" name="errno.notconn"></a> `notconn`
The socket is not connected.

- <a href="#errno.notdir" name="errno.notdir"></a> `notdir`
Not a directory or a symbolic link to a directory.

- <a href="#errno.notempty" name="errno.notempty"></a> `notempty`
Directory not empty.

- <a href="#errno.notrecoverable" name="errno.notrecoverable"></a> `notrecoverable`
State not recoverable.

- <a href="#errno.notsock" name="errno.notsock"></a> `notsock`
Not a socket.

- <a href="#errno.notsup" name="errno.notsup"></a> `notsup`
Not supported, or operation not supported on socket.

- <a href="#errno.notty" name="errno.notty"></a> `notty`
Inappropriate I/O control operation.

- <a href="#errno.nxio" name="errno.nxio"></a> `nxio`
No such device or address.

- <a href="#errno.overflow" name="errno.overflow"></a> `overflow`
Value too large to be stored in data type.

- <a href="#errno.ownerdead" name="errno.ownerdead"></a> `ownerdead`
Previous owner died.

- <a href="#errno.perm" name="errno.perm"></a> `perm`
Operation not permitted.

- <a href="#errno.pipe" name="errno.pipe"></a> `pipe`
Broken pipe.

- <a href="#errno.proto" name="errno.proto"></a> `proto`
Protocol error.

- <a href="#errno.protonosupport" name="errno.protonosupport"></a> `protonosupport`
Protocol not supported.

- <a href="#errno.prototype" name="errno.prototype"></a> `prototype`
Protocol wrong type for socket.

- <a href="#errno.range" name="errno.range"></a> `range`
Result too large.

- <a href="#errno.rofs" name="errno.rofs"></a> `rofs`
Read-only file system.

- <a href="#errno.spipe" name="errno.spipe"></a> `spipe`
Invalid seek.

- <a href="#errno.srch" name="errno.srch"></a> `srch`
No such process.

- <a href="#errno.stale" name="errno.stale"></a> `stale`
Reserved.

- <a href="#errno.timedout" name="errno.timedout"></a> `timedout`
Connection timed out.

- <a href="#errno.txtbsy" name="errno.txtbsy"></a> `txtbsy`
Text file busy.

- <a href="#errno.xdev" name="errno.xdev"></a> `xdev`
Cross-device link.

- <a href="#errno.notcapable" name="errno.notcapable"></a> `notcapable`
Extension: Capabilities insufficient.

## <a href="#rights" name="rights"></a> `rights`: Flags(`u64`)
File descriptor rights, determining which actions may be performed.

Size: 8

Alignment: 8

### Flags
- <a href="#rights.fd_datasync" name="rights.fd_datasync"></a> `fd_datasync`
The right to invoke `fd_datasync`.
If `path_open` is set, includes the right to invoke
`path_open` with [`fdflags::dsync`](#fdflags.dsync).

- <a href="#rights.fd_read" name="rights.fd_read"></a> `fd_read`
The right to invoke `fd_read` and `sock_recv`.
If [`rights::fd_seek`](#rights.fd_seek) is set, includes the right to invoke `fd_pread`.

- <a href="#rights.fd_seek" name="rights.fd_seek"></a> `fd_seek`
The right to invoke `fd_seek`. This flag implies [`rights::fd_tell`](#rights.fd_tell).

- <a href="#rights.fd_fdstat_set_flags" name="rights.fd_fdstat_set_flags"></a> `fd_fdstat_set_flags`
The right to invoke `fd_fdstat_set_flags`.

- <a href="#rights.fd_sync" name="rights.fd_sync"></a> `fd_sync`
The right to invoke `fd_sync`.
If `path_open` is set, includes the right to invoke
`path_open` with [`fdflags::rsync`](#fdflags.rsync) and [`fdflags::dsync`](#fdflags.dsync).

- <a href="#rights.fd_tell" name="rights.fd_tell"></a> `fd_tell`
The right to invoke `fd_seek` in such a way that the file offset
remains unaltered (i.e., [`whence::cur`](#whence.cur) with offset zero), or to
invoke `fd_tell`.

- <a href="#rights.fd_write" name="rights.fd_write"></a> `fd_write`
The right to invoke `fd_write` and `sock_send`.
If [`rights::fd_seek`](#rights.fd_seek) is set, includes the right to invoke `fd_pwrite`.

- <a href="#rights.fd_advise" name="rights.fd_advise"></a> `fd_advise`
The right to invoke `fd_advise`.

- <a href="#rights.fd_allocate" name="rights.fd_allocate"></a> `fd_allocate`
The right to invoke `fd_allocate`.

- <a href="#rights.path_create_directory" name="rights.path_create_directory"></a> `path_create_directory`
The right to invoke `path_create_directory`.

- <a href="#rights.path_create_file" name="rights.path_create_file"></a> `path_create_file`
If `path_open` is set, the right to invoke `path_open` with [`oflags::create`](#oflags.create).

- <a href="#rights.path_link_source" name="rights.path_link_source"></a> `path_link_source`
The right to invoke `path_link` with the file descriptor as the
source directory.

- <a href="#rights.path_link_target" name="rights.path_link_target"></a> `path_link_target`
The right to invoke `path_link` with the file descriptor as the
target directory.

- <a href="#rights.path_open" name="rights.path_open"></a> `path_open`
The right to invoke `path_open`.

- <a href="#rights.fd_readdir" name="rights.fd_readdir"></a> `fd_readdir`
The right to invoke `fd_readdir`.

- <a href="#rights.path_readlink" name="rights.path_readlink"></a> `path_readlink`
The right to invoke `path_readlink`.

- <a href="#rights.path_rename_source" name="rights.path_rename_source"></a> `path_rename_source`
The right to invoke `path_rename` with the file descriptor as the source directory.

- <a href="#rights.path_rename_target" name="rights.path_rename_target"></a> `path_rename_target`
The right to invoke `path_rename` with the file descriptor as the target directory.

- <a href="#rights.path_filestat_get" name="rights.path_filestat_get"></a> `path_filestat_get`
The right to invoke `path_filestat_get`.

- <a href="#rights.path_filestat_set_size" name="rights.path_filestat_set_size"></a> `path_filestat_set_size`
The right to change a file's size (there is no `path_filestat_set_size`).
If `path_open` is set, includes the right to invoke `path_open` with [`oflags::trunc`](#oflags.trunc).

- <a href="#rights.path_filestat_set_times" name="rights.path_filestat_set_times"></a> `path_filestat_set_times`
The right to invoke `path_filestat_set_times`.

- <a href="#rights.path_permissions_set" name="rights.path_permissions_set"></a> `path_permissions_set`
The right to invoke `path_permissions_set`.

- <a href="#rights.fd_filestat_get" name="rights.fd_filestat_get"></a> `fd_filestat_get`
The right to invoke `fd_filestat_get`.

- <a href="#rights.fd_filestat_set_size" name="rights.fd_filestat_set_size"></a> `fd_filestat_set_size`
The right to invoke `fd_filestat_set_size`.

- <a href="#rights.fd_filestat_set_times" name="rights.fd_filestat_set_times"></a> `fd_filestat_set_times`
The right to invoke `fd_filestat_set_times`.

- <a href="#rights.fd_permissions_set" name="rights.fd_permissions_set"></a> `fd_permissions_set`
The right to invoke `fd_permissions_set`.

- <a href="#rights.path_symlink" name="rights.path_symlink"></a> `path_symlink`
The right to invoke `path_symlink`.

- <a href="#rights.path_remove_directory" name="rights.path_remove_directory"></a> `path_remove_directory`
The right to invoke `path_remove_directory`.

- <a href="#rights.path_unlink_file" name="rights.path_unlink_file"></a> `path_unlink_file`
The right to invoke `path_unlink_file`.

- <a href="#rights.poll_fd_readwrite" name="rights.poll_fd_readwrite"></a> `poll_fd_readwrite`
If [`rights::fd_read`](#rights.fd_read) is set, includes the right to invoke `poll_oneoff` to subscribe to [`eventtype::fd_read`](#eventtype.fd_read).
If [`rights::fd_write`](#rights.fd_write) is set, includes the right to invoke `poll_oneoff` to subscribe to [`eventtype::fd_write`](#eventtype.fd_write).

- <a href="#rights.sock_shutdown" name="rights.sock_shutdown"></a> `sock_shutdown`
The right to invoke `sock_shutdown`.

## <a href="#fd" name="fd"></a> `fd`
A file descriptor handle.

Size: 4

Alignment: 4

### Supertypes
## <a href="#iovec" name="iovec"></a> `iovec`: Struct
A region of memory for scatter/gather reads.

Size: 8

Alignment: 4

### Struct members
- <a href="#iovec.buf" name="iovec.buf"></a> `buf`: `Pointer<u8>`
The address of the buffer to be filled.

Offset: 0

- <a href="#iovec.buf_len" name="iovec.buf_len"></a> `buf_len`: [`size`](#size)
The length of the buffer to be filled.

Offset: 4

## <a href="#ciovec" name="ciovec"></a> `ciovec`: Struct
A region of memory for scatter/gather writes.

Size: 8

Alignment: 4

### Struct members
- <a href="#ciovec.buf" name="ciovec.buf"></a> `buf`: `ConstPointer<u8>`
The address of the buffer to be written.

Offset: 0

- <a href="#ciovec.buf_len" name="ciovec.buf_len"></a> `buf_len`: [`size`](#size)
The length of the buffer to be written.

Offset: 4

## <a href="#iovec_array" name="iovec_array"></a> `iovec_array`: `Array<iovec>`

Size: 8

Alignment: 4

## <a href="#ciovec_array" name="ciovec_array"></a> `ciovec_array`: `Array<ciovec>`

Size: 8

Alignment: 4

## <a href="#filedelta" name="filedelta"></a> `filedelta`: `s64`
Relative offset within a file.

Size: 8

Alignment: 8

## <a href="#whence" name="whence"></a> `whence`: Enum(`u8`)
The position relative to which to set the offset of the file descriptor.

Size: 1

Alignment: 1

### Variants
- <a href="#whence.set" name="whence.set"></a> `set`
Seek relative to start-of-file.

- <a href="#whence.cur" name="whence.cur"></a> `cur`
Seek relative to current position.

- <a href="#whence.end" name="whence.end"></a> `end`
Seek relative to end-of-file.

## <a href="#dircookie" name="dircookie"></a> `dircookie`: Int(`u64`)
A reference to the offset of a directory entry.

Size: 8

Alignment: 8

### Consts
- <a href="#dircookie.start" name="dircookie.start"></a> `start`
In an `fd_readdir` call, this value signifies the start of the directory.

## <a href="#dirnamlen" name="dirnamlen"></a> `dirnamlen`: `u32`
The type for the [`dirent::d_namlen`](#dirent.d_namlen) field of [`dirent`](#dirent).

Size: 4

Alignment: 4

## <a href="#inode" name="inode"></a> `inode`: `u64`
File serial number that is unique within its file system.

Size: 8

Alignment: 8

## <a href="#filetype" name="filetype"></a> `filetype`: Enum(`u8`)
The type of a file descriptor or file.

Size: 1

Alignment: 1

### Variants
- <a href="#filetype.unknown" name="filetype.unknown"></a> `unknown`
The type of the file descriptor or file is unknown or is different from any of the other types specified.

- <a href="#filetype.block_device" name="filetype.block_device"></a> `block_device`
The file descriptor or file refers to a block device inode.

- <a href="#filetype.character_device" name="filetype.character_device"></a> `character_device`
The file descriptor or file refers to a character device inode.

- <a href="#filetype.directory" name="filetype.directory"></a> `directory`
The file descriptor or file refers to a directory inode.

- <a href="#filetype.regular_file" name="filetype.regular_file"></a> `regular_file`
The file descriptor or file refers to a regular file inode.

- <a href="#filetype.socket_dgram" name="filetype.socket_dgram"></a> `socket_dgram`
The file descriptor or file refers to a datagram socket.

- <a href="#filetype.socket_stream" name="filetype.socket_stream"></a> `socket_stream`
The file descriptor or file refers to a byte-stream socket.

- <a href="#filetype.symbolic_link" name="filetype.symbolic_link"></a> `symbolic_link`
The file refers to a symbolic link inode.

- <a href="#filetype.fifo" name="filetype.fifo"></a> `fifo`
The file descriptor or file refers to a FIFO.

## <a href="#dirent" name="dirent"></a> `dirent`: Struct
A directory entry.

Size: 24

Alignment: 8

### Struct members
- <a href="#dirent.d_next" name="dirent.d_next"></a> `d_next`: [`dircookie`](#dircookie)
The offset of the next directory entry stored in this directory.

Offset: 0

- <a href="#dirent.d_ino" name="dirent.d_ino"></a> `d_ino`: [`inode`](#inode)
The serial number of the file referred to by this directory entry.

Offset: 8

- <a href="#dirent.d_namlen" name="dirent.d_namlen"></a> `d_namlen`: [`dirnamlen`](#dirnamlen)
The length of the name of the directory entry.

Offset: 16

- <a href="#dirent.d_type" name="dirent.d_type"></a> `d_type`: [`filetype`](#filetype)
The type of the file referred to by this directory entry.

Offset: 20

## <a href="#advice" name="advice"></a> `advice`: Enum(`u8`)
File or memory access pattern advisory information.

Size: 1

Alignment: 1

### Variants
- <a href="#advice.normal" name="advice.normal"></a> `normal`
The application has no advice to give on its behavior with respect to the specified data.

- <a href="#advice.sequential" name="advice.sequential"></a> `sequential`
The application expects to access the specified data sequentially from lower offsets to higher offsets.

- <a href="#advice.random" name="advice.random"></a> `random`
The application expects to access the specified data in a random order.

- <a href="#advice.willneed" name="advice.willneed"></a> `willneed`
The application expects to access the specified data in the near future.

- <a href="#advice.dontneed" name="advice.dontneed"></a> `dontneed`
The application expects that it will not access the specified data in the near future.

- <a href="#advice.noreuse" name="advice.noreuse"></a> `noreuse`
The application expects to access the specified data once and then not reuse it thereafter.

## <a href="#fdflags" name="fdflags"></a> `fdflags`: Flags(`u16`)
File descriptor flags.

Size: 2

Alignment: 2

### Flags
- <a href="#fdflags.append" name="fdflags.append"></a> `append`
Append mode: Data written to the file is always appended to the file's end.

- <a href="#fdflags.dsync" name="fdflags.dsync"></a> `dsync`
Write according to synchronized I/O data integrity completion. Only the data stored in the file is synchronized.

- <a href="#fdflags.nonblock" name="fdflags.nonblock"></a> `nonblock`
Non-blocking mode.

- <a href="#fdflags.rsync" name="fdflags.rsync"></a> `rsync`
Synchronized read I/O operations.

- <a href="#fdflags.sync" name="fdflags.sync"></a> `sync`
Write according to synchronized I/O file integrity completion. In
addition to synchronizing the data stored in the file, the implementation
may also synchronously update the file's metadata.

## <a href="#fdstat" name="fdstat"></a> `fdstat`: Struct
File descriptor attributes.

Size: 24

Alignment: 8

### Struct members
- <a href="#fdstat.fs_filetype" name="fdstat.fs_filetype"></a> `fs_filetype`: [`filetype`](#filetype)
File type.

Offset: 0

- <a href="#fdstat.fs_flags" name="fdstat.fs_flags"></a> `fs_flags`: [`fdflags`](#fdflags)
File descriptor flags.

Offset: 2

- <a href="#fdstat.fs_rights_base" name="fdstat.fs_rights_base"></a> `fs_rights_base`: [`rights`](#rights)
Rights that apply to this file descriptor.

Offset: 8

- <a href="#fdstat.fs_rights_inheriting" name="fdstat.fs_rights_inheriting"></a> `fs_rights_inheriting`: [`rights`](#rights)
Maximum set of rights that may be installed on new file descriptors that
are created through this file descriptor, e.g., through `path_open`.

Offset: 16

## <a href="#device" name="device"></a> `device`: `u64`
Identifier for a device containing a file system. Can be used in combination
with [`inode`](#inode) to uniquely identify a file or directory in the filesystem.

Size: 8

Alignment: 8

## <a href="#fstflags" name="fstflags"></a> `fstflags`: Flags(`u16`)
Which file time attributes to adjust.

Size: 2

Alignment: 2

### Flags
- <a href="#fstflags.atim" name="fstflags.atim"></a> `atim`
Adjust the last data access timestamp to the value stored in [`filestat::atim`](#filestat.atim).

- <a href="#fstflags.atim_now" name="fstflags.atim_now"></a> `atim_now`
Adjust the last data access timestamp to the time of clock [`clockid::realtime`](#clockid.realtime).

- <a href="#fstflags.mtim" name="fstflags.mtim"></a> `mtim`
Adjust the last data modification timestamp to the value stored in [`filestat::mtim`](#filestat.mtim).

- <a href="#fstflags.mtim_now" name="fstflags.mtim_now"></a> `mtim_now`
Adjust the last data modification timestamp to the time of clock [`clockid::realtime`](#clockid.realtime).

## <a href="#lookupflags" name="lookupflags"></a> `lookupflags`: Flags(`u32`)
Flags determining the method of how paths are resolved.

Size: 4

Alignment: 4

### Flags
- <a href="#lookupflags.symlink_follow" name="lookupflags.symlink_follow"></a> `symlink_follow`
As long as the resolved path corresponds to a symbolic link, it is expanded.

## <a href="#oflags" name="oflags"></a> `oflags`: Flags(`u16`)
Open flags used by `path_open`.

Size: 2

Alignment: 2

### Flags
- <a href="#oflags.create" name="oflags.create"></a> `create`
Create file if it does not exist.

- <a href="#oflags.directory" name="oflags.directory"></a> `directory`
Fail if not a directory.

- <a href="#oflags.excl" name="oflags.excl"></a> `excl`
Fail if file already exists.

- <a href="#oflags.trunc" name="oflags.trunc"></a> `trunc`
Truncate file to size 0.

## <a href="#linkcount" name="linkcount"></a> `linkcount`: `u64`
Number of hard links to an inode.

Size: 8

Alignment: 8

## <a href="#permissions" name="permissions"></a> `permissions`: Flags(`u8`)
File permissions. This represents the permissions associated with a
file in a filesystem, and don't fully reflect all the conditions
which determine whether a given WASI program can access the file.

Size: 1

Alignment: 1

### Flags
- <a href="#permissions.read" name="permissions.read"></a> `read`
For files, permission to read the file.
For directories, permission to do `readdir` and access files
within the directory.

Note: This is similar to the read bit being set on files, and the
read *and* execute bits being set on directories, in POSIX.

- <a href="#permissions.write" name="permissions.write"></a> `write`
For files, permission to mutate the file.
For directories, permission to create, remove, and rename items
within the directory.

- <a href="#permissions.execute" name="permissions.execute"></a> `execute`
For files, permission to "execute" the file, using whatever
concept of "executing" the host filesystem has.
This flag is not valid for directories.

- <a href="#permissions.private" name="permissions.private"></a> `private`
For filesystems which have a concept of multiple "users", this flag
indicates that the file is only accessible by the effective "user"
that the WASI store uses to access the filesystem, and inaccessible
to other "users".

## <a href="#filestat" name="filestat"></a> `filestat`: Struct
File attributes.

Size: 64

Alignment: 8

### Struct members
- <a href="#filestat.dev" name="filestat.dev"></a> `dev`: [`device`](#device)
Device ID of device containing the file.

Offset: 0

- <a href="#filestat.ino" name="filestat.ino"></a> `ino`: [`inode`](#inode)
File serial number.

Offset: 8

- <a href="#filestat.filetype" name="filestat.filetype"></a> `filetype`: [`filetype`](#filetype)
File type.

Offset: 16

- <a href="#filestat.permissions" name="filestat.permissions"></a> `permissions`: [`permissions`](#permissions)
File permissions.

Offset: 17

- <a href="#filestat.nlink" name="filestat.nlink"></a> `nlink`: [`linkcount`](#linkcount)
Number of hard links to the file.

Offset: 24

- <a href="#filestat.size" name="filestat.size"></a> `size`: [`filesize`](#filesize)
For regular files, the file size in bytes. For symbolic links, the length in bytes of the pathname contained in the symbolic link.

Offset: 32

- <a href="#filestat.atim" name="filestat.atim"></a> `atim`: [`timestamp`](#timestamp)
Last data access timestamp.

Offset: 40

- <a href="#filestat.mtim" name="filestat.mtim"></a> `mtim`: [`timestamp`](#timestamp)
Last data modification timestamp.

Offset: 48

- <a href="#filestat.ctim" name="filestat.ctim"></a> `ctim`: [`timestamp`](#timestamp)
Last file status change timestamp.

Offset: 56

## <a href="#userdata" name="userdata"></a> `userdata`: `u64`
User-provided value that may be attached to objects that is retained when
extracted from the implementation.

Size: 8

Alignment: 8

## <a href="#eventtype" name="eventtype"></a> `eventtype`: Enum(`u8`)
Type of a subscription to an event or its occurrence.

Size: 1

Alignment: 1

### Variants
- <a href="#eventtype.clock" name="eventtype.clock"></a> `clock`
The time value of clock [`subscription_clock::id`](#subscription_clock.id) has
reached timestamp [`subscription_clock::timeout`](#subscription_clock.timeout).

- <a href="#eventtype.fd_read" name="eventtype.fd_read"></a> `fd_read`
File descriptor [`subscription_fd_readwrite::fd`](#subscription_fd_readwrite.fd) has data
available for reading. This event always triggers for regular files.

- <a href="#eventtype.fd_write" name="eventtype.fd_write"></a> `fd_write`
File descriptor [`subscription_fd_readwrite::fd`](#subscription_fd_readwrite.fd) has capacity
available for writing. This event always triggers for regular files.

## <a href="#eventrwflags" name="eventrwflags"></a> `eventrwflags`: Flags(`u16`)
The state of the file descriptor subscribed to with
[`eventtype::fd_read`](#eventtype.fd_read) or [`eventtype::fd_write`](#eventtype.fd_write).

Size: 2

Alignment: 2

### Flags
- <a href="#eventrwflags.fd_readwrite_hangup" name="eventrwflags.fd_readwrite_hangup"></a> `fd_readwrite_hangup`
The peer of this socket has closed or disconnected.

## <a href="#event_fd_readwrite" name="event_fd_readwrite"></a> `event_fd_readwrite`: Struct
The contents of an [`event`](#event) when type is [`eventtype::fd_read`](#eventtype.fd_read) or
[`eventtype::fd_write`](#eventtype.fd_write).

Size: 16

Alignment: 8

### Struct members
- <a href="#event_fd_readwrite.nbytes" name="event_fd_readwrite.nbytes"></a> `nbytes`: [`filesize`](#filesize)
The number of bytes available for reading or writing.

Offset: 0

- <a href="#event_fd_readwrite.flags" name="event_fd_readwrite.flags"></a> `flags`: [`eventrwflags`](#eventrwflags)
The state of the file descriptor.

Offset: 8

## <a href="#event_u" name="event_u"></a> `event_u`: Union
The contents of an [`event`](#event).

Size: 24

Alignment: 8

### Union Layout
- tag_size: 1
- tag_align: 1
- contents_offset: 8
- contents_size: 16
- contents_align: 8
### Union variants
- <a href="#event_u.fd_read" name="event_u.fd_read"></a> `fd_read`: [`event_fd_readwrite`](#event_fd_readwrite)

- <a href="#event_u.fd_write" name="event_u.fd_write"></a> `fd_write`: [`event_fd_readwrite`](#event_fd_readwrite)

- <a href="#event_u.clock" name="event_u.clock"></a> `clock`

## <a href="#event" name="event"></a> `event`: Struct
An event that occurred.

Size: 40

Alignment: 8

### Struct members
- <a href="#event.userdata" name="event.userdata"></a> `userdata`: [`userdata`](#userdata)
User-provided value that got attached to [`subscription::userdata`](#subscription.userdata).

Offset: 0

- <a href="#event.error" name="event.error"></a> `error`: [`errno`](#errno)
If non-zero, an error that occurred while processing the subscription request.

Offset: 8

- <a href="#event.u" name="event.u"></a> `u`: [`event_u`](#event_u)
The type of the event that occurred, and the contents of the event

Offset: 16

## <a href="#subclockflags" name="subclockflags"></a> `subclockflags`: Flags(`u16`)
Flags determining how to interpret the timestamp provided in
[`subscription_clock::timeout`](#subscription_clock.timeout).

Size: 2

Alignment: 2

### Flags
- <a href="#subclockflags.subscription_clock_abstime" name="subclockflags.subscription_clock_abstime"></a> `subscription_clock_abstime`
If set, treat the timestamp provided in
[`subscription_clock::timeout`](#subscription_clock.timeout) as an absolute timestamp of clock
[`subscription_clock::id`](#subscription_clock.id). If clear, treat the timestamp
provided in [`subscription_clock::timeout`](#subscription_clock.timeout) relative to the
current time value of clock [`subscription_clock::id`](#subscription_clock.id).

## <a href="#subscription_clock" name="subscription_clock"></a> `subscription_clock`: Struct
The contents of a [`subscription`](#subscription) when type is [`eventtype::clock`](#eventtype.clock).

Size: 32

Alignment: 8

### Struct members
- <a href="#subscription_clock.id" name="subscription_clock.id"></a> `id`: [`clockid`](#clockid)
The clock against which to compare the timestamp.

Offset: 0

- <a href="#subscription_clock.timeout" name="subscription_clock.timeout"></a> `timeout`: [`timestamp`](#timestamp)
The absolute or relative timestamp.

Offset: 8

- <a href="#subscription_clock.precision" name="subscription_clock.precision"></a> `precision`: [`timestamp`](#timestamp)
The amount of time that the implementation may wait additionally
to coalesce with other events.

Offset: 16

- <a href="#subscription_clock.flags" name="subscription_clock.flags"></a> `flags`: [`subclockflags`](#subclockflags)
Flags specifying whether the timeout is absolute or relative

Offset: 24

## <a href="#subscription_fd_readwrite" name="subscription_fd_readwrite"></a> `subscription_fd_readwrite`: Struct
The contents of a [`subscription`](#subscription) when type is type is
[`eventtype::fd_read`](#eventtype.fd_read) or [`eventtype::fd_write`](#eventtype.fd_write).

Size: 4

Alignment: 4

### Struct members
- <a href="#subscription_fd_readwrite.fd" name="subscription_fd_readwrite.fd"></a> `fd`: [`fd`](#fd)
The file descriptor on which to wait for it to become ready for reading or writing.

Offset: 0

## <a href="#subscription_u" name="subscription_u"></a> `subscription_u`: Union
The contents of a [`subscription`](#subscription).

Size: 40

Alignment: 8

### Union Layout
- tag_size: 1
- tag_align: 1
- contents_offset: 8
- contents_size: 32
- contents_align: 8
### Union variants
- <a href="#subscription_u.clock" name="subscription_u.clock"></a> `clock`: [`subscription_clock`](#subscription_clock)

- <a href="#subscription_u.fd_read" name="subscription_u.fd_read"></a> `fd_read`: [`subscription_fd_readwrite`](#subscription_fd_readwrite)

- <a href="#subscription_u.fd_write" name="subscription_u.fd_write"></a> `fd_write`: [`subscription_fd_readwrite`](#subscription_fd_readwrite)

## <a href="#subscription" name="subscription"></a> `subscription`: Struct
Subscription to an event.

Size: 48

Alignment: 8

### Struct members
- <a href="#subscription.userdata" name="subscription.userdata"></a> `userdata`: [`userdata`](#userdata)
User-provided value that is attached to the subscription in the
implementation and returned through [`event::userdata`](#event.userdata).

Offset: 0

- <a href="#subscription.u" name="subscription.u"></a> `u`: [`subscription_u`](#subscription_u)
The type of the event to which to subscribe, and the contents of the subscription.

Offset: 8

## <a href="#exitcode" name="exitcode"></a> `exitcode`: `u32`
Exit code generated by a process when exiting.

Size: 4

Alignment: 4

## <a href="#riflags" name="riflags"></a> `riflags`: Flags(`u16`)
Flags provided to `sock_recv`.

Size: 2

Alignment: 2

### Flags
- <a href="#riflags.recv_peek" name="riflags.recv_peek"></a> `recv_peek`
Returns the message without removing it from the socket's receive queue.

- <a href="#riflags.recv_waitall" name="riflags.recv_waitall"></a> `recv_waitall`
On byte-stream sockets, block until the full amount of data can be returned.

## <a href="#roflags" name="roflags"></a> `roflags`: Flags(`u16`)
Flags returned by `sock_recv`.

Size: 2

Alignment: 2

### Flags
- <a href="#roflags.recv_data_truncated" name="roflags.recv_data_truncated"></a> `recv_data_truncated`
Returned by `sock_recv`: Message data has been truncated.

## <a href="#siflags" name="siflags"></a> `siflags`: `u16`
Flags provided to `sock_send`. As there are currently no flags
defined, it must be set to zero.

Size: 2

Alignment: 2

## <a href="#sdflags" name="sdflags"></a> `sdflags`: Flags(`u8`)
Which channels on a socket to shut down.

Size: 1

Alignment: 1

### Flags
- <a href="#sdflags.rd" name="sdflags.rd"></a> `rd`
Disables further receive operations.

- <a href="#sdflags.wr" name="sdflags.wr"></a> `wr`
Disables further send operations.

## <a href="#preopentype" name="preopentype"></a> `preopentype`: Enum(`u8`)
Identifiers for preopened capabilities.

Size: 1

Alignment: 1

### Variants
- <a href="#preopentype.dir" name="preopentype.dir"></a> `dir`
A pre-opened directory.

## <a href="#prestat_dir" name="prestat_dir"></a> `prestat_dir`: Struct
The contents of a [`prestat`](#prestat) when its type is [`preopentype::dir`](#preopentype.dir).

Size: 4

Alignment: 4

### Struct members
- <a href="#prestat_dir.pr_name_len" name="prestat_dir.pr_name_len"></a> `pr_name_len`: [`size`](#size)
The length of the directory name for use with `fd_prestat_dir_name`.

Offset: 0

## <a href="#prestat" name="prestat"></a> `prestat`: Union
Information about a pre-opened capability.

Size: 8

Alignment: 4

### Union Layout
- tag_size: 1
- tag_align: 1
- contents_offset: 4
- contents_size: 4
- contents_align: 4
### Union variants
- <a href="#prestat.dir" name="prestat.dir"></a> `dir`: [`prestat_dir`](#prestat_dir)
When type is [`preopentype::dir`](#preopentype.dir):

## <a href="#protocolfd" name="protocolfd"></a> `protocolfd`: [`fd`](#fd)
The protocol file descriptor used for creating any networking file descriptor.

Size: 4

Alignment: 4

## <a href="#controlfd" name="controlfd"></a> `controlfd`: [`fd`](#fd)
The control file descriptor used for controlling the socket behavior.

Size: 4

Alignment: 4

## <a href="#connectionfd" name="connectionfd"></a> `connectionfd`: [`fd`](#fd)
The file descriptor representing a network connection.

Size: 4

Alignment: 4

## <a href="#streamfd" name="streamfd"></a> `streamfd`: [`fd`](#fd)
The file descriptor representing a communication stream within a connection.

Size: 4

Alignment: 4

## <a href="#sockettype" name="sockettype"></a> `sockettype`: [`filetype`](#filetype)
[`filetype`](#filetype) already has `socket_dgram` and `socket_stream`.

Size: 1

Alignment: 1

# Modules
## <a href="#networking" name="networking"></a> networking
### Imports
#### Memory
### Functions

---

#### <a href="#open" name="open"></a> `open(protocol_fd: protocolfd, bind_address: string, type: sockettype, rights_base: rights, rights_inherting: rights, fdflags: fdflags) -> (errno, controlfd)`
Open a networking socket.
Note: This is similar to `socket` and `bind` in POSIX.

##### Params
- <a href="#open.protocol_fd" name="open.protocol_fd"></a> `protocol_fd`: [`protocolfd`](#protocolfd)
The file descriptor that represents a protocol in a
capability-oriented manner.

- <a href="#open.bind_address" name="open.bind_address"></a> `bind_address`: `string`
The address assigned to the created socket.

- <a href="#open.type" name="open.type"></a> `type`: [`sockettype`](#sockettype)

- <a href="#open.rights_base" name="open.rights_base"></a> `rights_base`: [`rights`](#rights)
The initial rights of the newly created file descriptor. The
implementation is allowed to return a file descriptor with fewer rights
than specified, if and only if those rights do not apply to the type of
file being opened.
The *base* rights are rights that will apply to operations using the file
descriptor itself, while the *inheriting* rights are rights that apply to
file descriptors derived from it.

- <a href="#open.rights_inherting" name="open.rights_inherting"></a> `rights_inherting`: [`rights`](#rights)

- <a href="#open.fdflags" name="open.fdflags"></a> `fdflags`: [`fdflags`](#fdflags)

##### Results
- <a href="#open.error" name="open.error"></a> `error`: [`errno`](#errno)

- <a href="#open.opened_controlfd" name="open.opened_controlfd"></a> `opened_controlfd`: [`controlfd`](#controlfd)
The file descriptor of the socket that has been opened.


---

#### <a href="#listen" name="listen"></a> `listen(controlfd: controlfd, backlog: size) -> errno`
Listen on a network socket.
Note: This is similar to [`listen`](#listen) in POSIX.

##### Params
- <a href="#listen.controlfd" name="listen.controlfd"></a> `controlfd`: [`controlfd`](#controlfd)

- <a href="#listen.backlog" name="listen.backlog"></a> `backlog`: [`size`](#size)
The maximum length to which the pending connections may grow.

##### Results
- <a href="#listen.error" name="listen.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#accept" name="accept"></a> `accept(controlfd: controlfd) -> (errno, connectionfd)`
Accept an incoming connection.

Note: This is similar to [`accept`](#accept) in POSIX, but the file
descriptor is not immediately usable for reading/writing until
[`stream_create`](#stream_create) or [`stream_accept`](#stream_accept) is called.

##### Params
- <a href="#accept.controlfd" name="accept.controlfd"></a> `controlfd`: [`controlfd`](#controlfd)

##### Results
- <a href="#accept.error" name="accept.error"></a> `error`: [`errno`](#errno)

- <a href="#accept.opened_connectionfd" name="accept.opened_connectionfd"></a> `opened_connectionfd`: [`connectionfd`](#connectionfd)


---

#### <a href="#connect" name="connect"></a> `connect(controlfd: controlfd, address: string) -> (errno, connectionfd)`
Connect a network socket to an external address.

Note: This is similar to [`connect`](#connect) in POSIX, but the file
descriptor is not immediately usable for reading/writing until
[`stream_create`](#stream_create) or [`stream_accept`](#stream_accept) is called.

##### Params
- <a href="#connect.controlfd" name="connect.controlfd"></a> `controlfd`: [`controlfd`](#controlfd)

- <a href="#connect.address" name="connect.address"></a> `address`: `string`
The destination address.

##### Results
- <a href="#connect.error" name="connect.error"></a> `error`: [`errno`](#errno)

- <a href="#connect.opened_connectionfd" name="connect.opened_connectionfd"></a> `opened_connectionfd`: [`connectionfd`](#connectionfd)


---

#### <a href="#stream_create" name="stream_create"></a> `stream_create(connectionfd: connectionfd, rights: rights, fdflags: fdflags) -> (errno, streamfd)`
Create a stream for communication and request the remote peer to
accept it.

##### Params
- <a href="#stream_create.connectionfd" name="stream_create.connectionfd"></a> `connectionfd`: [`connectionfd`](#connectionfd)

- <a href="#stream_create.rights" name="stream_create.rights"></a> `rights`: [`rights`](#rights)

- <a href="#stream_create.fdflags" name="stream_create.fdflags"></a> `fdflags`: [`fdflags`](#fdflags)

##### Results
- <a href="#stream_create.error" name="stream_create.error"></a> `error`: [`errno`](#errno)

- <a href="#stream_create.opened_streamfd" name="stream_create.opened_streamfd"></a> `opened_streamfd`: [`streamfd`](#streamfd)
The file descriptor of the stream that has been opened.


---

#### <a href="#stream_accept" name="stream_accept"></a> `stream_accept(connectionfd: connectionfd) -> (errno, streamfd)`
Accept an incoming stream from the remote peer.

##### Params
- <a href="#stream_accept.connectionfd" name="stream_accept.connectionfd"></a> `connectionfd`: [`connectionfd`](#connectionfd)

##### Results
- <a href="#stream_accept.error" name="stream_accept.error"></a> `error`: [`errno`](#errno)

- <a href="#stream_accept.opened_streamfd" name="stream_accept.opened_streamfd"></a> `opened_streamfd`: [`streamfd`](#streamfd)
The file descriptor of the stream that has been opened.

