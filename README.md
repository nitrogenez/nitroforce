<h2 align=center>
  <img align=center src=icon.webp width=30% alt=Icon></img>
</h2>

<h1 align=center><b>NITROFORCE</b></h1>


## **DESCRIPTION**
**NITROFORCE** picks numbers in range 0..9 from pin list and passes 4-digit combination into Android smartphone through **adb**.

> **NOTICE**  
> This software was developed for educational purposes ONLY.  
Author do not take any responsibility for his software's users or any unfair/malicious usage.


## **BUILDING**
  + Install Rust and Cargo (e.g via rustup)
    - **Arch-based distributions:** `sudo pacman -S rust`
  + Build project: `cargo build --release`


## **ADB INSTALLATION**
  + **Arch-based distributions**
    - `sudo pacman -S android-tools`
  + **Debian-based distributions**
    - `sudo apt-get install android-tools-adb`
  + **MacOS**: See [MACOSX_INSTALLATION](MACOSX_INSTALLATION.md)


## **QUICK GUIDE**
+ Enable USB-Debugging on your Android phone
+ **Linux/MacOS X**
  - Clone or download repository: `git clone https://github.com/nitrogenez/nitroforce`
  - Connect target device to your machine via USB
  - Goto `nitroforce` directory: `cd /path/to/nitroforce`
  - Build project


## **EXAMPLE**
```bash
git clone https://github.com/nitrogenez/nitroforce
cd ~/nitroforce
cargo build --release
./target/release/nitroforce
```

## **LICENSE**
**This software is licensed under GNU Affero General Public License v3-or-later.**
See [LICENSE.md](LICENSE.md) for further details.


<h2 align=center>ILLGIVEYOUNITROGENESIS</h2>