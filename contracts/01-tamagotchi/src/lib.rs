#![no_std]

#[allow(unused_imports)]
use gstd::{msg,prelude::*,debug,exec};
//use scale_info::named_type_params;
use tamagotchi_io::*;
 
static mut TAMAGOTCHI:Tamagotchi=Tamagotchi{
    name:String::new(),
    date_of_birth: 0,
};

#[no_mangle]
extern fn handle() {
    // TODO: 6️⃣ Add handling of `Name` and `Age` actions
    let input_message:TmgAction=msg::load()
        .expect("Error in loading TmgAction");
    let tamagotchi=Tamagotchi{
        name: unsafe {TAMAGOTCHI.name.clone()},
        date_of_birth: unsafe {TAMAGOTCHI.date_of_birth.clone()},
    };
    match input_message{
        TmgAction::Name=>{
            msg::reply(tamagotchi.name,0)
                .expect("Error in sending Name");
        }
        TmgAction::Age=>{
            msg::reply(tamagotchi.date_of_birth, 0)
                .expect("Error in sendig Age");
        }
    }
}

#[no_mangle]
extern fn init() {
    // TODO: 5️⃣ Initialize the Tamagotchi program
    let tamagochi_name:String=msg::load()
        .expect("Can't decode an init message");
    let current_birth = exec::block_timestamp();
    let tamagotchi = Tamagotchi{
        name: tamagochi_name,
        date_of_birth: current_birth,
    };
    unsafe{
        TAMAGOTCHI=tamagotchi;
    }
    msg::reply("Successfull initialize of tamagochi", 0)
        .expect("Can't send reply");
}

#[no_mangle]
extern fn state() {
    // TODO: 7️⃣ Return the Tamagotchi state
    let tamagotchi=Tamagotchi{
        name: unsafe {TAMAGOTCHI.name.clone()},
        date_of_birth: unsafe {TAMAGOTCHI.date_of_birth.clone()},
    };
    msg::reply(tamagotchi, 0).expect("Failed to share state");
}