use futures_util::Stream;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct SyncStream<S>(pub S);

unsafe impl<S> Sync for SyncStream<S> {}

impl<S: Stream + Unpin> Stream for SyncStream<S> {
    type Item = S::Item;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.0).poll_next(cx)
    }
}
