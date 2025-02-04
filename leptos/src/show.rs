use leptos::component;
use leptos_dom::{Fragment, IntoView};
use leptos_reactive::{create_memo, Scope};

/// A component that will show its children when the `when` condition is `true`,
/// and show the fallback when it is `false`, without rerendering every time
/// the condition changes.
///
/// ```rust
/// # use leptos_reactive::*;
/// # use leptos_macro::*;
/// # use leptos_dom::*; use leptos::*;
/// # run_scope(create_runtime(), |cx| {
/// let (value, set_value) = create_signal(cx, 0);
///
/// view! { cx,
///   <Show
///     when=move || value() < 5
///     fallback=|cx| view! { cx, "Big number!" }
///   >
///     "Small number!"
///   </Show>
/// }
/// # });
/// ```
#[component]
pub fn Show<F, W, IV>(
    /// The scope the component is running in
    cx: Scope,
    /// The components Show wraps
    children: Box<dyn Fn(Scope) -> Fragment>,
    /// A closure that returns a bool that determines whether this thing runs
    when: W,
    /// A closure that returns what gets rendered if the when statement is false
    fallback: F,
) -> impl IntoView
where
    W: Fn() -> bool + 'static,
    F: Fn(Scope) -> IV + 'static,
    IV: IntoView,
{
    let memoized_when = create_memo(cx, move |_| when());

    move || match memoized_when.get() {
        true => children(cx).into_view(cx),
        false => fallback(cx).into_view(cx),
    }
}
