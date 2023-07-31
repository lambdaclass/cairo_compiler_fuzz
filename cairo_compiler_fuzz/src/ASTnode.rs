pub(crate) trait CairoCode {
    fn to_cairo(&mut self) -> String{
        "".to_string()
    }
}

#[macro_export]
macro_rules! to_cairo_method {
    ($type:ty) => {
        const _: () = {
            fn to_cairo<T: CairoCode>() -> String{}
        };
    };
}
