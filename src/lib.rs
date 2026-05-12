pub use derive::DeepFlatten;

pub trait DeepFlatten {
    type Item;
    fn deep_flatten(self) -> Vec<Self::Item>;
}

impl<T: DeepFlatten> DeepFlatten for Vec<T> {
    type Item = T::Item;
    fn deep_flatten(self) -> Vec<T::Item> {
        self.into_iter().flat_map(|x| x.deep_flatten()).collect()
    }
}

macro_rules! impl_deep_flatten_primitive {
    ($($t:ty),*) => {
        $(impl DeepFlatten for $t {
            type Item = $t;
            fn deep_flatten(self) -> Vec<$t> {
                vec![self]
            }
        })*
    };
}

impl_deep_flatten_primitive!(
    i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, bool, char, String
);
