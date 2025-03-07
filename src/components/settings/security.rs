use crate::{app::invoke, components::icons::Icon};
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct PasswordGeneratorArgs {
    length: usize,
}

#[derive(Deserialize)]
struct PasswordGeneratorLength {
    default_length: usize,
}

#[component]
pub fn SecuritySettings() -> impl IntoView {
    let (current_password, set_current_password) = create_signal(String::new());
    let (new_password, set_new_password) = create_signal(String::new());
    let (confirm_password, set_confirm_password) = create_signal(String::new());
    let (show_passwords, set_show_passwords) = create_signal(false);
    let (show_master_password, set_show_master_password) = create_signal(false);
    let (error, _set_error) = create_signal(String::new());
    let (auto_logout, set_auto_logout) = create_signal(0);
    let (is_loading, set_is_loading) = create_signal(false);
    let (password_length, set_password_length) = create_signal(0);

    let shield_icon = create_memo(move |_| "shield-check");
    let clock_icon = create_memo(move |_| "clock");

    let show_password_icon = create_memo(move |_| {
        if show_passwords.get() {
            "eye-slash"
        } else {
            "eye"
        }
    });

    let show_master_password_icon = create_memo(move |_| {
        if show_master_password.get() {
            "eye-slash"
        } else {
            "eye"
        }
    });

    let generate_icon = create_memo(move |_| "arrow-path");

    spawn_local(async move {
        set_is_loading.set(true);
        let response = invoke("get_auto_logout_time", wasm_bindgen::JsValue::NULL).await;
        if let Ok(time) = serde_wasm_bindgen::from_value::<u64>(response) {
            set_auto_logout.set(time);
        }

        let response = invoke("get_default_generator_length", wasm_bindgen::JsValue::NULL).await;
        if let Ok(settings) = serde_wasm_bindgen::from_value::<PasswordGeneratorLength>(response) {
            set_password_length.set(settings.default_length);
        }

        set_is_loading.set(false);
    });

    let handle_password_generator = move |_| {
        set_is_loading.set(true);
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&PasswordGeneratorArgs {
                length: password_length.get(),
            })
            .unwrap();
            let response = invoke("generate_password", args).await;
            if let Ok(password) = serde_wasm_bindgen::from_value::<String>(response) {
                set_new_password.set(password.clone());
                set_confirm_password.set(password);
            }

            set_is_loading.set(false);
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
                                        <div class="text-primary-100 text-sm">
                                            {error.get()}
                                        </div>
                                    }
                                )}

                                <fieldset class="space-y-4">
                                    <h3 class="text-lg font-semibold bg-gradient-primary bg-clip-text text-transparent mb-4">
                                        <div class="flex items-center space-x-2">
                                            <Icon icon=shield_icon.into() class="w-5 h-5 text-primary-100" />
                                            <span>"Master-Passwort ändern"</span>
                                        </div>
                                    </h3>

                                    <div>
                                        <label class="block text-white text-sm font-bold mb-2">
                                            "Aktuelles Master-Passwort"
                                        </label>
                                        <div class="relative">
                                            <input
                                                type={move || if show_master_password.get() { "text" } else { "password" }}
                                                class="shadow appearance-none border border-gray-600 rounded w-full py-2 px-3 bg-background text-white leading-tight focus:outline-none focus:border-primary-100 pr-10"
                                                placeholder="Gib dein aktuelles Master-Passwort ein"
                                                on:input=move |ev| set_current_password.set(event_target_value(&ev))
                                                prop:value=current_password
                                            />
                                            <button
                                                type="button"
                                                class="absolute inset-y-0 right-0 px-3 flex items-center"
                                                on:click=move |_| set_show_master_password.update(|show| *show = !*show)
                                            >
                                                <Icon
                                                    icon=show_master_password_icon.into()
                                                    class="w-5 h-5 text-gray-400 hover:text-primary-100"
                                                />
                                            </button>
                                        </div>
                                        <p class="mt-1 text-sm text-gray-400">
                                            "Gib hier dein aktuelles Master-Passwort ein, um die Änderung zu autorisieren"
                                        </p>
                                    </div>

                                    <div>
                                        <label class="block text-white text-sm font-bold mb-2">
                                            "Neues Master-Passwort"
                                        </label>
                                        <div class="relative">
                                            <input
                                                type={move || if show_passwords.get() { "text" } else { "password" }}
                                                class="shadow appearance-none border border-gray-600 rounded w-full py-2 px-3 bg-background text-white leading-tight focus:outline-none focus:border-primary-100 pr-10"
                                                placeholder="Gib dein neues Master-Passwort ein"
                                                on:input=move |ev| set_new_password.set(event_target_value(&ev))
                                                prop:value=new_password
                                            />
                                            <button
                                                type="button"
                                                class="absolute inset-y-0 right-0 px-3 flex items-center"
                                                on:click=move |_| set_show_passwords.update(|show| *show = !*show)
                                            >
                                                <Icon
                                                    icon=show_password_icon.into()
                                                    class="w-5 h-5 text-gray-400 hover:text-primary-100"
                                                />
                                            </button>
                                        </div>
                                        <p class="mt-1 text-sm text-gray-400">
                                            "Wähle ein sicheres neues Master-Passwort mit mindestens 12 Zeichen"
                                        </p>
                                    </div>

                                    <button
                                        type="button"
                                        class="w-full flex justify-center items-center bg-background border border-primary-100 hover:bg-primary-400/10 text-white font-bold py-2 px-4 rounded focus:outline-none transition-all duration-200"
                                        on:click=handle_password_generator
                                    >
                                        <Icon icon=generate_icon.into() class="w-5 h-5 mr-2 text-primary-100" />
                                        "Generiere sicheres Passwort"
                                    </button>

                                    <div>
                                        <label class="block text-white text-sm font-bold mb-2">
                                            "Bestätige neues Master-Passwort"
                                        </label>
                                        <div class="relative">
                                            <input
                                                type={move || if show_passwords.get() { "text" } else { "password" }}
                                                class="shadow appearance-none border border-gray-600 rounded w-full py-2 px-3 bg-background text-white leading-tight focus:outline-none focus:border-primary-100 pr-10"
                                                placeholder="Bestätige dein neues Master-Passwort"
                                                on:input=move |ev| set_confirm_password.set(event_target_value(&ev))
                                                prop:value=confirm_password
                                            />
                                            <button
                                                type="button"
                                                class="absolute inset-y-0 right-0 px-3 flex items-center"
                                                on:click=move |_| set_show_passwords.update(|show| *show = !*show)
                                            >
                                                <Icon
                                                    icon=show_password_icon.into()
                                                    class="w-5 h-5 text-gray-400 hover:text-primary-100"
                                                />
                                            </button>
                                        </div>
                                        <p class="mt-1 text-sm text-gray-400">
                                            "Gib dein neues Master-Passwort erneut ein, um Tippfehler zu vermeiden"
                                        </p>
                                    </div>
                                    <div class="flex justify-end pt-4 border-t border-gray-600">
                                            <button
                                                type="button"
                                                class="bg-gradient-primary text-white font-bold py-2 px-8 rounded focus:outline-none hover:opacity-90 transition-opacity"
                                                on:click=move |_| {}
                                            >
                                                "Master-Passwort ändern"
                                            </button>
                                        </div>
                                </fieldset>

                                <fieldset class="space-y-4">
                                    <h3 class="text-lg font-semibold bg-gradient-primary bg-clip-text text-transparent mb-4">
                                        <div class="flex items-center space-x-2">
                                            <Icon icon=clock_icon.into() class="w-5 h-5 text-primary-100" />
                                            <span>"Automatischer Logout"</span>
                                        </div>
                                    </h3>
                                    <div class="flex items-center space-x-4">
                                        <input
                                            type="number"
                                            min="1"
                                            max="60"
                                            class="w-20 p-2 rounded bg-background text-white border border-gray-600 focus:border-primary-100 focus:outline-none"
                                            on:input=move |ev| set_auto_logout.set(event_target_value(&ev).parse().unwrap_or(15))
                                            prop:value=auto_logout
                                        />
                                        <span class="text-white">"Minuten"</span>
                                    </div>
                                    <p class="mt-1 text-sm text-gray-400">
                                        "Die Zeit in Minuten, nach der du aus Sicherheitsgründen automatisch ausgeloggt wirst (1-60 Minuten)"
                                    </p>
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
