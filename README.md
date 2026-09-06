# Chaospass

Secure, fast, and lightweight CLI password generator written in Rust.

Chaospass supports dual-mode generation, memory-zeroing security, and automated clipboard management to keep your credentials safe.

---

## Features

* **Dual-Mode Generation**:
  * **Chaos Mode**: Generates high-entropy random strings with customizable length and character sets.
  * **Diceware Mode**: Generates secure, memorable passphrases using the official EFF large wordlist embedded directly into the binary.
* **Security First**: Uses memory-zeroing ([`zeroize`](https://crates.io)) to clear sensitive data from RAM immediately after use.
* **Clipboard Integration**: Automatically copies generated passwords to your system clipboard with a secure timeout.

---

## Installation

Make sure you have [Rust and Cargo](https://rust-lang.org) installed, then clone and build the project:

```bash
# Clone the repository
git clone https://github.com
cd chaospass

# Build the release binary
cargo build --release
```

---

## Usage

Run the compiled binary from the project root:

### Chaos Mode (default length: 16)
```bash
./target/release/chaospass --mode chaos --length 20
```

### Diceware Mode (default: 5 words)
```bash
./target/release/chaospass --mode diceware --words 5
```

---

## License

This project is licensed under the Apache License, Version 2.0. See the [LICENSE](LICENSE) file for details.




<img width="1176" height="912" alt="avatar" src="https://github.com/user-attachments/assets/dbb963e8-4b51-4545-9d6d-0a8ac7b75466" />



