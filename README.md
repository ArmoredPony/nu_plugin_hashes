# Hashes for Nushell

A [Nushell](https://www.nushell.sh) plugin that adds a collection of **63**
cryptographic hash functions from [Hashes](https://github.com/RustCrypto/hashes)
project.

This plugin's implementation is based on code stolen from the official Nushell
repository and on compile-time code generation with a build script.

Excess algorithms can be filtered off by selecting only specific features of the
crate.

## Installation

To install this plugin with all algorithms available run
```nu
cargo install nu_plugin_hashes
plugin add ($env.CARGO_HOME ++ /bin/nu_plugin_hashes)
```

or on Windows
```nu
cargo install nu_plugin_hashes
plugin add ($env.CARGO_HOME ++ /bin/nu_plugin_hashes.exe)
```

After loading the plugin, execute `help hash` to list newly added commands

```nu
~> help hash
Apply hash function.

...

Subcommands:
  hash ascon - Hash a value using the ascon hash algorithm.
  hash ascon-a - Hash a value using the ascon-a hash algorithm.
  hash belt - Hash a value using the belt hash algorithm.
  hash blake2b-512 - Hash a value using the blake2b-512 hash algorithm.
  hash blake2s-256 - Hash a value using the blake2s-256 hash algorithm.
  hash fsb160 - Hash a value using the fsb160 hash algorithm.
  hash fsb224 - Hash a value using the fsb224 hash algorithm.
  ...
```

## Features

If you only need some algorithms, disable default features and select only
those you need
```nu
cargo install nu_plugin_hashes --no-default-features --features sha2,streebog
```

Then check what's installed
```nu
~> help hash
Apply hash function.

...

Subcommands:
  hash md5 - Hash a value using the md5 hash algorithm.
  hash sha224 - Hash a value using the sha224 hash algorithm.
  hash sha256 - Hash a value using the sha256 hash algorithm.
  hash sha384 - Hash a value using the sha384 hash algorithm.
  hash sha512 - Hash a value using the sha512 hash algorithm.
  hash sha512-224 - Hash a value using the sha512-224 hash algorithm.
  hash sha512-256 - Hash a value using the sha512-256 hash algorithm.
  hash streebog256 - Hash a value using the streebog256 hash algorithm.
  hash streebog512 - Hash a value using the streebog512 hash algorithm.

Flags:
  ...
```

## Hashes

The list of implemented algorithms provided by the plugin can be found
in the Hashes project [repository](https://github.com/RustCrypto/hashes).

Almost all algorithms from this project are included in this plugin. The
exceptions are MD5 and SHA-256 as they are already present in Nushell, and
those algorithms, that don't implement the `Digest` trait or require additional
arguments for them to be run.

If you want to import only particular algorithms, build this plugin with those
features, which names correspond to the crates, that supply the algorithms you
want. Consult this [table](https://github.com/RustCrypto/hashes?tab=readme-ov-file#supported-algorithms)
to match features with required algorithms.

If you disable the default features and forget to enable at least one of them,
the plugin won't compile.

## Implemetation details

All the functions are implemented via generic code that I borrowed from Nushell
source file [generic_digest.rs](https://github.com/nushell/nushell/blob/0.101.0/crates/nu-command/src/hash/generic_digest.rs). 
Help page for each command is generated with a [build script](./build.rs). 
Hashes of the test text for each example are generated during compilation by
this script: the test text is fed to each hashing algorithm, and resulting bytes
are inserted into examples. This approach isdifferent from Nushell's
implementations, where examples are hardcoded as strings. Then, generated
examples are tested with [nu-plugin-test-support](https://crates.io/crates/nu-plugin-test-support)
crate. This ensures that the code in examples always behaves exactly as printed.

## TODO

Here is a list of things that could not be implemented with code generation
as they don't implement the `Digest` trait or require additional arguments
to be executed. I doubt that I will ever complete this list so you are welcome
to contribute.

- [ ] [blake2b] algorithm with runtime defined output size
- [ ] [sha1] algorithm with collision detection

## License

This crate is licensed under [MIT license](LICENSE).

---
<h6>I only created this plugin to explore compile-time code generation and
educate myself on subject of features. The product of my activity terrifies
me and I'm surprised it worked out at all.</h6>

[blake2b]: https://github.com/RustCrypto/hashes/blob/1dbb9535207176fceb93a8ec1d450712714aedec/blake2/src/lib.rs#L67
[sha1]: https://github.com/RustCrypto/hashes/tree/master/sha1-checked
