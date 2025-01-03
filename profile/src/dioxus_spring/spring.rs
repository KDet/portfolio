use futures::{stream, Stream};
use interpolation::Lerp;
use js_sys::Date;
use std::time::Duration;

use super::request_animation_frame;

/// Create a stream of interpolated values from one value to another over a [`Duration`].
pub fn spring<V>(from: V, to: V, duration: Duration) -> impl Stream<Item = V>
where
    V: Lerp<Scalar = f32> + Clone + 'static,
{
    let mut start_cell = None;
    stream::unfold(false, move |is_done| {
        if start_cell.is_none() {
            start_cell = Some(Date::now());
        }
        let start = start_cell.unwrap();

        let from = from.clone();
        let to = to.clone();
        async move {
            if is_done {
                return None;
            }

            request_animation_frame().await;

            let dt = Date::now() - start;
            let duration_ms = duration.as_secs_f64() * 1000.;
            if dt >= duration_ms {
                let v = interpolation::lerp(&from, &to, &1.);
                return Some((v, true));
            }

            let x = dt / duration_ms;
            let v = interpolation::lerp(&from, &to, &(x as f32));
            Some((v, false))
        }
    })
}
