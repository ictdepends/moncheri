use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <div class="wordmark">
                "moncheri"
                <span class="cursor"></span>
            </div>
        </main>
    }
}

fn main() {
    mount_to_body(App);
}
