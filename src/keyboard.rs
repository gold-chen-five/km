use crate::stg::{KeyBind, Setting};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::mem::size_of;
use std::time::{Duration, Instant};

const KEYBOARD_SIZE: usize = 125;

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
    holding_table: [bool; KEYBOARD_SIZE],
    hold_text: Vec<String>,
}

impl Keyboard {
    const EV_KEY: u16 = 0x01;

    const KEY_RELEASE: i32 = 0;
    const KEY_PRESS: i32 = 1;
    const KEY_REPEAT: i32 = 2;

    pub fn new(sequence_timeout: Duration) -> Self {
        Self {
            sequence: Sequence {
                key: String::new(),
                prev_press: Instant::now(),
                timeout: sequence_timeout,
            },
            holding_table: [false; KEYBOARD_SIZE],
            hold_text: Vec::new(),
        }
    }

    pub fn listener<F: FnMut(&KeyBind)>(&mut self, mut f: F) -> Result<(), Box<dyn Error>> {
        let mut file = File::open("/dev/input/event3")?; // Need to fix by kb find
        let mut buffer = [0u8; size_of::<libc::input_event>()];

        loop {
            file.read_exact(&mut buffer)?;
            let ev: libc::input_event = unsafe { std::ptr::read_unaligned(buffer.as_ptr().cast()) };

            let stg = Setting::default_new()?;
            let kb = match self.ev_to_kb(&stg, &ev) {
                Ok(kb) => kb,
                Err(err) => {
                    continue;
                }
            };

            f(kb);
            self.sequence.key.clear();
        }
    }

    fn ev_to_kb<'a>(
        &mut self,
        stg: &'a Setting,
        ev: &libc::input_event,
    ) -> Result<&'a KeyBind, String> {
        if ev.type_ != Self::EV_KEY {
            return Err(String::from("Not keyboard"));
        }

        let Some(k) = Self::key_name(ev.code) else {
            return Err(String::from("Didn't find the key name"));
        };

        let Some(key) = (match ev.value {
            Self::KEY_RELEASE => {
                self.holding_table[usize::from(ev.code)] = false;
                if let Some(index) = self.hold_text.iter().position(|item| item == k) {
                    self.hold_text.remove(index);
                }
                None
            }
            Self::KEY_PRESS => {
                match self.holding_table.get(usize::from(ev.code)) {
                    Some(true) => {
                        return Err(String::from(
                            "This key is holding, don't process the hold again.",
                        ));
                    }
                    Some(false) => {}
                    None => {
                        return Err(String::from("No holding state for this key."));
                    }
                }
                let k = self.sequence.push(k);
                Some(k.to_string())
            }
            Self::KEY_REPEAT => {
                match self.holding_table.get(usize::from(ev.code)) {
                    Some(true) => {
                        return Err(String::from(
                            "This key is holding, don't process the hold again.",
                        ));
                    }
                    Some(false) => {}
                    None => {
                        return Err(String::from("No holding state for this key."));
                    }
                }

                if let Some(h) = self.holding_table.get_mut(usize::from(ev.code)) {
                    *h = true;
                } else {
                    return Err(String::from("Didn't find the key you hold."));
                }

                self.hold_text.push(k.to_string());
                None
            }
            _ => None,
        }) else {
            return Err(String::from("ev.value: didn't have this ev value"));
        };
        if self.hold_text.is_empty() {
            stg.get(&key)
                .ok_or_else(|| String::from("Can't find the setting key"))
        } else {
            stg.get_hold(&self.hold_text, &key)
                .ok_or_else(|| String::from("Can't find the setting key"))
        }
    }

    // this follow <https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h>
    const fn key_name(code: u16) -> Option<&'static str> {
        Some(match code {
            1 => "<Esc>",
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
            14 => "<Backspace>",
            15 => "<Tab>",
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
            28 => "<Enter>",
            29 => "<Ctrl>",
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
            42 => "<Shift>",
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
            54 => "<Shift>",
            56 => "<Alt>",
            57 => "<Space>",
            58 => "<CapsLock>",
            59 => "<F1>",
            60 => "<F2>",
            61 => "<F3>",
            62 => "<F4>",
            63 => "<F5>",
            64 => "<F6>",
            65 => "<F7>",
            66 => "<F8>",
            67 => "<F9>",
            68 => "<F10>",
            87 => "<F11>",
            88 => "<F12>",
            97 => "<Ctrl>",
            100 => "<Alt>",
            102 => "<Home>",
            103 => "<Up>",
            104 => "<PageUp>",
            105 => "<Left>",
            106 => "<Right>",
            107 => "<End>",
            108 => "<Down>",
            109 => "<PageDown>",
            110 => "<Insert>",
            111 => "<Delete>",
            125 => "<Meta>",
            _ => return None,
        })
    }
}
