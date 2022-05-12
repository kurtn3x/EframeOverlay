use std::ops::IndexMut;
use std::fmt;
use inputbot::KeybdKey;
use strum::IntoEnumIterator;

use crate::app::TemplateApp; // 0.17.1

#[derive(Clone)]
pub struct Hotkey<'a>{
    key_state : Vec<i16>,
    activated : Vec<bool>,
    pub key: Vec<inputbot::KeybdKey>,
    pub identifier: &'a str,
    block: bool,
}


impl Default for Hotkey <'_>{
    fn default() -> Hotkey <'static>{
        Hotkey {
            key_state : vec![],
            activated : vec![],
            key: vec![],
            identifier: "Init",
            block: false,
        }
    }
}

impl fmt::Display for Hotkey <'_>{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.key)
    }
}

impl Hotkey <'_>{
    /// create a new shortcut available for checking
    /// the identifier is needed to identify the hotkey later
    /// this result should be pushed to a 'Vec<Hotkey>', in your main 
    /// programm, you will iterate over this Vector and check each Hotkey
    pub fn new(key: Vec<inputbot::KeybdKey>, identifier: &'static str) -> Hotkey{
        let mut tmp_cr_key_state: Vec<i16> = vec![];
        let mut tmp_cr_activated: Vec<bool> = vec![];
        for item in key.iter(){
            tmp_cr_key_state.push(10);
            tmp_cr_activated.push(false);
        }

        Hotkey{key_state: tmp_cr_key_state,
            activated: tmp_cr_activated,
            key: key, block:false, identifier: identifier}
    }

    /// check if this hotkey is pressed, depending on how many keys are given, this method acts differently 
    /// 1 key: return true if the key is released 
    /// more than 1 key: return true, when all the keys are activated at the same time for the first time, as soon as one of the keys is released, it can be repressed to return true again.
    pub fn check(&mut self) -> bool{
        // when there are more than 1 keys assigned 
        // this will return true as soon as THE ACTIVATIONS OF BOTH KEYS OVERLAP
        // as soon as both key states are active, it will return true and block input
        // until at least 1 of the keys is released
        if self.key.len() > 1{
            for (pos, key) in self.key.iter_mut().enumerate(){
                let temp2 = key.is_pressed();
                self.activated[pos] = temp2;
            }

            // check if the given keys are all activated, if yes set block to true and return true
            if self.block == false {
                let mut ret_checker: usize = 0;
                for boolean in self.activated.iter(){
                    if boolean == &true{
                        ret_checker += 1;
                    }
                }

                if ret_checker == self.key.len(){
                    self.block = true;
                    return true
                } else {
                    return false
                }
            }
            // check if any of the keys are released, if yes set block to false
            else{
                let mut ret_checker: usize = 0;
                for boolean in self.activated.iter(){
                    if boolean == &true{
                        ret_checker += 1;
                    }
                }
                if ret_checker != self.key.len(){
                    self.block = false;
                    return false
                } else {
                    return false
                }

            } 
        // when there is only 1 key assigned, its always at pos0, so why bother checking
        // the key will return true WHEN IT IS RELEASED!!!!
        } else if self.key.len() == 1 {
            let (temp1, temp2) = self.key[0].is_down(self.key_state[0]);
            self.key_state[0] = temp1;
            self.activated[0] = temp2;
            return self.activated[0]

        }else {
            return false;
        }

    }
}


/// capture any key and return it if pressed
pub fn capture_key() -> KeybdKey{
    'outer: loop{
        'inner: for key in inputbot::KeybdKey::iter(){
            if key.is_pressed(){
                return key;
            }
        }
    }
}


/// all the hotkeys and what they are supposed to do
pub fn check_hotkeys(app : &mut TemplateApp) {
    for hotkey in app.my_hotkeys.all_hotkeys.iter_mut(){
        if hotkey.identifier == "first_hotkey"{
            if hotkey.check(){
                println!("CapsLock + Tab pressed!");
            }
        } else if hotkey.identifier == "hotkey_item_inspection"{
            let status = hotkey.check();
            if status && app.item_inspection_settings.hotkey_item_inspection_pressed == false {
                app.item_inspection_settings.hotkey_item_inspection_pressed = true;
                app.item_inspection_settings.hotkey_item_inspection_pressed_first = true;
            } else if status && app.item_inspection_settings.hotkey_item_inspection_pressed == true {
                app.item_inspection_settings.hotkey_item_inspection_pressed = false;
            } 
        }
    }
}