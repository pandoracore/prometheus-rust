[![Build Status](https://travis-ci.org/pandoracore/prometheus-rust.svg?branch=master)](https://travis-ci.org/pandoracore/prometheus-rust) [![Codacy Badge](https://api.codacy.com/project/badge/Grade/6289725dbd8d4751b3fa8180e962c185)](https://www.codacy.com/app/pandoracore/prometheus-rust?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=pandoracore/prometheus-rust&amp;utm_campaign=Badge_Grade) [![Coverage Status](https://coveralls.io/repos/github/pandoracore/prometheus-rust/badge.svg?branch=master)](https://coveralls.io/github/pandoracore/prometheus-rust?branch=master)

# Prometheus library on Rust

Rust implementation of Prometheus protocol. For protocol specification please check 
<https://github.com/pandoracore/prometheus-spec>

`prometheus-rust` is written in "Rust"; "Cargo" is its build system and package manager.

## Install "Rust" and "Cargo"

Follow the instructions in [Rust Install](https://www.rust-lang.org/en-US/install.html)
For those who use "macOS" it is possible to install "Rust" through `brew`:

`$ brew install rust`

## Build `prometheus-rust`

`$ cargo build`

## Run the tests

`$ cargo test --package prometheus --lib tests`
