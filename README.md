# smilefetch
A system info tool written in c

![smilefetch](https://github.com/sudosmile/smilefetch/blob/master/smile.png?raw=true)

## TODO:
    - make the square around info always the correct size
    - add ram info

## Why this project?

I was dissatisfied with the system info tools that were available, especially neofetch, as the use of bash
greatly slows them down : almost a second to retrieve the necessary information for neofetch, simply unnacceptable.
Some tools exist that accomplish the same job as smilefetch but most seem to still rely on executing bash commands and
printing their output.

With this program, I try to rely on shell commands as little as possible in order to increase the performance.
Most of the information is gathered through linux files.

I hope you all have a wonderful day, whether you choose to use smilefetch or not, and remember to smile !

## installation

compile the program:
```sh
$ make
```

install it:
```sh
$ sudo make install
```

uninstalling:
```sh
$ sudo make uninstall
```

## usage

simply run <code>$ smilefetch</code> and enjoy the efficiency

## install path
<code>$ sudo make install</code> copies smilefetch to /usr/local/smilefetch by default, feel free to modify the INSTALL_DEST variable in the makefile to change that

## average speed
speed tests done with [hyperfine](https://github.com/sharkdp/hyperfine)

    Benchmark #1: neofetch --off --disable packages
        Time (mean ± σ):     157.6 ms ±   3.0 ms    [User: 69.0 ms, System: 21.4 ms]
        Range (min … max):   153.5 ms … 169.5 ms    100 runs

    Benchmark #2: smilefetch
        Time (mean ± σ):       0.9 ms ±   0.0 ms    [User: 0.4 ms, System: 0.0 ms]
        Range (min … max):     0.8 ms …   1.2 ms    2481 runs

    Summary
        'smilefetch' ran
        185.31 ± 10.36 times faster than 'neofetch --off --disable packages'
