use crate::{app::invoke, components::icons::Icon};
use leptos::*;
use serde::Serialize;

#[derive(Serialize)]
struct LoginArgs<'a> {
    username: &'a str,
    #[serde(rename = "masterPass")]
    master_pass: &'a str,
}

#[component]
pub fn Login(#[prop(into)] on_success: Callback<i32>) -> impl IntoView {
    let (username, set_username) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error, set_error) = create_signal(String::new());
    let (show_password, set_show_password) = create_signal(false);

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        let username = username.get();
        let password = password.get();

        if username.is_empty() || password.is_empty() {
            set_error.set("Bitte fülle alle Felder aus".to_string());
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
                    set_error.set("Login fehlgeschlagen".to_string());
                }
            }
        });
    };

    view! {
        <div class="w-full max-w-md ">
            <div class="bg-background-card rounded-lg p-6 shadow-lg">
                <h2 class="text-2xl font-bold mb-6 text-center text-white">"Melde dich an"</h2>

                {move || (!error.get().is_empty()).then(||
                    view! {
                        <div class="mb-4 text-primary-100 text-sm text-center">
                            {error.get()}
                        </div>
                    }
                )}

                <form
                    on:submit=handle_submit
                    class="w-full"
                >
                    <div class="mb-4">
                        <label
                            for="username"
                            class="block text-white text-sm font-bold mb-2"
                        >
                            "Benutzername"
                        </label>
                        <input
                            id="username"
                            type="text"
                            placeholder="Benutzername"
                            class="shadow appearance-none border border-gray-600 rounded w-full py-2 px-3 bg-background text-white leading-tight focus:outline-none focus:border-primary-100"
                            on:input=move |ev| set_username.set(event_target_value(&ev))
                            prop:value=username
                        />
                    </div>

                    <div class="mb-6">
                        <label
                            for="password"
                            class="block text-white text-sm font-bold mb-2"
                        >
                            "Passwort"
                        </label>
                        <div class="relative">
                            <input
                                id="password"
                                type={move || if show_password.get() { "text" } else { "password" }}
                                placeholder="Passwort"
                                class="shadow appearance-none border border-gray-600 rounded w-full py-2 px-3 bg-background text-white leading-tight focus:outline-none focus:border-primary-100 pr-10"
                                on:input=move |ev| set_password.set(event_target_value(&ev))
                                prop:value=password
                            />
                            <button
                                type="button"
                                class="absolute inset-y-0 right-0 px-3 flex items-center"
                                on:click=move |_| set_show_password.update(|show| *show = !*show)
                            >
                                <Icon
                                    icon={if show_password.get() { "eye-slash" } else { "eye" }}
                                    class="w-5 h-5 text-gray-400 hover:text-primary-100"
                                />
                            </button>
                        </div>
                    </div>

                    <div class="flex justify-center">
                        <button
                            type="submit"
                            class="bg-gradient-primary text-white font-bold py-2 px-8 rounded focus:outline-none hover:opacity-90 transition-opacity"
                        >
                            "Einloggen"
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
