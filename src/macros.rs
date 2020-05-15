//! Defines macros for common use cases

#[macro_export]
macro_rules! show {
    ($visitor: expr, $value:  expr, $fmt: literal, $($args: expr,)*) => {
        match ($value, std::format_args!($fmt, $($args,)*)) {
            (val, args) => $crate::Visitor::visit_message($visitor, $crate::Message::new(&args, &val))
        } 
    }
}