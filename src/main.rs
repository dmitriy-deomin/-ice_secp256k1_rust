use std::time::{Duration, Instant};
use rand::Rng;

mod ice_library;
const  HEX:[&str; 16] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"];
fn main() {

    let mut start = Instant::now();
    let mut speed: u64 = 0;

    let mut rng = rand::thread_rng();

    let ice_library = ice_library::IceLibrary::new();
    ice_library.init_secp256_lib();

    let mut addresa = vec!["".to_string()];
    let mut hex_rand = "".to_string();

    loop {

        addresa.clear();
        hex_rand.clear();

        for _i in 0..64 {
            hex_rand.push_str(HEX[rng.gen_range(0..16)])
        };

        //btc44_u
        addresa.push(ice_library.privatekey_to_coinaddress(hex_rand.as_str(), 0, 0, false));
        //btc44_c
        addresa.push(ice_library.privatekey_to_coinaddress(hex_rand.as_str(), 0, 0, true));
        //btc49
        addresa.push(ice_library.privatekey_to_coinaddress(hex_rand.as_str(), 0, 1, true));
        //btc84
        addresa.push(ice_library.privatekey_to_coinaddress(hex_rand.as_str(), 0, 2, true));

        //eth
        addresa.push(ice_library.privatekey_to_ETH_address(hex_rand.as_str()));

        //ltc_u
        addresa.push(ice_library.privatekey_to_coinaddress(hex_rand.as_str(), 21, 0, false));
        //ltc_c
        addresa.push(ice_library.privatekey_to_coinaddress(hex_rand.as_str(), 21, 0, true));

        //doge_u
        addresa.push(ice_library.privatekey_to_coinaddress(hex_rand.as_str(), 16, 0, false));
        //doge_c
        addresa.push(ice_library.privatekey_to_coinaddress(hex_rand.as_str(), 16, 0, true));

        let public_key_u = ice_library.privatekey_to_publickey(hex_rand.as_str());

        //---------------
        speed = speed + 1;
        if start.elapsed() >= Duration::from_secs(1) {
            println!("--------------------------------------------------------");
            println!("SPEED {speed}/sek", );
            println!("HEX:{}", hex_rand);
            println!("PUBLIC KEY UNCOMPRESSED:{public_key_u}");
            for adress in addresa.iter(){
                println!("ADDRESS:{adress}")
            }
            println!("--------------------------------------------------------");
            start = Instant::now();
            speed = 0;
        }

    }

}
