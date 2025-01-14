pub trait Sealed {}

macro_rules! impl_sealed_marker {
    ($($type:ty),* $(,)?) => {
        $(
            impl $crate::private::Sealed for $type {}
        )*
    };
}
pub(crate) use impl_sealed_marker;
