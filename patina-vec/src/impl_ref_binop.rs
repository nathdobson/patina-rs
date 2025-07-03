macro_rules! impl_ref_binop {
    (
        $(
            impl $tr:ident<$t2:ty> for $t1:ty {
                type Output = $to:ty;
                fn $method:ident();
            }
        )*
    ) => {
        $(
            impl $tr<$t2> for &$t1 {
                type Output = $to;
                #[inline]
                fn $method(self, rhs: $t2) -> $to {
                    $tr::$method(*self, rhs)
                }
            }
            impl $tr<&$t2> for $t1 {
                type Output = $to;
                #[inline]
                fn $method(self, rhs: &$t2) -> $to {
                    $tr::$method(self, *rhs)
                }
            }
            impl $tr<&$t2> for &$t1 {
                type Output = $to;
                #[inline]
                fn $method(self, rhs: &$t2) -> $to {
                    $tr::$method(*self, *rhs)
                }
            }
        )*
    };
}

pub(crate) use impl_ref_binop;
