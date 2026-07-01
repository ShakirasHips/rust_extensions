//! # Deep Flatten
//!
//! A crate for deeply flattening nested collections into flat iterators.
//!
//! This crate provides the [`DeepFlatten`] trait, which allows you to flatten
//! multi-dimensional vectors (like `Vec<Vec<Vec<T>>>`) into a single-dimensional
//! iterator of items in one method call.

/// Procedural macro to automatically derive the `DeepFlatten` trait.
///
/// # Examples
///
/// ```rust
/// use deep_flat::DeepFlatten;
///
/// #[derive(DeepFlatten)]
/// # // Note: A mock implementation would be needed for the macro to compile in this test.
/// struct MyCollection;
/// ```
pub use deep_flatten_derive::DeepFlatten;


/// A trait for recursively flattening nested structures into a flat iterator sequence.
///
/// Any type implementing `DeepFlatten` can be unwrapped down to its core elements,
/// regardless of how many nesting levels exist (e.g., `Vec<Vec<i32>>` becomes `Iterator<Item = i32>`).
pub trait DeepFlatten {
    /// The fundamental item type produced after all nesting levels are stripped away.
    type Item;

    /// Recursively flattens the collection into an implementation of [`Iterator`].
    ///
    /// # Examples
    ///
    /// Flattening a deeply nested `Vec`:
    /// ```rust
    /// use deep_flat::DeepFlatten;
    ///
    /// // A three-dimensional nested vector
    /// let nested: Vec<Vec<Vec<i32>>> = vec![
    ///     vec![vec![1, 2], vec![3]],
    ///     vec![vec![4, 5, 6]]
    /// ];
    ///
    /// // Flatten all layers down to a single iterator
    /// let flat: Vec<i32> = nested.deep_flatten().collect();
    ///
    /// assert_eq!(flat, vec![1, 2, 3, 4, 5, 6]);
    /// ```
    fn deep_flatten(self) -> impl Iterator<Item = Self::Item>;
}

/// Blanket implementation of `DeepFlatten` for generic vectors.
///
/// This relies on the inner item `T` also implementing `DeepFlatten`, allowing
/// Rust to recursively unwrap elements until a base case is hit.
impl<T: DeepFlatten> DeepFlatten for Vec<T> {
    /// The core item is delegated to the inner type's item definition.
    type Item = T::Item;

    /// Flattens the vector's elements by executing a recursive `flat_map`.
    fn deep_flatten(self) -> impl Iterator<Item = Self::Item> {
        self.into_iter().flat_map(|x| x.deep_flatten())
    }
}

macro_rules! impl_deep_flatten_primitive {
    ($($t:ty),*) => {
        $(impl DeepFlatten for $t {
            type Item = $t;
            fn deep_flatten(self) -> impl Iterator<Item = $t> {
                std::iter::once(self)
            }
        })*
    };
}

impl_deep_flatten_primitive!(
    i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, bool, char, String
);

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(DeepFlatten, Debug, PartialEq)]
    struct Point {
        x: f32,
        y: f32,
    }

    #[derive(DeepFlatten)]
    struct Wrapper<T> {
        inner: T,
    }

    #[test]
    fn struct_flatten() {
        let v = vec![
            vec![
                vec![Point { x: 1.0, y: 2.0 }, Point { x: 3.0, y: 4.0 }],
                vec![Point { x: 1.0, y: 2.0 }, Point { x: 3.0, y: 4.0 }],
            ],
            vec![vec![Point { x: 5.0, y: 6.0 }]],
        ];
        let flat = v.deep_flatten().collect::<Vec<_>>();
        assert_eq!(
            flat,
            vec![
                Point { x: 1.0, y: 2.0 },
                Point { x: 3.0, y: 4.0 },
                Point { x: 1.0, y: 2.0 },
                Point { x: 3.0, y: 4.0 },
                Point { x: 5.0, y: 6.0 }
            ]
        );
    }

    #[test]
    fn vec_i32_flatten() {
        let v = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6]]];
        let flat = v.deep_flatten().collect::<Vec<_>>();
        assert_eq!(flat, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn nested_generic_vec_flattening() {
        let nested_structure: Vec<Wrapper<i32>> =
            vec![Wrapper { inner: 10 }, Wrapper { inner: 20 }];

        let flat: Vec<Wrapper<i32>> = nested_structure.deep_flatten().collect();

        assert_eq!(flat.len(), 2);
        assert_eq!(flat[0].inner, 10);
        assert_eq!(flat[1].inner, 20);
    }
}
