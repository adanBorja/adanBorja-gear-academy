#![no_std]

use codec::{Decode, Encode};
use gmeta::{In, InOut, Metadata, Out};
//use gmeta::Metadata;
use gstd::{exec, prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]

pub struct Tamagotchi{
    // TODO: 0️⃣ Copy fields from previous lesson and push changes to the master branch
    pub name: String,
    pub date_of_birth: u64,
    // TODO: 1️⃣ Add new fields
    pub owner: ActorId,
    pub fed: u64,
    pub fed_block: u64,
    pub entertained: u64,
    pub entertained_block: u64,
    pub slept: u64,
    pub slept_block: u64,
}

const HUNGER_PER_BLOCK:u64 = 1;          // - how much Tamagotchi becomes hungry for the block (becomes less fed); 
const BOREDOM_PER_BLOCK:u64 = 2;         // - how bored Tamagotchi gets per block (becomes less entertained); 
const ENERGY_PER_BLOCK:u64 = 2;          // - how much Tamagotchi loses energy per block (becomes less slept); 
const FILL_PER_FEED:u64 = 1000;          // - how much Tamagotchi becomes full during feeding; 
const FILL_PER_ENTERTAINMENT:u64 = 1000; // - how much Tamagotchi becomes happy during entertainment; 
const FILL_PER_SLEEP:u64 = 1000;         // - how much energy Tamagotchi gets per sleep.

impl Tamagotchi {
    pub fn feed(&mut self){
        let hungry:u64 = HUNGER_PER_BLOCK * ((exec::block_timestamp() - self.fed_block) / 10000); //currentry hungry
        if hungry > FILL_PER_FEED {
            self.fed = FILL_PER_FEED;
        } 
        else {
            self.fed += FILL_PER_FEED - hungry;
        }
        self.fed_block=exec::block_timestamp();
    }     
    pub fn play(&mut self){
        let boredom_tmp:u64 = BOREDOM_PER_BLOCK * ((exec::block_timestamp() - self.entertained_block) / 1000);
        if boredom_tmp > FILL_PER_ENTERTAINMENT{
            self.entertained = FILL_PER_ENTERTAINMENT;
        }
        else{
            self.entertained += FILL_PER_ENTERTAINMENT - boredom_tmp;
        }
        self.entertained_block=exec::block_timestamp();
    }
    pub fn get_sleep(&mut self){
        let energy_lost:u64 = ENERGY_PER_BLOCK * ((exec::block_timestamp() - self.slept_block)/1000);
        if energy_lost > FILL_PER_SLEEP{
            self.slept = FILL_PER_SLEEP;
        }   
        else{
            self.slept += FILL_PER_SLEEP - energy_lost;
        }    
        self.slept_block=exec::block_timestamp();
    }
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgAction {
    // TODO: 0️⃣ Copy actions from previous lesson and push changes to the master branch
    Name,
    Age,
    // TODO: 2️⃣ Add new actions
    Feed,
    Entertain,
    Sleep,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgEvent {
    // TODO: 0️⃣ Copy events from previous lesson and push changes to the master branch
    Name(String),
    Age(u64),
    // TODO: 3️⃣ Add new events
    Fed(u64),
    Entertained(u64),
    Slept(u64),
}

pub struct ProgramMetadata;

// TODO: 0️⃣ Copy `Metadata` from the first lesson and push changes to the master branch
impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Handle = InOut<TmgAction, TmgEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<Tamagotchi>;
}
