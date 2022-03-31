# smilefetch
A system info tool written in Rust

## installation:
```sh
git clone "https://github.com/sudosmile/smilefetch.git" && \
    cd smilefetch && \
    cargo install --path .
```

## benchmark:

using [hyperfine](https://github.com/sharkdp/hyperfine):
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `neofetch` | 428.2 ± 13.2 | 382.4 | 477.1 | 519.61 ± 32.10 |
| `smilefetch` | 0.8 ± 0.0 | 0.8 | 1.5 | 1.00 |
