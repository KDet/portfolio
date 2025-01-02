use dioxus::prelude::*;
use interpolation::Lerp;

use super::{use_spring_ref, UseSpringRef};

/// Hook to create an animated signal from an initial value.
pub fn use_spring_signal<V>(from: V) -> (Signal<V>, UseSpringRef<V>)
where
    V: PartialEq + Lerp<Scalar = f32> + Clone + 'static,
{
    let from_clone = from.clone();
    let mut output = use_signal(move || from_clone);

    let spring_ref = use_spring_ref(from, move |value| output.set(value));

    (output, spring_ref)
}
