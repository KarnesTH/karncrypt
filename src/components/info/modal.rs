use leptos::*;

use crate::components::icons::Icon;

#[component]
pub fn InfoModal(
    #[prop(into)] title: String,
    #[prop(into)] on_close: Callback<()>,
    children: Children,
) -> impl IntoView {
    let exit_icon = create_memo(move |_| "x-mark");

    view! {
        <div class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50">
            <div class="bg-background-card rounded-lg w-full max-w-2xl h-[90vh] flex flex-col">
                <div class="flex justify-between items-center p-6 border-b border-gray-600">
                    <h2 class="text-xl font-bold bg-gradient-primary bg-clip-text text-transparent">{title}</h2>
                    <button
                        class="text-gray-400 hover:text-white"
                        on:click=move |_| on_close.call(())
                    >
                        <Icon icon=exit_icon.into() class="w-5 h-5" />
                    </button>
                </div>

                <div class="flex-1 overflow-y-auto p-6">
                    {children()}
                </div>
            </div>
        </div>
    }
}
