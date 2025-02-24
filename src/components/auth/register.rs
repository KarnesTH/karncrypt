use crate::app::invoke;
use leptos::*;
use serde::Serialize;

#[derive(Serialize)]
struct RegisterArgs<'a> {
    username: &'a str,
    #[serde(rename = "masterPass")]
    master_pass: &'a str,
}

#[component]
pub fn Register(
    #[prop(into)] on_success: Callback<()>,
    #[prop(into)] on_back: Callback<()>,
) -> impl IntoView {
    let (username, set_username) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (confirm_password, set_confirm_password) = create_signal(String::new());
    let (error, set_error) = create_signal(String::new());

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        let username = username.get();
        let password = password.get();
        let confirm = confirm_password.get();

        if username.is_empty() || password.is_empty() {
            set_error.set("Please fill in all fields".to_string());
            return;
        }

        if password != confirm {
            set_error.set("Passwords do not match".to_string());
            return;
        }

        if password.len() < 8 {
            set_error.set("Password must be at least 8 characters long".to_string());
            return;
        }

        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&RegisterArgs {
                username: &username,
                master_pass: &password,
            })
            .unwrap();

            let response = invoke("register", args).await;

            match serde_wasm_bindgen::from_value::<()>(response) {
                Ok(_) => {
                    on_success.call(());
                }
                Err(_) => {
                    set_error.set("Registration failed".to_string());
                }
            }
        });
    };

    view! {
        <div class="w-full max-w-md">
            <form
                on:submit=handle_submit
                class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4"
            >
                <h2 class="text-2xl font-bold mb-6 text-center">"Register"</h2>

                {move || (!error.get().is_empty()).then(||
                    view! {
                        <div class="mb-4 text-red-500 text-sm text-center">
                            {error.get()}
                        </div>
                    }
                )}

                <div class="mb-4">
                    <label
                        for="username"
                        class="block text-gray-700 text-sm font-bold mb-2"
                    >
                        "Username"
                    </label>
                    <input
                        id="username"
                        type="text"
                        placeholder="Choose a username"
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        on:input=move |ev| set_username.set(event_target_value(&ev))
                        prop:value=username
                    />
                </div>

                <div class="mb-4">
                    <label
                        for="password"
                        class="block text-gray-700 text-sm font-bold mb-2"
                    >
                        "Master Password"
                    </label>
                    <input
                        id="password"
                        type="password"
                        placeholder="Choose a strong password"
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        on:input=move |ev| set_password.set(event_target_value(&ev))
                        prop:value=password
                    />
                </div>

                <div class="mb-6">
                    <label
                        for="confirm-password"
                        class="block text-gray-700 text-sm font-bold mb-2"
                    >
                        "Confirm Master Password"
                    </label>
                    <input
                        id="confirm-password"
                        type="password"
                        placeholder="Confirm your password"
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        on:input=move |ev| set_confirm_password.set(event_target_value(&ev))
                        prop:value=confirm_password
                    />
                </div>
                <div class="flex items-center justify-between">
                    <button
                        type="submit"
                        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                    >
                        "Register"
                    </button>
                    <button
                        type="button"
                        on:click=move |_| on_back.call(())
                        class="inline-block align-baseline font-bold text-sm text-blue-500 hover:text-blue-800"
                    >
                        "Back to Login"
                    </button>
                </div>
            </form>
        </div>
    }
}
