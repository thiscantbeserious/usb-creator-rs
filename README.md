# USB-Creator-RS - 0.0.1

![build status](https://github.com/thiscantbeserious/usb-creator-rs/actions/workflows/rust.yml/badge.svg)

## Status: Not ready yet 

| Linux | MacOS | Windows | FreeBsd |
|-------------|-------------|-------------|-------------|
| lists disks | lists disks | lists disks?|-            |

## Next steps: 
1. Mock implementation for Unit-Tests without USB-Devices
2. Setup of custom Github Runners on Win, Linux, MacOS with actual USB-Sticks 
3. BSD: basic implementation for list_devices
4. Revamp / rethink proper architecture (API & Threading)
5. Implement UI - most likely via Dioxus -> https://dioxuslabs.com/

This is a tool to create an USB-Disk for Linux, MacOS, Windows and even FreeBSD

It does not use old legacy code and a plethora of libraries, but simple standard tools and system-level access to the OS directly.

Currently it does not do anything besides listing external Disks, that is also USB-HDDs and SD-Cards too. 

However I will add an GUI later on too, after the basics have been done.

Ideally this is a single binary that can be used from commandline and GUI without caring about the specific nitty picky OS-Details. 

Unsure - turn this into a Rust crate/library:

Maybe I'm able to make this a re-usable cross-device library / crate too that can serve as a basic package for other use-cases