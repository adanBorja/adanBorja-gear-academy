#![no_std]

#[allow(unused_imports)]
use gstd::{msg,prelude::*,debug,exec,ActorId};
use tamagotchi_nft_io::*;

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

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
            msg::reply(TmgEvent::Fed,0)
                .expect("Error with update fed");
        }
        TmgAction::Entertain=>{
            tamagotchi.play();
            msg::reply(TmgEvent::Entertained, 0)
                .expect("Error update Entertan");
        }
        TmgAction::Sleep=>{
            tamagotchi.get_sleep();
            msg::reply(TmgEvent::Slept, 0)
                .expect("Error update Sleep");
        }
        TmgAction::Transfer(actor_id)=>{
            let source_id = msg::source();
            let mut owner_trasfered = false;
            if tamagotchi.owner == source_id{
                tamagotchi.owner = actor_id;
                owner_trasfered = true;
            }
            if tamagotchi.approved_account == Some(source_id){
                tamagotchi.owner=actor_id;
                owner_trasfered=true;
            }
            if owner_trasfered{
                msg::reply(TmgEvent::Transferred(actor_id), 0)
                .expect("Error in trasnfer reply tamagotchi");
            }
        }
        TmgAction::Approve(actor_id)=>{
            let soruce_id = msg::source();
            if tamagotchi.owner == soruce_id {
                tamagotchi.approved_account = Some(actor_id);
                msg::reply(TmgEvent::Approved(actor_id),0)
                .expect("Error en reply to Approve");
            }
        }
        TmgAction::RevokeApproval=>{
            let source_id = msg::source();
            if tamagotchi.owner == source_id {
                tamagotchi.approved_account=None;
                msg::reply(TmgEvent::ApprovalRevoked,0)
                .expect("Error in reply RevokeApproval");
            }
        }
    }
}

#[no_mangle]
extern fn state() {
    // TODO: 0️⃣ Copy the `handle` function from the previous lesson and push changes to the master branch
    let tamagotchi = unsafe { TAMAGOTCHI.take().expect("Unexpected error in taking state") };
    msg::reply(tamagotchi, 0).expect("Failed to share state");
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
        slept_block:current_birth,
        approved_account: None,
    };
    unsafe{
        TAMAGOTCHI=Some(tamagotchi);
    }
    msg::reply("Successfull initialize of tamagochi", 0)
        .expect("Can't send reply");
}
