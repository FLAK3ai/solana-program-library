# Solana Program Library

The Solana Program Library (SPL) is a collection of on-chain programs targeting
the [Sealevel parallel
runtime](https://medium.com/solana-labs/sealevel-parallel-processing-thousands-of-smart-contracts-d814b378192).
These programs are tested against Solana's implementation of Sealevel,
solana-runtime, and some are deployed to Mainnet Beta.  As others implement
Sealevel, we will graciously accept patches to ensure the programs here are
portable across all implementations.

For more information see the [SPL documentation](https://spl.solana.com) and the [Token TypeDocs](https://solana-labs.github.io/solana-program-library/token/js/).

## Note

Only a subset of programs within the Solana Program Library repo are deployed to
the Solana Mainnet Beta and maintained by the team. Currently, this includes:

* [associated-token-account](https://github.com/solana-labs/solana-program-library/tree/master/associated-token-account/program)
* [feature-proposal](https://github.com/solana-labs/solana-program-library/tree/master/feature-proposal/program)
* [governance](https://github.com/solana-labs/solana-program-library/tree/master/governance/program)
* [memo](https://github.com/solana-labs/solana-program-library/tree/master/memo/program)
* [name-service](https://github.com/solana-labs/solana-program-library/tree/master/name-service/program)
* [stake-pool](https://github.com/solana-labs/solana-program-library/tree/master/stake-pool/program)
* [token](https://github.com/solana-labs/solana-program-library/tree/master/token/program)

All other programs are maintained on a best-effort basis with community support
and the team has no plans to deploy them to Mainnet Beta at this time. Many
programs, including
[token-swap](https://github.com/solana-labs/solana-program-library/tree/master/token-swap/program)
and [token-lending](https://github.com/solana-labs/solana-program-library/tree/master/token-lending/program),
are not audited, so fork and deploy them at your own risk.

More information about the repository's security policy at
[SECURITY.md](https://github.com/solana-labs/solana-program-library/tree/master/SECURITY.md).

## Development

### Environment Setup

1. Install the latest [Solana tools](https://docs.solana.com/cli/install-solana-cli-tools).
2. Install the latest [Rust stable](https://rustup.rs/). If you already have Rust, run `rustup update` to get the latest version.
3. Install the `libudev` development package for your distribution (`libudev-dev` on Debian-derived distros, `libudev-devel` on Redhat-derived).

### Build

### Build on-chain programs

```bash
# To build all on-chain programs
$ cargo build-sbf

# To build a specific on-chain program
$ cd <program_name>/program
$ cargo build-sbf
```

### Build clients

```bash
# To build all clients
$ cargo build

# To build a specific client
$ cd <program_name>/cli
$ cargo build
```

### Test

Unit tests contained within all projects can be run with:
```bash
$ cargo test      # <-- runs host-based tests
$ cargo test-sbf  # <-- runs BPF program tests
```

To run a specific program's tests, such as SPL Token:
```bash
$ cd token/program
$ cargo test      # <-- runs host-based tests
$ cargo test-sbf  # <-- runs BPF program tests
```

Integration testing may be performed via the per-project .js bindings.  See the
[token program's js project](token/js) for an example.

### Common Issues

Solutions to a few issues you might run into are mentioned here.

1. `Failed to open: ../../deploy/spl_<program-name>.so`

    Update your Rust and Cargo to the latest versions and re-run `cargo build-sbf` in the relevant `<program-name>` directory,
    or run it at the repository root to rebuild all on-chain programs.

2. [Error while loading shared libraries. (libssl.so.1.1)](https://solana.stackexchange.com/q/3029/36)

    A working solution was mentioned [here](https://solana.stackexchange.com/q/3029/36).
    Install libssl.
    ```bash
    wget http://nz2.archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1l-1ubuntu1.2_amd64.deb
    sudo dpkg -i libssl1.1_1.1.1l-1ubuntu1.2_amd64.deb
    ```

3.  CPU or Memory usage at 100%

    This is to be expected while building some of the programs in this library.
    The simplest solution is to add the `--jobs 1` flag to the build commands to limit the number of parallel jobs to 1 and check if that fixes the issue. Although this will mean much longer build times.


### Clippy
```bash
$ cargo clippy
```

### Coverage
```bash
$ ./coverage.sh  # Help wanted! Coverage build currently fails on MacOS due to an XCode `grcov` mismatch...
```

#### MacOS

You may need to pin your grcov version, and then rustup with the apple-darwin nightly toolchain:
```bash
$ cargo install grcov --version 0.6.1
$ rustup toolchain install nightly-x86_64-apple-darwin
```


## Release Process

SPL programs are currently tagged and released manually. Each program is
versioned independently of the others, with all new development occurring on
master. Once a program is tested and deemed ready for release:

### Bump Version

  * Increment the version number in the program's Cargo.toml
  * Run `cargo build-sbf <program>` to build binary. Note the
    location of the generated `spl_<program>.so` for attaching to the Github
    release.
  * Open a PR with these version changes and merge after passing CI.

### Create Github tag

Program tags are of the form `<program>-vX.Y.Z`.
Create the new tag at the version-bump commit and push to the
solana-program-library repository, eg:

```
$ git tag token-v1.0.0 b24bfe7
$ git push upstream --tags
```

### Publish Github release

  * Go to [GitHub Releases UI](https://github.com/solana-labs/solana-program-library/releases)
  * Click "Draft new release", and enter the new tag in the "Tag version" box.
  * Title the release "SPL <Program> vX.Y.Z", complete the description, and attach the `spl_<program>.so` binary
  * Click "Publish release"

### Publish to Crates.io

Navigate to the program directory and run `cargo package`
to test the build. Then run `cargo publish`.

 # Disclaimer

All claims, content, designs, algorithms, estimates, roadmaps,
specifications, and performance measurements described in this project
are done with the Solana Labs, Inc. (“SL”) best efforts. It is up to
the reader to check and validate their accuracy and truthfulness.
Furthermore nothing in this project constitutes a solicitation for
investment.

Any content produced by SL or developer resources that SL provides, are
for educational and inspiration purposes only. SL does not encourage,
induce or sanction the deployment, integration or use of any such
applications (including the code comprising the Solana blockchain
protocol) in violation of applicable laws or regulations and hereby
prohibits any such deployment, integration or use. This includes use of
any such applications by the reader (a) in violation of export control
or sanctions laws of the United States or any other applicable
jurisdiction, (b) if the reader is located in or ordinarily resident in
a country or territory subject to comprehensive sanctions administered
by the U.S. Office of Foreign Assets Control (OFAC), or (c) if the
reader is or is working on behalf of a Specially Designated National
(SDN) or a person subject to similar blocking or denied party
prohibitions.

The reader should be aware that U.S. export control and sanctions laws 
prohibit U.S. persons (and other persons that are subject to such laws) 
from transacting with persons in certain countries and territories or 
that are on the SDN list. Accordingly, there is a risk to individuals 
that other persons using any of the code contained in this repo, or a 
derivation thereof, may be sanctioned persons and that transactions with 
such persons would be a violation of U.S. export controls and sanctions law.
