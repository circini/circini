extern crate circini_core;
#[macro_use]
extern crate circini_derive;

use circini_core::{Event, Subtype, AnyEvent};


#[derive(Clone, Debug, Event)]
struct KeyDown {
    key: usize,
}

#[derive(Clone, Debug, Event)]
struct KeyUp {
    key: usize,
}

#[test]
fn test_upcast_to_any_and_back(){
    let event = KeyDown {
        key: 17,
    };
    println!("Before upcast: {:?}", &event);
    
    let event_any = event.upcast_to_any();
    println!("After upcast: {:?}", event_any);
    
    let event_down = <KeyDown as Subtype<AnyEvent>>::filter(event_any);
    println!("After downcast: {:?}", event_down.unwrap()); 
}

#[test]
fn test_upcast_to_any_and_back_invalid(){
    let event = KeyDown {
        key: 42,
    };
    println!("Before upcast: {:?}", &event);
    
    let event_any = event.upcast_to_any();
    println!("After upcast: {:?}", event_any);

    println!("Trying downcast to wrong event...");
    let event_down_maybe = <KeyUp as Subtype<AnyEvent>>::filter(event_any);
    match event_down_maybe {
        Some(_) => panic!("Invalid downcast somehow successful"),
        None => (),
    }
}