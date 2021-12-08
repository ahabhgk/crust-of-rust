use std::mem::MaybeUninit;

fn bar<T>(_: T) -> T {
    unsafe { MaybeUninit::zeroed().assume_init() }
}

fn baz(f: fn(u32) -> u32) {
    // f is a fn pointer, so the size of a fn pointer is 8
    println!("size of f: {}", std::mem::size_of_val(&f));
}

#[test]
fn playground1() {
    let b = bar::<u32>;
    // b is a fn item, so the size is 0
    println!("size of b: {}", std::mem::size_of_val(&b));
    baz(b)
}

// trait Fn: FnMut
// trait FnMut: FnOnce
// impl Fn for fn pointer

// fn foo<F: Fn()>(f: &F) { // f needs shared ref at least
//     f()
// }

// fn foo<F: FnMut()>(f: &mut F) { // f needs mut ref at least
//     f()
// }

// fn foo<F: FnOnce()>(f: F) { // f needs the value
//     f()
// }

#[test]
fn playground2() {
    fn foo(f: fn()) {
        f()
    }
    let s = String::new();
    // // closures can only be coerced to `fn` types if they do not capture any variables
    // // compiler generates like this:
    // // struct FClosure<'scope> {
    // //     s: &'scope String
    // // }
    // // impl Fn() for FClosure {
    // //     fn call(&self) {
    // //         println!("{}", self.s);
    // //     }
    // // }
    // let f = || println!("{}", &s);
    // foo(f)
}

#[test]
fn playground3() {
    fn foo<F: FnMut()>(mut f: F) {
        f()
    }
    let mut s = String::new();
    let f = || s.clear();
    foo(f);
}

#[test]
fn playground4() {
    fn foo<F: FnOnce()>(f: F) {
        f()
    }
    let s = String::new();
    let f = || drop(s);
    foo(f);
}

// https://blog.rust-lang.org/2019/05/23/Rust-1.35.0.html#fn-closure-traits-implemented-for-boxdyn-fn
// fn foo(f: Box<dyn Fn()>) {
//     f()
// }
// fn foo(mut f: Box<dyn FnMut()>) {
//     f()
// }
// fn foo(f: Box<dyn FnOnce()>) {
//     f()
// }

// https://doc.rust-lang.org/src/alloc/boxed.rs.html#1687-1693
// impl FnOnce() for Box<dyn FnOnce()> {
//     fn call(self) {
//         // dyn FnOnce() is not Sized, needs & or Box
//         let x: dyn FnOnce() = *self;
//         x.call(args)
//     }
// }

// const fn foo<F: ~const FnOnce()>(f: F) {
//     f()
// }

fn foo<F>(f: F)
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
}
