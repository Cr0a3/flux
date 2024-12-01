# The flux compiler

This reposentory contains the source code for the `flux` compiler

## Why flux?
 - No reasons

## Building flux

This sections teaches you how to build the flux compiler

### Requirements
 - You need to have rust installed
 - You need to have git installed


To build flux you simply need to clone the repo:

```
git clone https://github.com/Cr0a3/flux
```

Then you'll be able to build the flux language:
```
cargo install --git https://github.com/Cr0a3/ygen-backend-gen.git
cargo build --release --all
cargo install --all --path .
```

You now have flux installed on your system! 🎉