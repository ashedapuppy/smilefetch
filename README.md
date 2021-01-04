# smilefetch
A system info tool written in c

![smilefetch](https://github.com/sudosmile/smilefetch/blob/master/img.png?raw=true)

## TODO:
    - make the square around info always the correct size
    - add ram info and cute colour blocks like neofetch

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
