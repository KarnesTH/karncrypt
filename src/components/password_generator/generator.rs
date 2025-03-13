use crate::{app::invoke, components::icons::Icon};

use leptos::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct GeneratePasswordArgs {
    length: usize,
}

#[derive(Serialize)]
struct ClipboardArgs<'a> {
    text: &'a str,
}

#[derive(Deserialize)]
struct GeneratorLengthResponse {
    default_length: usize,
}

#[component]
pub fn PasswordGenerator() -> impl IntoView {
    let (length, set_length) = create_signal(16);
    let (password, set_password) = create_signal(String::new());
    let (is_copied, set_is_copied) = create_signal(false);
    let (is_loading, set_is_loading) = create_signal(false);

    let copied_icon = create_memo(move |_| {
        if is_copied.get() {
            "check"
        } else {
            "clipboard"
        }
    });

    let generate_icon = create_memo(move |_| "arrow-path");
    let sparkles_icon = create_memo(move |_| "sparkles");
    let slider_icon = create_memo(move |_| "adjustments-horizontal");

    spawn_local(async move {
        let response = invoke("get_default_generator_length", wasm_bindgen::JsValue::NULL).await;
        if let Ok(generator) = serde_wasm_bindgen::from_value::<GeneratorLengthResponse>(response) {
            set_length.set(generator.default_length);
        }
        set_is_loading.set(false);
    });

    view! {
        <div class="w-full flex gap-6">
            {move || {
                if is_loading.get() {
                    view! {
                        <div class="flex justify-center items-center h-64">
                            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-100"></div>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="w-1/2">
                            <h2 class="text-2xl font-bold mb-6 bg-gradient-primary bg-clip-text text-transparent flex items-center">
                                <Icon icon=sparkles_icon.into() class="w-8 h-8 mr-3 text-primary-100" />
                                "Passwort Generator"
                            </h2>

                            <div class="mb-6">
                                <label class="block text-white text-sm font-bold mb-2 flex items-center">
                                    <Icon icon=slider_icon.into() class="w-4 h-4 mr-2 text-primary-100" />
                                    <span>"Passwortlänge: "</span>
                                    <span class="text-primary-100 ml-1">{length}</span>
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
                        </div>

                        <div class="w-1/2">
                            <div class="relative group h-full">
                                <textarea
                                    readonly
                                    prop:value=password
                                    class="w-full h-full min-h-[200px] p-4 bg-background border-2 border-gray-400 hover:border-primary-100
                                    rounded text-white resize-none focus:outline-none focus:border-primary-200
                                        text-xl text-center flex items-center justify-center
                                        leading-relaxed tracking-wide
                                        transition-opacity duration-200 ease-in-out"
                                    style="display: flex; align-items: center; padding-top: 40px;"
                                />
                                <button
                                    class="absolute top-2 right-2 p-2 rounded-full bg-transparent opacity-0 group-hover:opacity-100 transition-opacity"
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
                                    {move || {
                                        if is_copied.get() {
                                            view! {
                                                <Icon
                                                    icon=copied_icon.into()
                                                    class="w-5 h-5 text-green-500"
                                                />
                                            }
                                        } else {
                                            view! {
                                                <Icon
                                                    icon=copied_icon.into()
                                                    class="w-5 h-5 text-primary-100"
                                                />
                                            }
                                        }
                                    }}
                                </button>
                            </div>
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}
