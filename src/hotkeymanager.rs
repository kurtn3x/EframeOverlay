use std::ops::IndexMut;


pub struct Hotkey{
    key_state : Vec<i16>,
    activated : Vec<bool>,
    key: Vec<inputbot::KeybdKey>,
    pub identifier: &'static str,
    block: bool,
}

impl Hotkey{

    pub fn new(key: Vec<inputbot::KeybdKey>, identifier: &'static str) -> Hotkey{
        let mut tmp_cr_key_state: Vec<i16> = vec![];
        let mut tmp_cr_activated: Vec<bool> = vec![];

        for (i,p) in key.iter().enumerate(){
            tmp_cr_key_state.push(10);
            tmp_cr_activated.push(false);

        }

        Hotkey{key_state: tmp_cr_key_state,
            activated: tmp_cr_activated,
            key: key, block:false, identifier: identifier}
    }

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