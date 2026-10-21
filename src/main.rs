use km::keyboard;
use km::stg::Setting;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let stg = Setting::default_new()?;

    keyboard::listener(|ev| {
        let Some(k) = keyboard::key_name(ev.code) else {
            return false;
        };
        println!("{k}");

        let Some(kb) = stg.get(k) else {
            return false;
        };

        false
    })?;
    Ok(())
}
