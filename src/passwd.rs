use mozdevice::Device;

use rand::thread_rng;
use rand::Rng;

pub struct Passwd {}


impl Passwd {

    fn get_random_char(charset: &[u8]) -> char {
        let index = thread_rng().gen_range(0..charset.len());
        charset[index] as char
    }

    // Generate & push random  password
    pub fn random(device: &Device, length: usize) {

        // Password generation
        // Character list
        let chars: String = concat!(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            "abcdefghijklmnopqrstuvwxyz",
            "0123456789",
            "!@#$%&*"
        ).to_string();

        let charset: &[u8] = &chars.into_bytes();
        let mut keycodes: Vec<u8> = Vec::new();


        // Generate 9999 passwords and push them
        for ret in 1..=1 {
            let passwd: String = (0..length).map(|_| Passwd::get_random_char(&charset)).collect();

            println!("{passwd}");
        }
    }
}