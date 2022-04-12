
use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let hm1_solar = LED::new(21); //21
    let hm1_battery = LED::new(20); //20
    let hm1_fossil = LED::new(16); //16
    let hm2_solar = LED::new(12); //12
    let hm2_battery = LED::new(7); //07
    let hm2_fossil = LED::new(8); //08
    let hm3_solar = LED::new(25); //25
    let hm3_battery = LED::new(24); //24
    let hm3_fossil = LED::new(23); //23

    let sys1_solar = LED::new(18); //18
    let sys1_battery = LED::new(15); //15
    let sys2_solar = LED::new(14); //14
    let sys2_battery = LED::new(26); //26

    loop {
        hm1_solar.on();
        hm1_battery.on();
        hm1_fossil.on();
        sleep(Duration::from_secs(1));
        hm1_solar.off();
        hm1_battery.off();
        hm1_fossil.off();
        sleep(Duration::from_secs(1));


        hm2_solar.on();
        hm2_battery.on();
        hm2_fossil.on();
        sleep(Duration::from_secs(1));
        hm2_solar.off();
        hm2_battery.off();
        hm2_fossil.off();
        sleep(Duration::from_secs(1));


        hm3_solar.on();
        hm3_battery.on();
        hm3_fossil.on();
        sleep(Duration::from_secs(1));
        hm3_solar.off();
        hm3_battery.off();
        hm3_fossil.off();
        sleep(Duration::from_secs(1));


        sleep(Duration::from_secs(1));
        sys1_solar.on();
        sleep(Duration::from_secs(1));
        sys1_solar.off();
        sleep(Duration::from_secs(1));
        sys1_battery.on();
        sleep(Duration::from_secs(1));
        sys1_battery.off();
        sleep(Duration::from_secs(1));
        sys2_solar.on();
        sleep(Duration::from_secs(1));
        sys2_solar.off();
        sleep(Duration::from_secs(1));
        sys2_battery.on();
        sleep(Duration::from_secs(1));
        sys2_battery.off();
        
    }



}

