#![feature(coroutines, coroutine_trait)]

use std::ops::Coroutine;
use std::pin::Pin;

fn main() {
    let _b = {
        let a = 3;
        Pin::new(&mut || yield &a).resume(())
        //~^ ERROR: `a` does not live long enough
    };

    let _b = {
        let a = 3;
        || {
            yield &a
            //~^ ERROR: `a` does not live long enough
        }
    };
}
