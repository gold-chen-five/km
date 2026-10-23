use km::keyboard::Keyboard;
use std::error::Error;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut keyboard = Keyboard::new(Duration::from_millis(300));

    keyboard.listener(|kb| {
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
