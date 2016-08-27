/// Creates operator implementations for a Filter struct
/// Currently only Fn and Filter implementations are handled
///
/// Takes at least two arguments: the struct to implement the traits for, and
/// the block to define the filter function. Preceeding the block should be the
/// variables for self and the passed in argument to filter, separated by spaces.
/// After that, any filter generics needed for the struct should be specified.
///
/// # Examples
///
/// ```
/// # #![cfg_attr(feature = "unstable-filter-as-fn", feature(unboxed_closures, fn_traits))]
/// # #[macro_use] extern crate filters;
/// # use filters::filter::Filter;
/// # fn main() {
/// struct Or<T, U> {
///     a: T,
///     b: U
/// }
///
/// impl_operators!(Or, self e { self.a.filter(e) || self.b.filter(e) }, T, U);
///
/// let or = Or {a: |&x: &i32| x < 5, b: |&x: &i32| x > 10};
/// assert_eq!(or.filter(&0), true);
/// assert_eq!(or.filter(&7), false);
/// assert_eq!(or.filter(&15), true);
/// // If unstable-filter-as-fn is enabled `or` will also be a Fn(&i32) -> bool
/// # }
/// ```
#[macro_export]
macro_rules! impl_operators {
    ($struct_ident:ident, $self_var: ident $arg_var: ident $filter_impl:block, $( $generic:ident ),*) => {
        #[cfg(feature = "unstable-filter-as-fn")]
        impl<'a, I, $( $generic: Filter<I>, )*> FnOnce<(&'a I,)> for $struct_ident<$( $generic, )*> {
            type Output = bool;
            extern "rust-call" fn call_once<'b>(self, (arg,): (&'a I,)) -> Self::Output {
                self.filter(arg)
            }
        }

        #[cfg(feature = "unstable-filter-as-fn")]
        impl<'a, I, $( $generic: Filter<I>, )*> FnMut<(&'a I,)> for $struct_ident<$( $generic, )*> {
            extern "rust-call" fn call_mut<'b>(&mut self, (arg,): (&'a I,)) -> Self::Output {
                self.filter(arg)
            }
        }

        #[cfg(feature = "unstable-filter-as-fn")]
        impl<'a, I, $( $generic: Filter<I>, )*> Fn<(&'a I,)> for $struct_ident<$( $generic, )*> {
            #[allow(unused_variables)]
            extern "rust-call" fn call<'b>(&$self_var, ($arg_var,): (&'a I,)) -> Self::Output $filter_impl
        }

        #[cfg(not(feature = "unstable-filter-as-fn"))]
        impl<I, $( $generic: Filter<I>, )*> Filter<I> for $struct_ident<$( $generic, )*> {
            #[allow(unused_variables)]
            fn filter(&$self_var, $arg_var: &I) -> bool $filter_impl
        }
    };
    ($struct_ident:ident, $self_var: ident $arg_var: ident $filter_impl: block) => {
        impl_operators!($struct_ident, $self_var $arg_var $filter_impl, );
    };
}
