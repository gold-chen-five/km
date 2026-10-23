use crate::stg::{KeyBind, Setting};
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::mem::size_of;
use std::time::{Duration, Instant};

struct Sequence {
    key: String,
    prev_press: Instant,
    timeout: Duration,
}

impl Sequence {
    fn push(&mut self, k: &str) -> &str {
        let now = Instant::now();
        if now - self.prev_press > self.timeout {
            self.key.clear();
        }
        self.key.push_str(k);
        self.prev_press = now;
        &self.key
    }
}
pub struct Keyboard {
    sequence: Sequence,
}

impl Keyboard {
    // const KEY_RELEASE: i3 = 0;
    const KEY_PRESS: i32 = 1;
    // const KEY_REPEAT: i32 = 2;
    const EV_KEY: u16 = 0x01;

    pub fn new(sequence_timeout: Duration) -> Self {
        Self {
            sequence: Sequence {
                key: String::new(),
                prev_press: Instant::now(),
                timeout: sequence_timeout,
            },
        }
    }

    pub fn listener<F: FnMut(&KeyBind)>(&mut self, mut f: F) -> Result<(), Box<dyn Error>> {
        let mut file = File::open("/dev/input/event4")?; // Need to fix by kb find
        let mut buffer = [0u8; size_of::<libc::input_event>()];
        let stg = Setting::default_new()?;

        loop {
            file.read_exact(&mut buffer)?;
            let ev: libc::input_event = unsafe { std::ptr::read_unaligned(buffer.as_ptr().cast()) };
            if !(ev.type_ == Self::EV_KEY && ev.value == Self::KEY_PRESS) {
                continue;
            }

            let Some(k) = self.key_name(ev.code) else {
                continue;
            };

            let key = self.sequence.push(k);
            let Some(kb) = stg.get(&key) else {
                continue;
            };

            f(kb);
            self.sequence.key.clear();
        }

        Ok(())
    }

    // this follow <https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h>
    const fn key_name(&self, code: u16) -> Option<&'static str> {
        Some(match code {
            1 => "Esc",
            2 => "1",
            3 => "2",
            4 => "3",
            5 => "4",
            6 => "5",
            7 => "6",
            8 => "7",
            9 => "8",
            10 => "9",
            11 => "0",
            12 => "-",
            13 => "=",
            14 => "Backspace",
            15 => "Tab",
            16 => "q",
            17 => "w",
            18 => "e",
            19 => "r",
            20 => "t",
            21 => "y",
            22 => "u",
            23 => "i",
            24 => "o",
            25 => "p",
            26 => "[",
            27 => "]",
            28 => "Enter",
            29 => "LeftCtrl",
            30 => "a",
            31 => "s",
            32 => "d",
            33 => "f",
            34 => "g",
            35 => "h",
            36 => "j",
            37 => "k",
            38 => "l",
            39 => ";",
            40 => "'",
            41 => "`",
            42 => "LeftShift",
            43 => "\\",
            44 => "z",
            45 => "x",
            46 => "c",
            47 => "v",
            48 => "b",
            49 => "n",
            50 => "m",
            51 => ",",
            52 => ".",
            53 => "/",
            54 => "RightShift",
            56 => "LeftAlt",
            57 => "Space",
            58 => "CapsLock",
            59 => "F1",
            60 => "F2",
            61 => "F3",
            62 => "F4",
            63 => "F5",
            64 => "F6",
            65 => "F7",
            66 => "F8",
            67 => "F9",
            68 => "F10",
            87 => "F11",
            88 => "F12",
            97 => "RightCtrl",
            100 => "RightAlt",
            102 => "Home",
            103 => "Up",
            104 => "PageUp",
            105 => "Left",
            106 => "Right",
            107 => "End",
            108 => "Down",
            109 => "PageDown",
            110 => "Insert",
            111 => "Delete",
            125 => "LeftMeta",
            _ => return None,
        })
    }
}
