use crate::dioxus_spring;
use dioxus::prelude::*;
use futures::StreamExt;
use interpolation::Lerp;
use std::{collections::VecDeque, pin::Pin, task::Poll, time::Duration};

use crate::dioxus_spring::spring;

/// Hook to animate a value from some initial value `from`.
///
/// The returned [`UseSpringRef`] can be used to queue and control animations.
/// Values are linearly interpolated and sent to the handler `f`.
pub fn use_spring_ref<V>(from: V, f: impl FnMut(V) + 'static) -> UseSpringRef<V>
where
    V: Lerp<Scalar = f32> + Clone + 'static,
{
    let mut channel = use_hook(|| {
        let (tx, rx) = async_channel::unbounded();
        CopyValue::new((tx, Box::pin(rx)))
    });
    let mut f_cell = Some(f);

    use_future(move || {
        let mut f = f_cell.take().unwrap();
        let mut current = from.clone();
        let mut spring_cell = None;
        let mut stack = VecDeque::new();

        futures::future::poll_fn(move |cx| {
            while let Poll::Ready(Some(msg)) = channel.write().1.poll_next_unpin(cx) {
                match msg {
                    Message::Set(to, duration_cell) => {
                        if let Some(duration) = duration_cell {
                            let spring = spring(current.clone(), to, duration);
                            spring_cell = Some(Box::pin(spring));
                            stack.clear();
                        } else {
                            current = to.clone();
                            spring_cell.take();
                            stack.clear();
                            f(to);
                        }
                    }
                    Message::Queue(to, duration) => {
                        stack.push_back((to, duration));
                    }
                }
            }

            if spring_cell.is_none() {
                if let Some((to, duration)) = stack.pop_front() {
                    let spring = dioxus_spring::spring(current.clone(), to, duration);
                    spring_cell = Some(Box::pin(spring));
                } else {
                    spring_cell = None;
                }
            }

            while let Some(spring) = spring_cell.as_mut() {
                let mut is_done = false;
                while let Poll::Ready(item) = spring.poll_next_unpin(cx) {
                    if let Some(val) = item {
                        current = val.clone();
                        f(val);
                    } else {
                        is_done = true;
                        break;
                    }
                }

                if is_done {
                    if let Some((to, duration)) = stack.pop_front() {
                        let spring = dioxus_spring::spring(current.clone(), to, duration);
                        spring_cell = Some(Box::pin(spring));
                    } else {
                        spring_cell = None;
                    }
                } else {
                    break;
                }
            }

            Poll::<()>::Pending
        })
    });

    UseSpringRef { channel }
}

pub(crate) enum Message<V> {
    Set(V, Option<Duration>),
    Queue(V, Duration),
}

/// Hook returned from [`use_spring_ref`]
pub struct UseSpringRef<V: 'static> {
    channel: CopyValue<(
        async_channel::Sender<Message<V>>,
        Pin<Box<async_channel::Receiver<Message<V>>>>,
    )>,
}

impl<V> UseSpringRef<V> {
    pub fn set(&self, to: V) {
        self.channel
            .read()
            .0
            .force_send(Message::Set(to, None))
            .unwrap();
    }

    pub fn animate(&self, to: V, duration: Duration) {
        self.channel
            .read()
            .0
            .force_send(Message::Set(to, Some(duration)))
            .unwrap();
    }

    pub fn queue(&self, to: V, duration: Duration) {
        self.channel
            .read()
            .0
            .force_send(Message::Queue(to, duration))
            .unwrap();
    }
}

impl<V> Clone for UseSpringRef<V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<V> Copy for UseSpringRef<V> {}
