use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <h1>"MonCheri"</h1>
            <p>"Private searchable records app."</p>

            <section>
                <h2>"Status"</h2>
                <p>"Initial web interface deployed."</p>
            </section>
        </main>
    }
}

fn main() {
    mount_to_body(App);
}
