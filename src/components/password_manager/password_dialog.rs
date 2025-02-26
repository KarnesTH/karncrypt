use crate::{app::invoke, components::icons::Icon};
use leptos::*;
use serde::Serialize;

#[derive(Serialize)]
struct VerifyPasswordArgs<'a> {
    master_pass: &'a str,
}

#[derive(Serialize)]
struct ClipboardArgs<'a> {
    text: &'a str,
}

#[component]
pub fn PasswordDialog(password: String, #[prop(into)] on_close: Callback<()>) -> impl IntoView {
    let password = create_signal(password).0;
    let (master_password, set_master_password) = create_signal(String::new());
    let (error, set_error) = create_signal(String::new());
    let (password_verified, set_password_verified) = create_signal(false);
    let (is_copied, set_is_copied) = create_signal(false);

    let copy_icon = create_memo(move |_| {
        if is_copied.get() {
            "check"
        } else {
            "clipboard"
        }
    });

    let x_mark_icon = create_memo(move |_| "x-mark");

    let verify_password = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let master_pass = master_password.get();

        if master_pass.is_empty() {
            set_error.set("Bitte gib dein Master-Passwort ein".into());
            return;
        }

        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&VerifyPasswordArgs {
                master_pass: &master_pass,
            })
            .unwrap();

            let response = invoke("verify_master_password", args).await;
            match serde_wasm_bindgen::from_value::<bool>(response) {
                Ok(true) => {
                    set_password_verified.set(true);
                    set_error.set(String::new());
                }
                _ => {
                    set_error.set("Falsches Master-Passwort".into());
                }
            }
        });
    };

    let copy_to_clipboard = move || {
        let pass = password.get();
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&ClipboardArgs { text: &pass }).unwrap();
            if serde_wasm_bindgen::from_value::<()>(invoke("copy_to_clipboard", args).await).is_ok()
            {
                set_is_copied.set(true);
            }
        });
    };

    view! {
        <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center">
            <div class="bg-background-card rounded-lg p-6 w-full max-w-md">
                <div class="flex justify-between items-center mb-6">
                    <h2 class="text-xl font-bold text-white">
                        "Passwort anzeigen"
                    </h2>
                    <button
                        class="text-gray-400 hover:text-white"
                        on:click=move |_| on_close.call(())
                    >
                        <Icon icon=x_mark_icon.into() class="w-5 h-5" />
                    </button>
                </div>

                {if !password_verified.get() {
                    view! {
                        <form on:submit=verify_password class="space-y-4">
                            <div>
                                <label class="block text-white text-sm font-bold mb-2">
                                    "Master-Passwort"
                                </label>
                                <input
                                    type="password"
                                    class="w-full p-2 rounded bg-background text-white border border-gray-600 focus:outline-none focus:border-primary-100"
                                    placeholder="Gib dein Master-Passwort ein"
                                    on:input=move |ev| set_master_password.set(event_target_value(&ev))
                                    prop:value=master_password
                                />
                            </div>

                            {if !error.get().is_empty() {
                                view! {
                                    <div class="text-primary-100 text-sm">
                                        {error.get()}
                                    </div>
                                }
                            } else {
                                view! { <div/> }
                            }}

                            <button
                                type="submit"
                                class="w-full bg-gradient-primary text-white px-4 py-2 rounded hover:opacity-90"
                            >
                                "Bestätigen"
                            </button>
                        </form>
                        }.into_view()
                } else {
                    view! {
                        <div class="space-y-4">
                            <div>
                                <label class="block text-white text-sm font-bold mb-2">
                                    "Passwort"
                                </label>
                                <div class="flex">
                                    <input
                                        type="text"
                                        readonly
                                        class="flex-grow p-2 rounded-l bg-background text-white border border-r-0 border-gray-600 focus:outline-none"
                                        prop:value=password.get()
                                    />
                                    <button
                                        class="px-4 rounded-r bg-primary-200 hover:bg-primary-300 text-white border border-l-0 border-primary-200 hover:border-primary-300"
                                        on:click=move |_| copy_to_clipboard()
                                        on:mouseleave=move |_| set_is_copied.set(false)
                                    >
                                        <Icon icon=copy_icon.into() class="w-5 h-5" />
                                    </button>
                                </div>
                            </div>
                        </div>
                    }.into_view()
                }}
            </div>
        </div>
    }
}
