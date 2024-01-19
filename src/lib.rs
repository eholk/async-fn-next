#![allow(async_fn_in_trait)]

use core::future::Future;

pub trait Iterator {
    type Item;

    async fn next(&mut self) -> Option<Self::Item>;
}

/// An iterator that yields a single element.
/// 
/// # Examples
/// 
/// ```
/// # use futures_executor::block_on;
/// # use async_fn_next::*;
/// # block_on(async {
/// let mut iter = once(async { 1 });
/// assert_eq!(iter.next().await, Some(1));
/// assert_eq!(iter.next().await, None);
/// # })
pub fn once<F: Future>(f: F) -> impl Iterator<Item = F::Output> {
    Once { f: Some(f) }
}

pub struct Once<F> {
    f: Option<F>,
}

impl<F: Future> Iterator for Once<F> {
    type Item = F::Output;

    async fn next(&mut self) -> Option<Self::Item> {
        match self.f.take() {
            Some(f) => Some(f.await),
            None => None,
        }
    }
}
