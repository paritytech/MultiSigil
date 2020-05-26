# MultiSigil — Substrate multisig address calculator for your CLI

It is basically what it says on the tin.
Since Substrate multisig addresses are [deterministic](https://github.com/paritytech/substrate/blob/bda3b4092681cc1ab95be4de71fe3a313721852a/frame/utility/src/lib.rs#L318-L343), MultiSigil doesn't need to do any network connections — and can be used even _before_ the chain has been started.

## Usage

```
$ multi-sigil --help

multi-sigil 0.1.0
Parity Technologies <admin@parity.io>
CLI for generating Substrate multisig addresses

USAGE:
    multi-sigil [OPTIONS] <THRESHOLD> <ADDRESSES>...

ARGS:
    <THRESHOLD>       The number of signatures needed to perform the operation
    <ADDRESSES>...    The addresses to use

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --network <NETWORK>    Network to calculate multisig for; defaults to Kusama [default: kusama]  [possible
                               values: kusama, polkadot]
```

### Supported networks

Currently only Kusama and Polkadot are supported.

It should be fairly trivial to add support of other networks from [the list of supported in SS58](https://github.com/paritytech/substrate/blob/dbf2163250833e6ac898e7f6c3c8f89f08a7c19d/primitives/core/src/crypto.rs#L436-L480) — PRs are welcome!
