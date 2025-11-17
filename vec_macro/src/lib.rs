/*
avec![];
avec![1]; either $($element: expr,)* or $($element: expr),*
avec![1,];
*/

#[macro_export]
macro_rules! avec {
    ($($element: expr),*) => {
        {
            #[allow(unused_mut)]
            let mut vs = Vec::with_capacity($crate::count![@COUNT; $($element),*]);
            $(vs.push($element);)*
            vs
        }
    };

    ($($element: expr,)*) => {
        {
            $crate::avec![$($element),*]
        }
    };

    ($element: expr; $count: expr) => {
        {
            let mut v = Vec::new();
            v.resize($count, $element);
            v
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($element: expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])       
    };
    (@SUBST; $element: expr) => {
        ()
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_vec() {
        let v: Vec<u32> = avec![];
        assert!(v.is_empty());
    }

    #[test]
    fn multiple() {
        let v = avec![0,1,2,3];
        assert_eq!(v.len(), 4);
        assert_eq!(v[0], 0);
        assert_eq!(v[1], 1);
        assert_eq!(v[2], 2);
        assert_eq!(v[3], 3);
    }

    #[test]
    fn count() {
        let mut x = Some(0);
        let v = avec![x.take().unwrap();3];
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], 0);
        assert_eq!(v[1], 0);
        assert_eq!(v[2], 0);
    }

    #[test]
    fn trailing() {
        let v: Vec<&'static str> = avec![
            "abcdefghijklmnopqrstuvwxyz",
            "abcdefghijklmnopqrstuvwxyz",
            "abcdefghijklmnopqrstuvwxyz",
            "abcdefghijklmnopqrstuvwxyz",
            "abcdefghijklmnopqrstuvwxyz",
        ];
    }
}

/// ```compile fail
/// let x: Vec<u32> = VEC_MACRO::avec![42; "foo"];
/// ```
#[allow(dead_code)]
struct CompileFailTest;