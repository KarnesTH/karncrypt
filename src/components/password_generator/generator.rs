use crate::{app::invoke, components::icons::Icon};

use leptos::*;

use serde::Serialize;

#[derive(Serialize)]
struct GeneratePasswordArgs {
    length: usize,
}

#[component]
pub fn PasswordGenerator() -> impl IntoView {
    let (length, set_length) = create_signal(16);
    let (password, set_password) = create_signal(String::new());

    view! {
        <div class="w-full max-w-md mx-auto p-6 bg-white rounded-lg shadow-md">
            <h2 class="text-2xl font-bold mb-6 text-center">"Passwort Generator"</h2>

            <div class="mb-6">
                <label class="block text-gray-700 text-sm font-bold mb-2">
                    "Passwortlänge: " {length}
                </label>
                <input
                    type="range"
                    min="8"
                    max="64"
                    value=length
                    class="w-full h-2 bg-blue-200 rounded-lg appearance-none cursor-pointer accent-blue-500"
                    on:input=move |ev| set_length.set(event_target_value(&ev).parse().unwrap_or(16))
                />
            </div>

            <button
                class="w-full flex justify-center items-center bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition duration-150 ease-in-out"
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
                <Icon icon="arrow-path" class="w-5 h-5 m-2" />
                "Generiere Passwort"
            </button>

            <div class="mt-6">
                <label class="block text-gray-700 text-sm font-bold mb-2">
                    "Generiertes Passwort:"
                </label>
                <div class="flex">
                    <input
                        type="text"
                        value=password
                        readonly
                        class="flex-grow shadow appearance-none border rounded py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                    />
                    // Optional: Copy Button
                    <button
                        class="ml-2 bg-gray-500 hover:bg-gray-700 text-white py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                        // TODO: Implement copy functionality
                    >
                        <Icon icon="clipboard" class="w-5 h-5" />
                    </button>
                </div>
            </div>
        </div>
    }
}
