use crate::{app::invoke, components::icons::Icon};
use leptos::*;
use serde::Serialize;

#[derive(Clone)]
pub enum DialogAction {
    Verify,
    CreateBackup,
    ExportPasswords,
    RestoreBackup,
}

#[derive(Serialize)]
struct PasswordDialogArgs<'a> {
    #[serde(rename = "masterPass")]
    master_pass: &'a str,
}

#[component]
pub fn PasswordDialog(
    #[prop(into)] on_close: Callback<()>,
    #[prop(into)] on_verify: Callback<bool>,
    #[prop(into)] action: DialogAction,
) -> impl IntoView {
    let (master_password, set_master_password) = create_signal(String::new());
    let (error, set_error) = create_signal(String::new());

    let exit_icon = create_memo(move |_| "x-mark");
    let key_icon = create_memo(move |_| "key");
    let master_key_icon = create_memo(move |_| "shield-check");
    let check_icon = create_memo(move |_| "check");

    let handle_password = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let master_pass = master_password.get();
        let action = action.clone();

        if master_pass.is_empty() {
            set_error.set("Bitte gib dein Master-Passwort ein".into());
            return;
        }

        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&PasswordDialogArgs {
                master_pass: &master_pass,
            })
            .unwrap();

            match action {
                DialogAction::Verify => {
                    let response = invoke("verify_master_password", args).await;
                    match serde_wasm_bindgen::from_value::<bool>(response) {
                        Ok(true) => {
                            set_error.set("".into());
                            on_verify.call(true);
                            on_close.call(());
                        }
                        _ => {
                            set_error.set("Falsches Master-Passwort".into());
                        }
                    }
                }
                DialogAction::CreateBackup => {
                    let response = invoke("create_backup", args).await;
                    if response.is_null() || response.as_bool().unwrap_or(false) {
                        set_error.set("Backup erfolgreich erstellt!".into());
                        on_close.call(());
                    } else {
                        let error_msg = response
                            .as_string()
                            .unwrap_or("Unbekannter Fehler".to_string());
                        set_error.set(format!("Backup fehlgeschlagen: {}", error_msg));
                    }
                }
                DialogAction::ExportPasswords => {
                    let response = invoke("export_passwords", args).await;

                    if response.is_null() || response.as_bool().unwrap_or(false) {
                        set_error.set("Export erfolgreich".into());
                        on_close.call(());
                    } else {
                        let error_msg = response
                            .as_string()
                            .unwrap_or("Unbekannter Fehler".to_string());
                        set_error.set(format!("Export fehlgeschlagen: {}", error_msg));
                    }
                }
                DialogAction::RestoreBackup => {
                    let response = invoke("restore_backup", args).await;

                    if response.is_null() || response.as_bool().unwrap_or(false) {
                        set_error.set("Backup erfolgreich wiederhergestellt".into());
                        on_close.call(());
                    } else {
                        let error_msg = response
                            .as_string()
                            .unwrap_or("Unbekannter Fehler".to_string());
                        set_error.set(format!("Wiederherstellung fehlgeschlagen: {}", error_msg));
                    }
                }
            };
        });
    };

    view! {
        <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center">
            <div class="bg-background-card rounded-lg p-6 w-full max-w-md">
                <div class="flex justify-between items-center mb-6">
                    <h2 class="text-xl font-bold bg-gradient-primary bg-clip-text text-transparent flex items-center">
                        <div class="flex items-center">
                            <Icon
                                icon=master_key_icon.into()
                                class="w-8 h-8 mr-2 text-primary-100"
                            />
                            "Master-Passwort"
                        </div>
                    </h2>
                    <button
                        class="text-gray-400 hover:text-white"
                        on:click=move |_| on_close.call(())
                    >
                        <Icon icon=exit_icon.into() class="w-5 h-5" />
                    </button>
                </div>

                <form on:submit=handle_password class="space-y-4">
                    <div>
                        <label class="block text-white text-sm font-bold mb-2 flex items-center">
                            <Icon icon=key_icon.into() class="w-4 h-4 mr-2 text-primary-100" />
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

                    {move || {
                        if !error.get().is_empty() {
                            view! {
                                <div class="text-primary-100 text-sm">
                                    {error.get()}
                                </div>
                            }
                        } else {
                            view! { <div/> }
                        }
                    }}

                    <button
                        type="submit"
                        class="w-full bg-gradient-primary text-white px-4 py-2 rounded hover:opacity-90 flex items-center justify-center"
                    >
                        <Icon icon=check_icon.into() class="w-5 h-5 mr-2" />
                        "Bestätigen"
                    </button>
                </form>
            </div>
        </div>
    }
}
