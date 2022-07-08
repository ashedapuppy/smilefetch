# Smilefetch

<!--toc:start-->

- [installation:](#installation)
- [benchmark:](#benchmark)
<!--toc:end-->

A system info tool written in Rust inspired by neofetch

## To-do:

- [ ] Rewrite the project to be asynchronous

## Installation:

```sh
git clone "https://github.com/sudosmile/smilefetch.git" && \
    cd smilefetch && \
    cargo install --path .
```

## Benchmark:

using [hyperfine](https://github.com/sharkdp/hyperfine):
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `neofetch` | 220.8 ± 9.9 | 204.5 | 253.0 | 2.06 ± 0.28 |
| `smilefetch` | 107.3 ± 13.9 | 69.3 | 158.9 | 1.00 |

benchmark ran on my old decrepit laptop, expect better performance depending on your IO read speeds
