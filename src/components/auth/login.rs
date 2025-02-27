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
pub fn Login(#[prop(into)] on_success: Callback<()>) -> impl IntoView {
    let (username, set_username) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error, set_error) = create_signal(String::new());
    let (show_password, set_show_password) = create_signal(false);

    let show_password_icon = create_memo(move |_| {
        if show_password.get() {
            "eye-slash"
        } else {
            "eye"
        }
    });
    let user_icon = create_memo(move |_| "user");
    let key_icon = create_memo(move |_| "key");
    let login_icon = create_memo(move |_| "arrow-right-end-on-rectangle");

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

            match serde_wasm_bindgen::from_value::<()>(response) {
                Ok(_) => {
                    on_success.call(());
                }
                Err(_) => {
                    set_error.set("Login fehlgeschlagen".to_string());
                }
            }
        });
    };

    view! {
        <div class="w-full max-w-md">
            <div class="bg-background-card rounded-lg p-6 shadow-lg">
                <h2 class="text-2xl font-bold mb-6 text-center bg-gradient-primary bg-clip-text text-transparent">
                    "Willkommen zurück"
                </h2>

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
                            class="block text-white text-sm font-bold mb-2 flex items-center"
                        >
                            <Icon icon=user_icon.into() class="w-4 h-4 mr-2 text-primary-100" />
                            "Benutzername"
                        </label>
                        <input
                            id="username"
                            type="text"
                            placeholder="Dein Benutzername"
                            class="shadow appearance-none border border-gray-600 rounded w-full py-2 px-3 bg-background text-white leading-tight focus:outline-none focus:border-primary-100"
                            on:input=move |ev| set_username.set(event_target_value(&ev))
                            prop:value=username
                        />
                    </div>

                    <div class="mb-6">
                        <label
                            for="password"
                            class="block text-white text-sm font-bold mb-2 flex items-center"
                        >
                            <Icon icon=key_icon.into() class="w-4 h-4 mr-2 text-primary-100" />
                            "Master-Passwort"
                        </label>
                        <div class="relative">
                            <input
                                id="password"
                                type={move || if show_password.get() { "text" } else { "password" }}
                                placeholder="Dein Master-Passwort"
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
                                    icon=show_password_icon.into()
                                    class="w-5 h-5 text-gray-400 hover:text-primary-100"
                                />
                            </button>
                        </div>
                    </div>

                    <div class="flex justify-center">
                        <button
                            type="submit"
                            class="bg-gradient-primary text-white font-bold py-2 px-8 rounded focus:outline-none hover:opacity-90 transition-opacity flex items-center"
                        >
                            <span>"Anmelden"</span>
                            <Icon icon=login_icon.into() class="w-5 h-5 ml-2" />
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
