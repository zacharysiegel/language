use std::time::Duration;

use leptos::prelude::*;
use shared::pending_indication;

pub fn pending_indication_delay() -> Duration {
    Duration::from_millis(u64::from(pending_indication::PENDING_INDICATION_DELAY_MILLISECONDS))
}

#[derive(Clone, Copy)]
pub struct PageBusyStore {
    loads_under_way: RwSignal<u32>,
}

impl PageBusyStore {
    pub fn is_busy(self) -> bool {
        self.loads_under_way.get() > 0
    }

    fn begin_load(self) {
        self.loads_under_way.update(|count| *count += 1);
    }

    fn end_load(self) {
        self.loads_under_way.update(|count| *count = count.saturating_sub(1));
    }
}

pub fn provide_page_busy_store() {
    provide_context(PageBusyStore {
        loads_under_way: RwSignal::new(0),
    });
}

pub fn use_page_busy_store() -> PageBusyStore {
    expect_context::<PageBusyStore>()
}

/// The glyph a control shows in place of its own while what it started is still on its way.
#[component]
pub fn PendingIcon() -> impl IntoView {
    view! {
        <svg class="icon pending-glyph" viewBox="0 0 13 13" aria-hidden="true">
            <path d="M2.5 6.5h0.5M6.25 6.5h0.5M10 6.5h0.5" />
        </svg>
    }
}

/// Whether this is the server's render or the browser's first render over it.
fn renders_the_page_as_served() -> bool {
    let hydrating: bool = Owner::current_shared_context().is_some_and(|shared_context| shared_context.during_hydration());

    cfg!(feature = "ssr") || hydrating
}

/// A status line whose text appears only once the pending indication delay has passed.
#[component]
pub fn DelayedStatus(message: &'static str) -> impl IntoView {
    // The page as served shows it at once, since the browser is still loading the client code behind it.
    let delay_passed: RwSignal<bool> = RwSignal::new(renders_the_page_as_served());

    Effect::new(move |_| {
        set_timeout(move || delay_passed.set(true), pending_indication_delay());
    });

    view! { <p class="status" role="status">{move || delay_passed.get().then_some(message)}</p> }
}

/// A [`DelayedStatus`] for a page that is loading, which marks the page busy while it is shown.
#[component]
pub fn PageLoadStatus(message: &'static str) -> impl IntoView {
    let page_busy_store: PageBusyStore = use_page_busy_store();

    // In an effect, because only the browser runs the cleanup that ends the load.
    Effect::new(move |_| {
        page_busy_store.begin_load();
        on_cleanup(move || page_busy_store.end_load());
    });

    view! { <DelayedStatus message /> }
}
