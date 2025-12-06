#!/bin/bash
cargo build --release
sudo -E ./target/release/keyboard-display-utility
