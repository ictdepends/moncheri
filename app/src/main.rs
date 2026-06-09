use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <main class="mc-shell d-flex align-items-center justify-content-center px-3 py-5">
            <section class="mc-card card rounded-5 bg-white w-100">
                <div class="card-body p-4 p-sm-5 p-lg-6">
                    <div class="d-flex align-items-center gap-3 mb-5">
                        <div class="mc-mark rounded-circle bg-primary text-white d-flex align-items-center justify-content-center fw-bold fs-4">
                            "M"
                        </div>
                        <div>
                            <div class="fw-bold fs-4 lh-1">"MonCheri"</div>
                            <div class="small text-secondary mt-1">"Private records interface"</div>
                        </div>
                    </div>

                    <div class="row align-items-center g-5">
                        <div class="col-12 col-lg-8">
                            <h1 class="mc-title display-2 fw-bold mb-4">
                                "Private searchable records."
                            </h1>

                            <p class="mc-muted fs-4 mb-0">
                                "A clean interface for storing, amending, and finding records."
                            </p>
                        </div>

                        <div class="col-12 col-lg-4">
                            <div class="mc-status rounded-4 p-4">
                                <div class="text-uppercase small fw-bold text-secondary mb-2">
                                    "Status"
                                </div>
                                <p class="fs-5 mb-0">
                                    "Initial web interface deployed."
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </main>
    }
}

fn main() {
    mount_to_body(App);
}
