use km::keyboard;
use km::stg::Setting;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let stg = Setting::default_new()?;

    keyboard::listener(|ev| match ev.code {
        1 => {
            println!("exit");
            true
        }
        45 => {
            println!("x");
            false
        }

        _ => {
            println!("not x");
            false
        }
    })?;
    Ok(())
}
