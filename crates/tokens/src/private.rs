pub trait Sealed {}

macro_rules! impl_sealed_marker {
    ($trait:ty, $($type:ty),* $(,)?) => {
        $(
            impl $crate::private::Sealed for $type {}
            impl $trait for $type {}
        )*
    };
}
pub(crate) use impl_sealed_marker;
