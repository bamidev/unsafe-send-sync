# Unsafe Send Sync

This is a Rust package that basically provides 3 wrapper types.
* UnsafeSend
* UnsafeSync
* UnsafeSendSync

They can be used to force structs to be Send and/or Sync, which is unsafe of course.

## Example

```rust
use std::thread;
use std::rc::Rc;

fn main() {
    let not_send = UnsafeSend::new( Rc::<u32>::new( 1337 ) );
    
    assert!( not_send.strong_count() == 1,
        "We can't really send a reference counted pointer across threads unless it only has one reference." );
    
    thread::spawn(move || {
        println!("We found a number: {}", *not_send);
    });
}
```
