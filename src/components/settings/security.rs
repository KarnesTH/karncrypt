use leptos::*;

use crate::components::icons::Icon;

#[component]
pub fn SecuritySettings() -> impl IntoView {
    let (current_password, set_current_password) = create_signal(String::new());
    let (new_password, set_new_password) = create_signal(String::new());
    let (confirm_password, set_confirm_password) = create_signal(String::new());
    let (show_passwords, set_show_passwords) = create_signal(false);
    let (error, _set_error) = create_signal(String::new());
    let (auto_logout, set_auto_logout) = create_signal(15);

    let shield_icon = create_memo(move |_| "shield-check");
    let clock_icon = create_memo(move |_| "clock");

    let show_password_icon = create_memo(move |_| {
        if show_passwords.get() {
            "eye-slash"
        } else {
            "eye"
        }
    });

    let generate_icon = create_memo(move |_| "arrow-path");

    view! {
        <div class="flex justify-center">
            <div class="max-w-xl w-full space-y-8">
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
                                    type={move || if show_passwords.get() { "text" } else { "password" }}
                                    class="shadow appearance-none border border-gray-600 rounded w-full py-2 px-3 bg-background text-white leading-tight focus:outline-none focus:border-primary-100 pr-10"
                                    placeholder="Gib dein aktuelles Master-Passwort ein"
                                    on:input=move |ev| set_current_password.set(event_target_value(&ev))
                                    prop:value=current_password
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
                        </div>

                        <button
                            type="button"
                            class="w-full flex justify-center items-center bg-background border border-primary-100 hover:bg-primary-400/10 text-white font-bold py-2 px-4 rounded focus:outline-none transition-all duration-200"
                            on:click=move |_| {
                                // TODO: Password Generator Logic
                            }
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
            </div>
        </div>
    }
}
