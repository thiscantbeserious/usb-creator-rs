# USB-Creator-RS - 0.0.1

![build status](https://github.com/thiscantbeserious/usb-creator-rs/actions/workflows/rust.yml/badge.svg)

This is a Commandline & GUI Tool to create an Bootable USB-Disk/External-HDD/SD-Card for Linux, MacOS, Windows and maybe even FreeBSD. It is not designed to work with Internal drives to prevent accidental overrides.

## Status: Not ready yet 

<table>
  <th>Feature</th><th>Linux</th><th>MacOS</th><th>Windows</th><th>FreeBSD</th>
  <tr>
    <th>List Disks</th>
    <td>&check;</td>
    <td>&check;</td>
    <td>&check;</td>
    <td>5%</td>
  </tr>
  <tr>
    <th>Write Image</th>
    <td>5%</td>
    <td>5%</td>
    <td>5%</td>
    <td>5%</td>
  </tr>  
  <tr>
    <th>Unit Tests</th>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
  </tr>
  <tr>
    <th>Integration Tests</th>
    <td>20%</td>
    <td>20%</td>
    <td>20%</td>
    <td>0%</td>
  </tr>     
  <tr>
    <th>Request Elevated Permissions</th>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
  </tr>
  <tr>
    <th>Ensure Drive not Mounted</th>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
  </tr>
  <tr>
    <th>GUI</th>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
    <td>0%</td>
  </tr>  
</table>

## Next steps: 
1. Mock implementation for Unit-Tests without USB-Devices
2. Setup of custom Github Runners on Win, Linux, MacOS with actual USB-Sticks 
3. BSD: basic implementation for list_devices
4. Revamp / rethink proper architecture (API & Threading)
5. Implement UI - most likely via Dioxus -> https://dioxuslabs.com/

It does not use old legacy code and a plethora of libraries, but simple standard tools and system-level access to the OS directly.

Currently it does not do anything besides listing external Disks, that is also USB-HDDs and SD-Cards too. 

However I will add an GUI later on too, after the basics have been done.

Ideally this is a single binary that can be used from commandline and GUI without caring about the specific nitty picky OS-Details. 

Unsure - turn this into a Rust crate/library:

Maybe I'm able to make this a re-usable cross-device library / crate too that can serve as a basic package for other use-cases