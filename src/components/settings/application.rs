use crate::app::invoke;
use leptos::{ev::SubmitEvent, *};
use serde::{Deserialize, Serialize};

use crate::components::icons::Icon;
use crate::components::utils::LoadingSpinner;

#[derive(Deserialize)]
struct AppSettingsResponse {
    default_length: usize,
}

#[derive(Serialize)]
pub struct AppSettingsArgs {
    #[serde(rename = "defaultLength")]
    default_length: usize,
}

#[component]
pub fn ApplicationSettings() -> impl IntoView {
    let (password_length, set_password_length) = create_signal(16);
    let (error, set_error) = create_signal(String::new());
    let (is_loading, set_is_loading) = create_signal(true);

    let lock_icon = create_memo(move |_| "lock-closed");
    let palette_icon = create_memo(move |_| "paint-brush");
    let language_icon = create_memo(move |_| "language");

    spawn_local(async move {
        let response = invoke("get_default_generator_length", wasm_bindgen::JsValue::NULL).await;
        if let Ok(settings) = serde_wasm_bindgen::from_value::<AppSettingsResponse>(response) {
            set_password_length.set(settings.default_length);
            set_is_loading.set(false);
        }
    });

    let handle_save_settings = move |ev: SubmitEvent| {
        ev.prevent_default();

        set_is_loading.set(true);

        let args = serde_wasm_bindgen::to_value(&AppSettingsArgs {
            default_length: password_length.get(),
        })
        .unwrap();
        spawn_local(async move {
            let response = invoke("save_app_settings", args).await;
            match serde_wasm_bindgen::from_value::<()>(response) {
                Ok(_) => {
                    set_error.set("Einstellungen gespeichert".to_string());
                    set_is_loading.set(false);
                }
                Err(_) => {
                    set_error.set("Fehler beim Speichern der Einstellungen".to_string());
                    set_is_loading.set(false);
                }
            }
        });
    };

    view! {
        <div class="flex justify-center">
            <div class="max-w-xl w-full space-y-8">
                {move || {
                    if is_loading.get() {
                        view! {
                            <LoadingSpinner />
                        }.into_view()
                    } else {
                        view! {
                            <form class="space-y-8" on:submit=handle_save_settings>
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
                                            <Icon icon=lock_icon.into() class="w-5 h-5 text-primary-100" />
                                            <span>"Generator-Einstellungen"</span>
                                        </div>
                                    </h3>

                                    <div>
                                        <label class="block text-white text-sm font-bold mb-2">
                                            "Standard Passwortlänge"
                                        </label>
                                        <div class="flex items-center space-x-4">
                                            <input
                                                type="range"
                                                min="8"
                                                max="64"
                                                class="w-full h-2 bg-background rounded-lg appearance-none cursor-pointer border border-gray-600
                                                    [&::-webkit-slider-runnable-track]:bg-background
                                                    [&::-webkit-slider-runnable-track]:rounded-lg
                                                    [&::-webkit-slider-runnable-track]:border-gray-600
                                                    [&::-webkit-slider-thumb]:w-4
                                                    [&::-webkit-slider-thumb]:h-4
                                                    [&::-webkit-slider-thumb]:bg-primary-100
                                                    [&::-webkit-slider-thumb]:border-2
                                                    [&::-webkit-slider-thumb]:border-background
                                                    [&::-webkit-slider-thumb]:rounded-full
                                                    [&::-webkit-slider-thumb]:appearance-none
                                                    hover:[&::-webkit-slider-thumb]:bg-primary-200"
                                                on:input=move |ev| set_password_length.set(event_target_value(&ev).parse().unwrap_or(16))
                                                prop:value=password_length
                                            />
                                            <span class="text-white w-12 text-center">{password_length}</span>
                                        </div>
                                        <p class="mt-1 text-sm text-gray-400">"Legt die Standardlänge neu generierter Passwörter fest"</p>
                                    </div>
                                </fieldset>

                                <fieldset class="space-y-4 opacity-50">
                                    <h3 class="text-lg font-semibold bg-gradient-primary bg-clip-text text-transparent mb-4">
                                        <div class="flex items-center space-x-2">
                                            <Icon icon=palette_icon.into() class="w-5 h-5 text-primary-100" />
                                            <span>"Theme-Einstellungen"</span>
                                        </div>
                                    </h3>
                                    <div class="flex items-center space-x-2 text-white">
                                        <span class="text-sm italic">"Coming Soon"</span>
                                    </div>
                                </fieldset>

                                <fieldset class="space-y-4 opacity-50">
                                    <h3 class="text-lg font-semibold bg-gradient-primary bg-clip-text text-transparent mb-4">
                                        <div class="flex items-center space-x-2">
                                            <Icon icon=language_icon.into() class="w-5 h-5 text-primary-100" />
                                            <span>"Sprach-Einstellungen"</span>
                                        </div>
                                    </h3>
                                    <div class="flex items-center space-x-2 text-white">
                                        <span class="text-sm italic">"Coming Soon"</span>
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
