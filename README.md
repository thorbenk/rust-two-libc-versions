I'm trying to understand why this small testproject gives me the
following error:

```
$#> cargo build

    Updating registry `https://github.com/rust-lang/crates.io-index`
   Compiling mozjpeg-sys v0.2.0
   Compiling libc v0.1.10
   Compiling rust-two-libc-versions v0.1.0 (file:///tmp/rust-two-libc-versions)
src/main.rs:26:37: 26:44 error: mismatched types:
 expected `*mut libc::types::common::c95::FILE`,
    found `*mut libc::types::common::c95::FILE`
(expected enum `libc::types::common::c95::FILE`,
    found a different enum `libc::types::common::c95::FILE`) [E0308]
src/main.rs:26         jpeg_stdio_dest(&mut cinfo, outfile);
                                                   ^~~~~~~
src/main.rs:26:37: 26:44 help: run `rustc --explain E0308` to see a detailed explanation
src/main.rs:26:37: 26:44 note: Perhaps two different versions of crate `libc` are being used?
src/main.rs:26         jpeg_stdio_dest(&mut cinfo, outfile);
                                                   ^~~~~~~
error: aborting due to previous error
Could not compile `rust-two-libc-versions`.
```

Note: for `mozjpeg-sys`, you need to do (at least)
```
sudo apt-get install nasm dh-autoreconf
```
