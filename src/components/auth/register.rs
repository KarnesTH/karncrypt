use crate::{app::invoke, components::icons::Icon};
use leptos::*;
use serde::Serialize;

#[derive(Serialize)]
struct RegisterArgs<'a> {
    username: &'a str,
    #[serde(rename = "masterPass")]
    master_pass: &'a str,
}

#[derive(Serialize)]
struct GeneratePasswordArgs {
    length: usize,
}

#[component]
pub fn Register(#[prop(into)] on_success: Callback<()>) -> impl IntoView {
    let (username, set_username) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (confirm_password, set_confirm_password) = create_signal(String::new());
    let (error, set_error) = create_signal(String::new());
    let (show_password, set_show_password) = create_signal(false);

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        let username = username.get();
        let password = password.get();
        let confirm = confirm_password.get();

        if username.is_empty() || password.is_empty() {
            set_error.set("Bitte fülle alle Felder aus.".to_string());
            return;
        }

        if password != confirm {
            set_error.set("Passwörter sind nicht gleich.".to_string());
            return;
        }

        if password.len() < 8 {
            set_error.set("Passwörter müssen mindestens 8 Zeichen haben".to_string());
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
                    set_error.set("Registrierung Fehlgeschlagen.".to_string());
                }
            }
        });
    };

    view! {
        <form
            on:submit=handle_submit
            class="w-full"
        >
            {move || (!error.get().is_empty()).then(||
                view! {
                    <div class="mb-4 text-primary-100 text-sm text-center">
                        {error.get()}
                    </div>
                }
            )}
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
                    placeholder="Wähle einen Benutzernamen"
                    class="shadow appearance-none border border-gray-600 rounded w-full py-2 px-3 bg-background text-white leading-tight focus:outline-none focus:border-primary-100"
                    on:input=move |ev| set_username.set(event_target_value(&ev))
                    prop:value=username
                />
            </div>
            <div class="mb-4">
                <label
                    for="password"
                    class="block text-white text-sm font-bold mb-2"
                >
                    "Master Passwort"
                </label>
                <div class="relative">
                    <input
                        id="password"
                        type={move || if show_password.get() { "text" } else { "password" }}
                        placeholder="Wähle ein starkes Passwort"
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
            <div class="mb-4">
                <button
                    type="button"
                    class="w-full flex justify-center items-center bg-background border border-primary-100 hover:bg-primary-400/10 text-white font-bold py-2 px-4 rounded focus:outline-none transition-all duration-200"
                    on:click=move |_| {
                        spawn_local(async move {
                            let args = serde_wasm_bindgen::to_value(&GeneratePasswordArgs {
                                length: 16
                            }).unwrap();

                            let response = invoke("generate_password", args).await;

                            match serde_wasm_bindgen::from_value::<String>(response) {
                                Ok(new_pass) => {
                                    set_password.set(new_pass.clone());
                                    set_confirm_password.set(new_pass);
                                }
                                Err(_) => {
                                    set_error.set("Fehler beim Generieren des Passworts".to_string());
                                }
                            }
                        });
                    }
                >
                    <Icon icon="arrow-path" class="w-5 h-5 mr-2 text-primary-100" />
                    "Generiere sicheres Passwort"
                </button>
            </div>
            <div class="mb-6">
                <label
                    for="confirm-password"
                    class="block text-white text-sm font-bold mb-2"
                >
                    "Bestätige dein Passwort"
                </label>
                <div class="relative">
                    <input
                        id="confirm-password"
                        type={move || if show_password.get() { "text" } else { "password" }}
                        placeholder="Bestätige dein Passwort"
                        class="shadow appearance-none border border-gray-600 rounded w-full py-2 px-3 bg-background text-white leading-tight focus:outline-none focus:border-primary-100"
                        on:input=move |ev| set_confirm_password.set(event_target_value(&ev))
                        prop:value=confirm_password
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
                    "Erstellen"
                </button>
            </div>
        </form>
    }
}
