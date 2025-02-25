use crate::{app::invoke, components::icons::Icon};

use leptos::*;

use serde::Serialize;

#[derive(Serialize)]
struct GeneratePasswordArgs {
    length: usize,
}

#[derive(Serialize)]
struct ClipboardArgs<'a> {
    text: &'a str,
}

#[component]
pub fn PasswordGenerator() -> impl IntoView {
    let (length, set_length) = create_signal(16);
    let (password, set_password) = create_signal(String::new());
    let (is_copied, set_is_copied) = create_signal(false);

    let copied_icon = create_memo(move |_| {
        if is_copied.get() {
            "check"
        } else {
            "clipboard"
        }
    });

    let generate_icon = create_memo(move |_| "arrow-path");

    view! {
        <div class="w-full">
            <h2 class="text-2xl font-bold mb-6 text-center text-white">"Passwort Generator"</h2>

            <div class="mb-6">
                <label class="block text-white text-sm font-bold mb-2">
                    "Passwortlänge: "
                    <span class="text-primary-100">{length}</span>
                </label>
                <div class="relative">
                    <input
                        type="range"
                        min="8"
                        max="64"
                        value=length
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
                                hover:[&::-webkit-slider-thumb]:bg-primary-200
                                focus:[&::-webkit-slider-thumb]:ring-2
                                focus:[&::-webkit-slider-thumb]:ring-primary-100"
                        on:input=move |ev| set_length.set(event_target_value(&ev).parse().unwrap_or(16))
                    />
                </div>
            </div>

            <button
                class="w-full flex justify-center items-center bg-gradient-primary text-white font-bold py-2 px-4 rounded focus:outline-none hover:opacity-90 transition-opacity"
                on:click=move |_| {
                    spawn_local(async move {
                        let args = serde_wasm_bindgen::to_value(&GeneratePasswordArgs {
                            length: length.get()
                        }).unwrap();

                        let response = invoke("generate_password", args).await;

                        if let Ok(new_pass) = serde_wasm_bindgen::from_value(response) {
                            set_password.set(new_pass);
                        }
                    });
                }
            >
                <Icon icon=generate_icon.into() class="w-5 h-5 mr-2" />
                "Generiere Passwort"
            </button>

            <div class="mt-6">
                <label class="block text-white text-sm font-bold mb-2">
                    "Generiertes Passwort:"
                </label>
                <div class="flex">
                    <input
                        type="text"
                        value=password
                        readonly
                        class="flex-grow shadow appearance-none border border-gray-600 rounded py-2 px-3 bg-background text-white leading-tight focus:outline-none focus:border-primary-100"
                    />
                    <button
                        class="ml-2 bg-primary-200 hover:bg-primary-300 text-white py-2 px-4 rounded focus:outline-none transition-colors"
                        on:click=move |_| {
                            let current_password = password.get();
                            if !current_password.is_empty() {
                                set_is_copied.set(true);
                                spawn_local(async move {
                                    let args = serde_wasm_bindgen::to_value(&ClipboardArgs {
                                        text: &current_password,
                                    }).unwrap();

                                    let _ = invoke("copy_to_clipboard", args).await;
                                });
                            }
                        }
                        on:mouseleave=move |_| set_is_copied.set(false)
                    >
                    <Icon
                        icon=copied_icon.into()
                        class="w-5 h-5"
                    />
                    </button>
                </div>
            </div>
        </div>
    }
}
