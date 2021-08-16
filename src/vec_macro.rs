/// ```compile_fail
/// let x: Vec<i32> = vecmacro::avec![2; "foo"];
/// ```
#[macro_export]
macro_rules! avec {
    ($($element: expr),* $(,)?) => {{
        const COUNT: usize = $crate::count![$($element),*];
        #[allow(unused_mut)]
        let mut v = Vec::with_capacity(COUNT);
        $(v.push($element);)*
        v
    }};
    ($element: expr; $count: expr) => {{
        let count = $count;
        let mut v = Vec::with_capacity(count);
        v.resize(count, $element);
        // let x = $element;
        // for _ in 0..count {
        //     v.push(x.clone());
        // }
        v
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    ($($element: expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
    };
    (@SUBST; $_element: expr) => {
        ()
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_vec() {
        let v: Vec<u32> = avec![];
        assert!(v.is_empty())
    }

    #[test]
    fn single_element() {
        let v: Vec<u32> = avec![2];
        assert_eq!(v.len(), 1);
        assert_eq!(v[0], 2);
    }

    #[test]
    fn multi_elements() {
        let v: Vec<u32> = avec![2, 4,];
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], 2);
        assert_eq!(v[1], 4);
    }

    #[test]
    fn clone_2() {
        let v: Vec<u32> = avec![0; 2];
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], 0);
        assert_eq!(v[1], 0);
    }

    #[test]
    fn clone_2_nonliteral() {
        let mut y = Some(0);
        let v: Vec<u32> = avec![y.take().unwrap(); 2];
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], 0);
        assert_eq!(v[1], 0);
    }
}
