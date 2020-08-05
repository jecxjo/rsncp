# rsncp / Rust Network Copy

This project is a port of Pyncp which is a port of the original 
"ncp - a fast file copy tool for LANs" originally written by Felix von
Leitner <felix-ncp@fefe.de>

Installation
------------

Install rust

    $ cargo build --release

Quick Start
-----------

If you want to send a file, directory or group of files/directories, start
the listener on the destination machine:

    $ rsncp

To send a file or directory directly:

    $ rsncp send 192.168.1.2 file.txt ./directory

Supports IPV6

    $ rsncp send "[::1]" file.txt

To send files and directories without knowing IP Addresses or Hostnames 
setup a Multicast/Broadcast Poll on the destination machine:

    $ rsncp poll

To send via Multicast/Broadcast use a push:

    $ rsncp push file.txt ./directory

For legacy support (no compression) to work with the original ncp, use the `-l`
flag in all commands.

    $ rsncp listen -l
    $ rsncp send -l localhost test.txt
    $ rsncp poll -l
    $ rsncp push -l test.txt

Credits
-------
Thanks to [Felix](http://www.fefe.de/ncp/) and
[makefu](https://github.com/makefu/pyncp)
