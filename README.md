# bp

An interactive CLI to submit Slurm jobs to Blue Pebble, based on [LaurenceA/infrastructure](https://github.com/LaurenceA/infrastructure).
The CLI automatically chooses [the recommended system memory and CPUs per GPU for CNU nodes](https://github.com/LaurenceA/infrastructure?tab=readme-ov-file#choosing-different-cards-and-the-corresponding-recommended-cpumemory-resources-cnu-nodes-only) for the least powerful GPU type selected.

The first time you run `bp`, it will ask you for an HPC project code (Slurm account) and queue name (Slurm partition).
These are saved to `.bp.json` in your home directory and used as default values from then on, along with most other inputs.

## Instructions

Download a prebuilt binary from the [releases section](https://github.com/tim-lawson/bp/releases) and add it to your PATH.

Alternatively, install Rust, if you haven't already:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the repository:

```sh
git clone git@github.com:tim-lawson/bp.git
```

Build the project:

```sh
cd bp
cargo build --release
```

Add the binary to your PATH:

```sh
export PATH="$PATH:/path/to/bp/target/release"
```

Then, you can run the binary:

```sh
bp
```

## Roadmap

- [ ] Support other command-line arguments
- [ ] Support interactive and array jobs
- [ ] Make job names optional
- [x] Save other default values to `.bp.json`
