use gtest::{Log,Program, System};
use tamagotchi_interaction_io::*;

// TODO: 0️⃣ Copy tests from the previous lesson and push changes to the master branch
#[test]
fn interaction_test(){
//fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    // TODO: 8️⃣ Test the program initialization and message handling
     
    let name_tmg:String=String::from("Tamagito");
    let date_of_birth_test = sys.block_timestamp();
    let res = program.send(2,name_tmg.clone());
    assert!(!res.main_failed());
    
    let expected_log = Log::builder()
        .dest(2)
        .payload(String::from("Successfull initialize of tamagochi"));
    assert!(res.contains(&expected_log));
    
    //Tests to Name
    let res=program.send(2, TmgAction::Name);
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Name(name_tmg.clone()));
    assert!(res.contains(&expected_log));    
    
    //Tests to Age
    let res=program.send(2, TmgAction::Age);
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Age(date_of_birth_test));
    assert!(res.contains(&expected_log));
   
    // TODO: 6️⃣ Test new functionality 
/***********************************************************************************************/
/*Tamagotchi Iteraction                     */
/***********************************************************************************************/
    /*******Tests to Feed******/
    
    let mut my_payload_feed:u64 = 1050;
    let res=program.send(2, TmgAction::Feed);
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Fed(my_payload_feed));

    assert!(res.contains(&expected_log));
    my_payload_feed = 2050;
    let res=program.send(2, TmgAction::Feed);
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Fed(my_payload_feed));
    assert!(res.contains(&expected_log));

    /*******Tests to Enternain******/
    let mut my_payload_enternain:u64 = 1050;
    let res=program.send(2, TmgAction::Entertain);
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Entertained(my_payload_enternain));
    assert!(res.contains(&expected_log));
  
    my_payload_enternain = 2050;
    let res=program.send(2, TmgAction::Entertain);
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Entertained(my_payload_enternain));
    assert!(res.contains(&expected_log));
    
    /*******Tests to Sleep******/
    let mut my_payload_sleep:u64 = 1050;
    let res = program.send(2,TmgAction::Sleep);
    let expected_log=Log::builder()
        .dest(2)
        .payload(TmgEvent::Slept(my_payload_sleep));
    assert!(res.contains(&expected_log));

    my_payload_sleep=2050;
    let res = program.send(2, TmgAction::Sleep);
    let expected_log =Log::builder()
        .dest(2)
        .payload(TmgEvent::Slept(my_payload_sleep));
    assert!(res.contains(&expected_log)); 

}
/*/
#[test]
fn interaction_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    // TODO: 6️⃣ Test new functionality
}*/

