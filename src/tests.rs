use std::thread;
use unsafe_send_sync::*;



#[test]
fn example() {
    let not_send = UnsafeSend::new( Rc::<u32>::new( 1337 ) );

    assert!( not_send.strong_count() == 1,
             "We can't really send a reference counted pointer across threads unless it only has one reference." );

    thread::spawn(move || {
        println!("We found a number: {}", *not_send);
    });
}