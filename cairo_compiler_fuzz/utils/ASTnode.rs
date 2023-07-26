pub(crate) trait CairoCode {
    fn to_cairo() -> String{}
}

#[macro_export]
macro_rules! to_cairo_method {
    ($type:ty) => {
        const _: () = {
            fn to_cairo<T: CairoCode>() -> String{}
        };
    };
}
