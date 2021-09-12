use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

const UNLOCKED: bool = false;
const LOCKED: bool = true;

pub struct Mutex<T> {
    v: UnsafeCell<T>,
    locked: AtomicBool,
}

unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            locked: AtomicBool::new(UNLOCKED),
            v: UnsafeCell::new(t),
        }
    }

    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while self.locked.load(Ordering::Relaxed) == LOCKED {
                thread::yield_now();
            }
            thread::yield_now();
        }
        // Safety: we hold the lock, therefore we can create a mutable reference.
        let r = f(unsafe { &mut *self.v.get() });
        self.locked.store(UNLOCKED, Ordering::Release);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::atomic::AtomicUsize, thread};

    #[test]
    fn gogo() {
        let l: &'static _ = Box::leak(Box::new(Mutex::new(0)));
        let handles = (0..10)
            .map(|_| {
                thread::spawn(move || {
                    for _ in 0..100 {
                        l.with_lock(|v| {
                            *v += 1;
                        })
                    }
                })
            })
            .collect::<Vec<_>>();
        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(l.with_lock(|v| *v), 10 * 100);
    }

    #[test]
    fn too_release() {
        let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
        let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
        let t1 = thread::spawn(move || {
            let r1 = x.load(Ordering::Relaxed);
            y.store(r1, Ordering::Relaxed);
            r1
        });
        let t2 = thread::spawn(move || {
            let r2 = y.load(Ordering::Relaxed);
            x.store(42, Ordering::Relaxed);
            r2
        });
        let r1 = t1.join().unwrap();
        let r2 = t2.join().unwrap();
        println!("x: {:?}", r1);
        println!("y: {:?}", r2);
    }

    #[test]
    fn fetch_methods() {
        let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
        let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
        let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

        let _tx = thread::spawn(move || {
            x.store(true, Ordering::Release);
        });
        let _ty = thread::spawn(move || {
            y.store(true, Ordering::Release);
        });
        let t1 = thread::spawn(move || {
            while !x.load(Ordering::Acquire) {}
            if y.load(Ordering::Acquire) {
                z.fetch_add(1, Ordering::Relaxed);
            }
        });
        let t2 = thread::spawn(move || {
            while !y.load(Ordering::Acquire) {}
            if x.load(Ordering::Acquire) {
                z.fetch_add(1, Ordering::Relaxed);
            }
        });
        t1.join().unwrap();
        t2.join().unwrap();
        let z = z.load(Ordering::SeqCst);
        println!("{:?}", z);
        // 1: tx, t1, ty, t2
        // 2: tx, ty, t1, t2
        // 0:
        //   Restriction:
        //     t1 must run "after" tx
        //     t2 must run "after" ty
        //   Given that:
        //     .. tx .. ty
        //     ty t2 t1 tx -> t1 will inc z
        //     tx t1 t2 ty -> t2 will inc z
        //     tx ty t1 t2 -> t1 & t2 will inc z
        //     ...         -> t1 & t2 will inc z
        //   Seems 0 is impossiable...
        //   But! if x and y were cached (or something else) when load at the while statement, then x and y stored,
        //   then the if condition is false because of the cache, z will be **zore** at the end,
        //   t1 and t2 thread may saw the different view of x and y.
    }
}
