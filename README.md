# USB-Creator-RS

![build status](https://github.com/thiscantbeserious/usb-creator-rs/actions/workflows/rust.yml/badge.svg)

This is a tool to create an USB-Disk for Linux, MacOS, Windows and even FreeBSD

It does not use old legacy code and a plethora of libraries, but simple standard tools and system-level access to the OS directly.

Currently it does not do anything besides listing external Disks, that is also USB-HDDs and SD-Cards too. 

However I will add an GUI later on too, after the basics have been done.

Ideally this is a single binary that can be used from commandline and GUI without caring about the specific nitty picky OS-Details.