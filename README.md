<p align="center">
<h1 align="center"><b>Shin</b><br>Another web fuzzer</h1>

---

### Summary

Just another web fuzzer, but this time written in Rust.

### Instalation

- You need to have `cargo` installed: https://rustup.rs
- Paste in terminal:
    ```sh
    git clone https://github.com/LucasKatashi/shin && cd jito && cargo run
    ```
- The `shin` binary will be generated in `target/debug`

---

### Usage

```sh
Usage: shin [OPTIONS] --target <TARGET> --wordlist <WORDLIST>

Options:
  -t, --target <TARGET>
  -w, --wordlist <WORDLIST>
      --threads <THREADS>    [default: 1]
      --mc                   Only displays responses with status-code 200
  -h, --help                 Print help
```

---

### License

This work is licensed under [MIT License.](/LICENSE.md)
