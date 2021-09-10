use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

pub struct Boks<T> {
    // p: *mut T,
    p: NonNull<T>,
    _m: PhantomData<T>,
}

impl<T> Boks<T> {
    pub fn new(t: T) -> Self {
        Self {
            // Safety: Box never creates a null ptr.
            p: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(t))) },
            _m: PhantomData,
        }
    }
}

// to fix dropck, we can use #[may_dangle] in nightly to tell compiler that the drop impl
// will not use the T, see Box's drop impl
impl<T> Drop for Boks<T> {
    fn drop(&mut self) {
        // std::ptr::read(self.p) maybe use p at here.

        // Safety: p was constructed from a Box in the first place, and has not been freed,
        // otherwise sinse self still exits (otherwise, drop could not be called).
        unsafe { Box::from_raw(self.p.as_mut()) };
    }
}

impl<T> Deref for Boks<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: is valid since it was constructed from a valid T,
        // and turned into a pointer through Box which creates aligned pointers,
        // and hasn't been freed, since self is alived.
        unsafe { &*self.p.as_ref() }
    }
}

impl<T> DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: is valid since it was constructed from a valid T,
        // and turned into a pointer through Box which creates aligned pointers,
        // and hasn't been freed, since self is alived.
        // Also, since we have &mut self, no other mutable reference has been given out to p.
        unsafe { &mut *self.p.as_mut() }
    }
}

pub struct Oisann<T: Debug>(T);

impl<T: Debug> Drop for Oisann<T> {
    fn drop(&mut self) {
        println!("{:?}", self.0)
    }
}

// std::iter::Empty is using PhantomData<T>
pub struct EmptyIterator<T> {
    // Use T by PhantomData to make T not unused...
    // but we don't want the dropck to check T and we want it to be covarient,
    // so we use PhantomData<fn() -> T> instead of PhantomData<T>.
    _m: PhantomData<fn() -> T>,
}

impl<T> Iterator for EmptyIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[test]
fn gogo() {
    let x = 2;
    let b = Boks::new(x);
    println!("{:?}", *b);

    // let mut x = 2;
    // let b = Boks::new(&mut x);
    // println!("{:?}", x); // ownership of x gives to println
    // drop(b); // ptr::read(x) at here

    // let mut x = 2;
    // let b = Boks::new(Oisann(&mut x));
    // println!("{:?}", x);
    // drop(b); // Compiler don't kown the Osiann will drop when Boks drops (without PhantomData).

    // Boks is invarient because Boks has *mut T
    // let s = String::from("hah");
    // let mut bk1 = Boks::new(&*s);
    // let bk2: Boks<&'static str> = Boks::new("eee");
    // bk1 = bk2;
}
