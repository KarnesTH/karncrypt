use leptos::*;

use crate::components::icons::Icon;

#[component]
pub fn Dashboard() -> impl IntoView {
    let dashboard_icon = create_memo(move |_| "chart-pie");
    let refresh_icon = create_memo(move |_| "arrow-path");

    view! {
        <div class="w-full flex flex-col h-full">
            <div class="flex justify-between items-center mb-4">
                <h2 class="text-2xl font-bold mb-0 bg-gradient-primary bg-clip-text text-transparent flex items-center">
                    <Icon icon=dashboard_icon.into() class="w-8 h-8 mr-3 text-primary-100" />
                    "Dashboard"
                </h2>
                <div class="flex gap-6">
                    <button
                        class="group relative flex items-center text-gray-400 hover:text-white transition-colors"
                        on:click=move |_| {}
                    >
                        <Icon icon=refresh_icon.into() class="w-5 h-5" />
                        <span class="ml-2 whitespace-nowrap max-w-0 overflow-hidden group-hover:max-w-[100px] transition-all duration-300 text-primary-100">
                            "Aktualisieren"
                        </span>
                    </button>
                </div>
            </div>
        </div>
    }
}
