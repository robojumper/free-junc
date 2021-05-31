# free-junc

Mark Russinovich's [Sysinternals Junction](https://docs.microsoft.com/en-us/sysinternals/downloads/junction)
is an important tool but subject to restrictive licensing terms. Here, have some free junctions instead.

It accepts `junction.exe`'s command line arguments, so it can be used in place of `junction.exe`.

```
free-junc v1.0: Create, delete and list NTFS junctions.
Usage: target\release\free-junc.exe [-s] [-q] <directory>
    List junction, if it exists, at the given directory.
      -s: Recursive. Print all junctions at and below the given directory.
      -q: Quiet. Do not report filesystem access errors.

Usage: target\release\free-junc.exe <junction directory> <target directory>
    Create a junction from <junction directory> to <target directory>.

Usage: target\release\free-junc.exe -d <junction directory>
    Remove the junction at <junction directory> and remove the resulting empty directory.
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.