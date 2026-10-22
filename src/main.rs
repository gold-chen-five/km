use km::keyboard;
use km::stg::Setting;
use std::error::Error;
use std::process::{Command, Stdio};
use std::thread;

fn main() -> Result<(), Box<dyn Error>> {
    let stg = Setting::default_new()?;

    keyboard::listener(|ev| {
        let Some(k) = keyboard::key_name(ev.code) else {
            return;
        };

        let Some(kb) = stg.get(k) else {
            return;
        };

        let child = match Command::new(&kb.command)
            .args(kb.args.as_deref().unwrap_or(&[]))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(err) => {
                eprintln!("command run fail: {err}");
                return;
            }
        };

        thread::spawn(move || match child.wait_with_output() {
            Ok(out) => {
                println!("{}", String::from_utf8_lossy(&out.stdout));
                eprintln!("{}", String::from_utf8_lossy(&out.stderr));
            }
            Err(err) => {
                eprintln!("thread fail: {err}");
            }
        });
    })?;

    Ok(())
}
