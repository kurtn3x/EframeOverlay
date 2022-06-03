use super::super::App;
use inputbot::KeybdKey;
use std::fmt;
use std::ops::IndexMut;
use strum::IntoEnumIterator;
use winapi::{
    ctypes::*,
    shared::{minwindef::*, windef::*},
    um::winuser::*,
};

#[derive(Clone, Debug)]
pub struct Hotkey<'a> {
    key_state: Vec<i16>,
    activated: Vec<bool>,
    pub key: Vec<inputbot::KeybdKey>,
    pub identifier: &'a str,
    block: bool,
}

impl Default for Hotkey<'_> {
    fn default() -> Hotkey<'static> {
        Hotkey {
            key_state: vec![],
            activated: vec![],
            key: vec![],
            identifier: "Init",
            block: false,
        }
    }
}

impl fmt::Display for Hotkey<'_> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.key)
    }
}

impl Hotkey<'_> {
    /// create a new shortcut available for checking
    /// the identifier is needed to identify the hotkey later
    /// this result should be pushed to a 'Vec<Hotkey>', in your main
    /// programm, you will iterate over this Vector and check each Hotkey
    pub fn new(key: Vec<inputbot::KeybdKey>, identifier: &'static str) -> Hotkey {
        let mut tmp_cr_key_state: Vec<i16> = vec![];
        let mut tmp_cr_activated: Vec<bool> = vec![];
        for item in key.iter() {
            tmp_cr_key_state.push(10);
            tmp_cr_activated.push(false);
        }

        Hotkey {
            key_state: tmp_cr_key_state,
            activated: tmp_cr_activated,
            key: key,
            block: false,
            identifier: identifier,
        }
    }

    /// check if this hotkey is pressed, depending on how many keys are given, this method acts differently
    /// 1 key: return true if the key is released
    /// more than 1 key: return true, when all the keys are activated at the same time for the first time, as soon as one of the keys is released, it can be repressed to return true again.
    pub fn check(&mut self) -> bool {
        // when there are more than 1 keys assigned
        // this will return true as soon as THE ACTIVATIONS OF BOTH KEYS OVERLAP
        // as soon as both key states are active, it will return true and block input
        // until at least 1 of the keys is released
        if self.key.len() > 1 {
            if self.activated.len() == 1 {
                self.activated.push(false);
            }
            for (pos, key) in self.key.iter_mut().enumerate() {
                let temp2 = key.is_pressed();
                self.activated[pos] = temp2;
            }

            // check if the given keys are all activated, if yes set block to true and return true
            if self.block == false {
                let mut ret_checker: usize = 0;
                for boolean in self.activated.iter() {
                    if boolean == &true {
                        ret_checker += 1;
                    }
                }

                if ret_checker == self.key.len() {
                    self.block = true;
                    return true;
                } else {
                    return false;
                }
            }
            // check if any of the keys are released, if yes set block to false
            else {
                let mut ret_checker: usize = 0;
                for boolean in self.activated.iter() {
                    if boolean == &true {
                        ret_checker += 1;
                    }
                }
                if ret_checker != self.key.len() {
                    self.block = false;
                    return false;
                } else {
                    return false;
                }
            }
        // when there is only 1 key assigned, its always at pos0, so why bother checking
        // the key will return true WHEN IT IS RELEASED!!!!
        } else if self.key.len() == 1 {
            let (temp1, temp2) = self.key[0].is_down(self.key_state[0]);
            self.key_state[0] = temp1;
            self.activated[0] = temp2;
            return self.activated[0];
        } else {
            return false;
        }
    }
}

#[derive(Debug)]
struct CaptureKeys {
    keys: Vec<KeybdKey>,
    keystate: Vec<bool>,
}

/// capture any key and return it if pressed
pub fn capture_key() -> Vec<KeybdKey> {
    let mut MyKeys = CaptureKeys {
        keys: vec![],
        keystate: vec![],
    };
    'outer: loop {
        'inner: for key in inputbot::KeybdKey::iter() {
            let state = (unsafe { GetAsyncKeyState(u64::from(key) as i32) } >> 15);
            if state != 0 {
                if !MyKeys.keys.contains(&key) {
                    MyKeys.keys.push(key);
                    MyKeys.keystate.push(true);
                }
            } else if MyKeys.keys.contains(&key) && state == 0 {
                for (i, keys) in MyKeys.keys.iter().enumerate() {
                    if keys == &key {
                        MyKeys.keystate[i] = false;
                    }
                }
            }

            let keystatelen = MyKeys.keystate.len();
            let mut templen = 0;

            for state in MyKeys.keystate.iter_mut() {
                if state == &mut false {
                    templen += 1;
                }
            }

            if keystatelen != 0 && templen == keystatelen {
                return MyKeys.keys;
            }
        }
    }
}

// all the hotkeys and what they are supposed to do
pub fn check_hotkeys(app: &mut App) {
    for hotkey in app.hotkey_settings.all_hotkeys.iter_mut() {
        if hotkey.identifier == "first_hotkey" {
            if hotkey.check() {
                println!("CapsLock + Tab pressed!");
            }
        } else if hotkey.identifier == "hotkey_item_inspection" {
            let status = hotkey.check();
            if status && app.item_inspection_settings.hotkey_item_inspection_pressed == false {
                app.item_inspection_settings.hotkey_item_inspection_pressed = true;
                app.item_inspection_settings
                    .hotkey_item_inspection_pressed_first = true;
            } else if status && app.item_inspection_settings.hotkey_item_inspection_pressed == true
            {
                app.item_inspection_settings.hotkey_item_inspection_pressed = false;
            }
        } else if hotkey.identifier == "hotkey1" {
            if hotkey.check() {
                println!("hotkey1");
            }
        } else if hotkey.identifier == "hotkey2" {
            if hotkey.check() {
                println!("hotkey2");
            }
        } else if hotkey.identifier == "hotkey3" {
            if hotkey.check() {
                println!("hotkey3");
            }
        }
    }
}

/// reinitialize all hotkeys by copying the values from the temp hotkeys into the values of the real hotkeys
/// will be called after changing any hotkey
pub fn reinitialize_hotkeys(app: &mut App) {
    for hotkeys in app.hotkey_settings.all_hotkeys.iter_mut() {
        if hotkeys.identifier == "hotkey_item_inspection" {
            hotkeys.key = app
                .hotkey_settings
                .custom_hotkeys
                .hotkey_item_inspection
                .clone();
        } else if hotkeys.identifier == "hotkey1" {
            hotkeys.key = app.hotkey_settings.custom_hotkeys.hotkey1.clone();
        } else if hotkeys.identifier == "hotkey2" {
            hotkeys.key = app.hotkey_settings.custom_hotkeys.hotkey2.clone();
        } else if hotkeys.identifier == "hotkey3" {
            hotkeys.key = app.hotkey_settings.custom_hotkeys.hotkey3.clone();
        }
    }
    app.hotkey_settings.reinitialize_hotkeys = false;
}
