use crate::app::invoke;
use leptos::*;
use serde::Serialize;

#[derive(Serialize)]
struct LoginArgs<'a> {
    username: &'a str,
    master_pass: &'a str,
}

#[component]
pub fn Login(
    #[prop(into)] on_success: Callback<i32>,
    #[prop(into)] on_register: Callback<()>,
) -> impl IntoView {
    let (username, set_username) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error, set_error) = create_signal(String::new());

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        let username = username.get();
        let password = password.get();

        if username.is_empty() || password.is_empty() {
            set_error.set("Please fill in all fields".to_string());
            return;
        }

        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&LoginArgs {
                username: &username,
                master_pass: &password,
            })
            .unwrap();

            let response = invoke("login", args).await;

            match serde_wasm_bindgen::from_value::<i32>(response) {
                Ok(user_id) => {
                    on_success.call(user_id);
                }
                Err(_) => {
                    set_error.set("Login failed".to_string());
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
                <h2 class="text-2xl font-bold mb-6 text-center">"Login"</h2>

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
                        placeholder="Enter username"
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        on:input=move |ev| set_username.set(event_target_value(&ev))
                        prop:value=username
                    />
                </div>

                <div class="mb-6">
                    <label
                        for="password"
                        class="block text-gray-700 text-sm font-bold mb-2"
                    >
                        "Master Password"
                    </label>
                    <input
                        id="password"
                        type="password"
                        placeholder="Enter master password"
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        on:input=move |ev| set_password.set(event_target_value(&ev))
                        prop:value=password
                    />
                </div>

                <div class="flex items-center justify-between">
                    <button
                        type="submit"
                        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                    >
                        "Sign In"
                    </button>
                    <button
                        type="button"
                        on:click=move |_| on_register.call(())
                        class="inline-block align-baseline font-bold text-sm text-blue-500 hover:text-blue-800"
                    >
                        "Register"
                    </button>
                </div>
            </form>
        </div>
    }
}
