pub trait Hei {
    fn hei(&self);

    fn weird() {}
}

impl Hei for &str {
    fn hei(&self) {
        println!("hei {}", self);
    }
}

impl Hei for String {
    fn hei(&self) {
        println!("hei {}", self);
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn static_dispatch() {
    //     fn bar<H: Hei>(hs: &[H]) {
    //         for h in hs {
    //             h.hei();
    //         }
    //     }

    //     // compiler will generate `fn bar_ref_str(hs: &[&str])`,
    //     // because &str is Sized (& is Sized).
    //     bar(&["a", "b"]);
    //     // compiler will generate `fn bar_string(hs: &[String])`,
    //     // because String is Sized (fat pointer is Sized).
    //     bar(&[String::from("a"), String::from("b")]);
    // }

    // #[test]
    // fn dyn_dispatch() {
    //     // for `fn bar(hs: &[dyn Hei]) {}` compiler can't generate the specific fn,
    //     // because `dyn Hei` is not Sized, we can't kown the size of Hei trait object
    //     // at compile-time (we can't kown what type the Hei trait object is gonna be,
    //     // we can only kown it at runtime).
    //     // so we need & or Box to make the `dyn Hei` Sized.

    //     // trait object (dyn Hei):
    //     //   a fat pointer which has two pointer
    //     //   1. a pointer to an instance of a type T that implements Hei.
    //     //   2. a pointer to a vtable for the referenced trait (`struct HeiVTable { hei: *mut Fn(*mut ()) }`).
    //     // for `bar(&[&"a", &String::from("b")])` the Hei trait object of String::from("b") has
    //     //   1. a pointer to the String.
    //     //   2. &HeiVTable { hei: &<String as Hei>::hei }
    //     // so h.hei() will be compiled to h.vtable.hei(h.instance)

    //     fn bar(hs: &[&dyn Hei]) {
    //         for h in hs {
    //             h.hei();
    //         }
    //     }

    //     fn bar_box(hs: &[Box<dyn Hei>]) {
    //         for h in hs {
    //             h.hei();
    //         }
    //     }

    //     bar(&[&"a", &String::from("b")]);
    //     bar_box(&[Box::new("a"), Box::new(String::from("b"))]);
    // }

    // #[test]
    // fn mulit_trait() {
    //     // `fn baz(s: &dyn (Hei + AsRef<str>)) {}` is wrong because the trait object will have
    //     // two vtable, then it will be ?Sized.

    //     trait HeiAsRef<T: ?Sized>: Hei + AsRef<T> {}

    //     fn baz(s: &dyn HeiAsRef<str>) {
    //         s.hei();
    //         s.as_ref().hei();
    //     }

    //     impl HeiAsRef<str> for &str {}

    //     baz(&"ha");
    // }

    // #[test]
    // fn object_safety() {
    //     fn bar(h: &dyn Hei) {
    //         // `weird` doesn't take &self, we don't kown which the impl of weird we want,
    //         // is it the default one, &str one or String one? we don't kown.
    //         // (dyn Hei)::weird();

    //         // if we change the type sign of weird fn to `fn weird() where Self: Sized {}`,
    //         // which means disable the trait object transform on weird fn (disable to put
    //         // the weird fn in the vtable), so we can call it by `h.weird()`, because this
    //         // time the h is not a trait object, it's just a static function call. (we can
    //         // alse do `trait Hei where Self: Sized` to disable the entire trait to be a
    //         // trait object.)

    //         // or we make the weird fn takes &self, then the h will be a trait object, but we
    //         // can't do `fn weird(&self) where Self: Sized {}`, it can't be a trait object and
    //         // a non-trait object at the same time...
    //     }

    //     // dyn Clone is not object sate because what is return type of v.clone (v/&self also
    //     // doesn't kown the Self type, it is a trait object).
    //     // fn cl(v: &dyn Clone) {
    //     //     let x = v.clone();
    //     // }

    //     // the reason of not have any type parameters is rust will static dispatch all the fn
    //     // that using T in trait at compile-time, then the vtable will be incomparably swollen,
    //     // just for a very small chance of being called.

    //     bar(&"a");
    // }
}
