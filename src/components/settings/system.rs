use leptos::*;

use crate::{app::invoke, components::icons::Icon};

#[component]
pub fn SystemSettings() -> impl IntoView {
    let (auto_start, set_auto_start) = create_signal(false);
    let (error, set_error) = create_signal(String::new());
    let (is_loading, set_is_loading) = create_signal(false);

    let rocket_icon = create_memo(move |_| "rocket-launch");
    let arrow_path_icon = create_memo(move |_| "arrow-path");
    let document_icon = create_memo(move |_| "document-text");

    let handle_open_log_folder = move |_| {
        set_is_loading.set(true);
        spawn_local(async move {
            let response = invoke("open_log_folder", wasm_bindgen::JsValue::NULL).await;
            if let Ok(_) = serde_wasm_bindgen::from_value::<()>(response) {
                set_is_loading.set(false);
            } else {
                set_is_loading.set(false);
                set_error.set("Failed to open log folder.".to_string());
            }
        });
    };

    view! {

        <div class="flex justify-center">
            <div class="max-w-xl w-full space-y-8">
                {move || {
                    if is_loading.get() {
                        view! {
                            <div class="flex justify-center items-center h-64">
                                <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-100"></div>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <form class="space-y-8">
                                {move || (!error.get().is_empty()).then(||
                                    view! {
                                        <div class="text-primary-100 text-sm text-center">
                                            {error.get()}
                                        </div>
                                    }
                                )}

                                <fieldset class="space-y-4">
                                    <h3 class="text-lg font-semibold bg-gradient-primary bg-clip-text text-transparent mb-4">
                                        <div class="flex items-center space-x-2">
                                            <Icon icon=rocket_icon.into() class="w-5 h-5 text-primary-100" />
                                            <span>"Autostart"</span>
                                        </div>
                                    </h3>
                                    <div class="flex items-center space-x-3">
                                        <input
                                            type="checkbox"
                                            id="auto-start"
                                            class="w-4 h-4 bg-background border-gray-600 rounded focus:ring-primary-100"
                                            on:change=move |ev| set_auto_start.set(event_target_checked(&ev))
                                            prop:checked=auto_start
                                        />
                                        <label for="auto-start" class="text-white text-sm font-bold">
                                            "Automatisch mit Windows starten"
                                        </label>
                                    </div>
                                </fieldset>

                                <fieldset class="space-y-4">
                                    <h3 class="text-lg font-semibold bg-gradient-primary bg-clip-text text-transparent mb-4">
                                        <div class="flex items-center space-x-2">
                                            <Icon icon=arrow_path_icon.into() class="w-5 h-5 text-primary-100" />
                                            <span>"Updates"</span>
                                        </div>
                                    </h3>
                                    <div class="space-y-4">
                                        <p class="text-gray-300">
                                            "Aktuelle Version: "{env!("CARGO_PKG_VERSION")}
                                        </p>
                                        <button
                                            type="button"
                                            class="flex items-center space-x-2 px-4 py-2 bg-background border border-primary-100 hover:bg-primary-400/10 text-white rounded focus:outline-none transition-all duration-200"
                                        >
                                            <Icon icon=arrow_path_icon.into() class="w-5 h-5 text-primary-100" />
                                            <span>"Nach Updates suchen"</span>
                                        </button>
                                    </div>
                                </fieldset>

                                <fieldset class="space-y-4">
                                    <h3 class="text-lg font-semibold bg-gradient-primary bg-clip-text text-transparent mb-4">
                                        <div class="flex items-center space-x-2">
                                            <Icon icon=document_icon.into() class="w-5 h-5 text-primary-100" />
                                            <span>"Diagnose"</span>
                                        </div>
                                    </h3>
                                    <div class="space-y-4">
                                        <button
                                            type="button"
                                            class="flex items-center space-x-2 px-4 py-2 bg-background border border-primary-100 hover:bg-primary-400/10 text-white rounded focus:outline-none transition-all duration-200"
                                            on:click=handle_open_log_folder
                                        >
                                            <Icon icon=document_icon.into() class="w-5 h-5 text-primary-100" />
                                            <span>"Log-Datei öffnen"</span>
                                        </button>
                                    </div>
                                </fieldset>

                                <div class="flex justify-end pt-4 border-t border-gray-600">
                                    <button
                                        type="submit"
                                        class="bg-gradient-primary text-white font-bold py-2 px-8 rounded focus:outline-none hover:opacity-90 transition-opacity"
                                    >
                                        "Speichern"
                                    </button>
                                </div>
                            </form>
                        }.into_view()
                    }
                }}
            </div>
        </div>
    }
}
