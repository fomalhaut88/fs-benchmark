# fs-benchmark

Benchmark between different standard ways to interact with the file system.

* std: `std::fs`
* std-ext: `std::fs + std::os::unix::prelude::FileExt`
* tokio: `tokio::fs`
* tokio-uring: `tokio_uring::fs`

`std` and `std-ext` are synchronous, `tokio` and `tokio-uring` are asynchronous.

## Test cases

Typical results:

```
Test for std:
Case 1. Push bytes:     55.446064ms
Case 2. Iterate blocks: 20.169141ms
Case 3. Random read:    32.383698ms
Case 4. Random update:  56.874558ms

Test for std-ext:
Case 1. Push bytes:     51.172654ms
Case 2. Iterate blocks: 19.236618ms
Case 3. Random read:    19.409872ms
Case 4. Random update:  47.623714ms

Test for tokio:
Case 1. Push bytes:     775.295021ms
Case 2. Iterate blocks: 356.283701ms
Case 3. Random read:    751.224829ms
Case 4. Random update:  890.48805ms

Test for tokio-uring:
Case 1. Push bytes:     566.379372ms
Case 2. Iterate blocks: 139.441158ms
Case 3. Random read:    136.529529ms
Case 4. Random update:  502.580173ms
```

Test options:

```rust
const COUNT: usize = 100000;
const BLOCK_SIZE: usize = 64;
```
