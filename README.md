# Basic RTL-SDR P2000 semaphone decoder for Raspberry Pi

This Rust program assumes that `rtl_fm` and `multimon-ng` are already installed. I am sorry, but this README does not show how to do this...

This is a simplified version of https://github.com/Schrolli91/BOSWatch, mainly for my learning, but rewritten in Rust instead of Python

It will acutally do more or less the same as the following command line:
    rtl_fm -f 169.65M -M fm -s 22050 | multmon-ng -a FLEX -t raw /dev/stdin"

The program will start rtl_fm as a subprocess and will pipe the output to the multimon_ng subprocess, which does do the actual decoding.