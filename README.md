# Unsafe Send Sync

This is a Rust package that basically provides 3 wrapper types.
* UnsafeSend
* UnsafeSync
* UnsafeSendSync
They can be used to force structs to be Send and/or Sync, which is unsafe of course.

## Example

```rustc
use std::rc::Rc;
use tokio;

...

let not_send = Rc::<int>::new( 1337 );

assert!( not_send.strong_count() == 1, "We can't really send a reference counted pointer across threads unless it only has one reference." );

tokio::spawn(move || {
	println!("We found a number: {}", *not_send);
});
```
