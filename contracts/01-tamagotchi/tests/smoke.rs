use gtest::{Log,Program, System};
use tamagotchi_io::*;

#[test]
fn smoke_test() {
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
        .payload(name_tmg.clone());
    assert!(res.contains(&expected_log));
    
    //Tests to Age
    let res=program.send(2, TmgAction::Age);
    let expected_log = Log::builder()
        .dest(2)
        .payload(date_of_birth_test);
    assert!(res.contains(&expected_log));

}