// std
use std::env;
use std::process::exit;
use std::convert::TryFrom;

mod npin;
mod passwd;

use crate::passwd::Passwd;
use crate::npin::NPin;


fn print_help() {
    println!("usage: nitroforce [PARAMETERS]... [FILE][TYPE]");
    println!(" args: --from-file    [FILE]");
    println!("       -f             [FILE]");
    println!("       --random       [TYPE] (p4)");
    println!("       -r             [TYPE] (p4)");
    println!("use «nitroforce-rs --help» command to display this message again");
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        print_help();
        std::process::exit(0);
    }
    if args.get(1).unwrap() == &String::from("--help") || args.get(1).unwrap() == &String::from("-h") {
        print_help();
        exit(0);
    }

    println!("This software was developed for educational purposes ONLY.");
    println!("Author do not take any responsibility for his software's users or any unfair/malicious usage.");

    let adb_server = mozdevice::Host { ..Default::default() };
    let connected: Vec<mozdevice::DeviceInfo> = adb_server.devices().unwrap();
    let connected_amount: usize = connected.len();

    if connected_amount <= 0 {
        panic!("No devices connected");
    }

    let active_device_info = connected.get(0).unwrap();

    println!("Connected {connected_amount} devices");
    println!("Active device: {:?} ({:?})", active_device_info.info["device"], active_device_info.info["model"]);

    let active_device = mozdevice::Device::new(
        adb_server,
        active_device_info.serial.to_owned(),
        mozdevice::AndroidStorageInput::Auto
    ).unwrap();


    for i in 0..=args.len() {
        if args[i] == "--random" || args[i] == "-r" {
            if args.get(i + 1) == None {
                print_help();
                exit(0);
            }

            else {
                let arg: u32 = args[i + 1].parse().unwrap();
                let len = usize::try_from(arg).unwrap();

                println!("{arg}");
                println!("{len}");

                NPin::random(&active_device, len);
                exit(0);
            }
        }

        else if args[i] == "--from-file" || args[i] == "-f" {
            if args.get(i + 1) == None {
                print_help();
                exit(0);
            }

            else {
                NPin::from_file(&args[i + 1], &active_device);
                exit(0);
            }
        }

        else if args[i] == "--passwd" || args[i] == "-p" {
            if args.get(i + 1) == None {
                print_help();
                exit(0);
            }

            else {
                Passwd::random(&active_device, args[i + 1].parse::<usize>().unwrap());
                exit(0);
            }
        }
    }
}