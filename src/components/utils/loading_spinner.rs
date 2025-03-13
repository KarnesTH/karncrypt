use leptos::*;

#[component]
pub fn LoadingSpinner() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center h-64">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-100"></div>
        </div>
    }
}
