# xxhashdir_comm

## Problem

- Sometimes you did a backup by "hey, let's just rsync it somewhere"
- Now you struggle to merge such "backups" to save space across hosts and drives?

This helps to identify common or duplicates across different hosts
using collected [xxhashdir](https://github.com/lunatic-cat/xxhashdir) results (plaintext in format: `\d{0,20}  .*` with first column is `xxhash` checksum)

## Howto

### Prepare files with checksums

```sh
# on remote host
xxhashdir . > remote.xxhashdir
# on local host
xxhashdir . > local.xxhashdir
scp remote:remote.xxhashdir remote.xxhashdir
```

### Usage

```sh
# 🚀 to get common files (sources are mostly different)
# you likely want to know this to delete duplicates first, then copy rest
xxhashdir --common local.xxhashdir remote.xxhashdir

# 🚀 to get different files (sources are mostly equal)
# you likely want to know this to merge uniq files from second into first, then delete the second at all
xxhashdir --only-second local.xxhashdir remote.xxhashdir
```

## Why not _

- `rsync` can delete files on reciever, but relies only on filenames and mtime
- `fdupes` works only locally
- `zfs snapshot` + `zfs diff` is perfect but also only local and requires to be a common dataset initially
- incremental backups - you don't always bother to have
## Why use it

- stdout can be reprocessed with sed/grep/whatever again
- unix way
- having fun with rust

## Further plans

- Unify output with standart `comm` utility (columns & accept `-1/2/3`)
- Consider `xxhashdir` with bytesize input and compare bytesizes
