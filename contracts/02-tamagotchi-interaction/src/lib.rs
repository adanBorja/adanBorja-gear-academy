#![no_std]

#[allow(unused_imports)]
use gstd::{msg,prelude::*,debug,exec,ActorId};
use tamagotchi_interaction_io::*;

static mut TAMAGOTCHI: Option<Tamagotchi> = None;


// TODO: 4️⃣ Define constants

#[no_mangle]
extern fn handle() {
    // TODO: 0️⃣ Copy the `handle` function from the previous lesson and push changes to the master branch
    let input_message:TmgAction=msg::load()
        .expect("Error in loading TmgAction");
    let tamagotchi = unsafe { 
        TAMAGOTCHI.get_or_insert(Default::default()) 
    };
    match input_message{
        TmgAction::Name=>{
            msg::reply(TmgEvent::Name(tamagotchi.name.clone()),0)
                .expect("Error in sending Name");
        }
        TmgAction::Age=>{
            msg::reply(TmgEvent::Age(tamagotchi.date_of_birth), 0)
                .expect("Error in sendig Age");
        }
        TmgAction::Feed=>{
            tamagotchi.feed();
            msg::reply(TmgEvent::Fed(tamagotchi.fed),0)
                .expect("Error with update fed");
        }
        TmgAction::Entertain=>{
            tamagotchi.play();
            msg::reply(TmgEvent::Entertained(tamagotchi.entertained), 0)
                .expect("Error update Entertan");
        }
        TmgAction::Sleep=>{
            tamagotchi.get_sleep();
            msg::reply(TmgEvent::Slept(tamagotchi.slept), 0)
                .expect("Error update Sleep");
        }

    }
    // TODO: 5️⃣ Add new logic for calculating the `fed`, `entertained` and `slept` levels
}

#[no_mangle]
extern fn init() {
    // TODO: 0️⃣ Copy the `init` function from the previous lesson and push changes to the master branch
    let tamagochi_name:String=msg::load()
        .expect("Can't decode an init message");
    let current_birth = exec::block_timestamp();
    let tamagotchi = Tamagotchi{
        name: tamagochi_name,
        date_of_birth: current_birth,
        owner:msg::source(),
        fed:50,
        fed_block: current_birth,
        entertained: 50,
        entertained_block: current_birth,
        slept: 50,
        slept_block:current_birth
    };
    unsafe{
        TAMAGOTCHI=Some(tamagotchi);
    }
    msg::reply("Successfull initialize of tamagochi", 0)
        .expect("Can't send reply");
}

#[no_mangle]
extern fn state() {
    // TODO: 0️⃣ Copy the `handle` function from the previous lesson and push changes to the master branch
    let tamagotchi = unsafe { TAMAGOTCHI.take().expect("Unexpected error in taking state") };
    msg::reply(tamagotchi, 0).expect("Failed to share state");
}