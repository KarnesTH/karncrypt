use crate::{
    app::invoke,
    components::{
        icons::Icon,
        password_manager::password_dialog::{DialogAction, PasswordDialog},
    },
};
use leptos::*;
use serde::{Deserialize, Serialize};

use super::TableItemArgs;

#[derive(Clone, PartialEq)]
pub enum ModalMode {
    Add,
    Edit(TableItemArgs),
}

#[derive(Serialize)]
struct GeneratePasswordArgs {
    length: usize,
}

#[derive(Serialize)]
struct DecryptPasswordArgs<'a> {
    #[serde(rename = "encryptedPassword")]
    encrypted_password: &'a str,
}

#[derive(Deserialize)]
struct GeneratorLengthResponse {
    default_length: usize,
}

#[component]
pub fn PasswordModal(
    #[prop(into)] mode: ModalMode,
    #[prop(into)] on_save: Callback<TableItemArgs>,
    #[prop(into)] on_close: Callback<()>,
) -> impl IntoView {
    let (length, set_length) = create_signal(16);
    let (service, set_service) = create_signal(String::new());
    let (username, set_username) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (url, set_url) = create_signal(String::new());
    let (notes, set_notes) = create_signal(String::new());
    let (show_password, set_show_password) = create_signal(false);
    let (show_password_dialog, set_show_password_dialog) = create_signal(false);
    let (decrypted_password, set_decrypted_password) = create_signal(String::new());
    let (password_verified, set_password_verified) = create_signal(false);
    let (is_original_password, set_is_original_password) = create_signal(false);

    let eye_icon = create_memo(move |_| {
        if show_password.get() {
            "eye-slash"
        } else {
            "eye"
        }
    });
    let generate_icon = create_memo(move |_| "arrow-path");
    let service_icon = create_memo(move |_| "bookmark");
    let user_icon = create_memo(move |_| "user");
    let key_icon = create_memo(move |_| "key");
    let link_icon = create_memo(move |_| "link");
    let note_icon = create_memo(move |_| "document-text");
    let cancel_icon = create_memo(move |_| "x-mark");
    let add_icon = create_memo(move |_| "plus");
    let edit_icon = create_memo(move |_| "pencil-square");

    let mode = create_memo(move |_| mode.clone());

    let modal_icon = create_memo(move |_| {
        if matches!(mode.get(), ModalMode::Add) {
            add_icon.into()
        } else {
            edit_icon.into()
        }
    });

    let input_type = move || {
        if show_password.get() {
            "text"
        } else {
            "password"
        }
    };

    let modal_title = create_memo(move |_| match mode.get() {
        ModalMode::Add => "Passwort hinzufügen",
        ModalMode::Edit(_) => "Passwort bearbeiten",
    });

    let password_value = move || {
        if matches!(mode.get(), ModalMode::Edit(_)) && password_verified.get() {
            decrypted_password.get()
        } else {
            password.get()
        }
    };

    spawn_local(async move {
        let response = invoke("get_default_generator_length", wasm_bindgen::JsValue::NULL).await;
        if let Ok(generator) = serde_wasm_bindgen::from_value::<GeneratorLengthResponse>(response) {
            set_length.set(generator.default_length);
        }
    });

    if let ModalMode::Edit(ref item) = mode.get() {
        set_service.set(item.service.clone());
        set_username.set(item.username.clone());
        set_password.set(item.password.clone());
        set_url.set(item.url.clone());
        set_notes.set(item.notes.clone());
        set_is_original_password.set(true);
    }

    let handle_password_change = move |new_password: String| {
        set_password.set(new_password);
        set_is_original_password.set(false);
        set_password_verified.set(false);
    };

    view! {
        <div class="absolute inset-0 flex items-center justify-center bg-black bg-opacity-50 overflow-auto p-4">
            <div class="bg-background-card rounded-lg p-6 w-full max-w-md max-h-[90%] flex flex-col">
                <div class="flex justify-between items-center mb-6">
                    <h2 class="text-xl font-bold bg-gradient-primary bg-clip-text text-transparent flex items-center">
                        <div class="flex items-center">
                            <Icon
                                icon=modal_icon.get()
                                class="w-8 h-8 mr-2 text-primary-100"
                            />
                            {modal_title.get()}
                        </div>
                    </h2>
                    <button
                        class="text-gray-400 hover:text-white"
                        on:click=move |_| on_close.call(())
                    >
                        <Icon icon=cancel_icon.into() class="w-5 h-5" />
                    </button>
                </div>

                <div class="flex-1 overflow-y-auto">
                    <div class="space-y-4">
                        <div>
                            <label class="block text-white text-sm font-bold mb-2 flex items-center">
                                <Icon icon=service_icon.into() class="w-4 h-4 mr-2 text-primary-100" />
                                "Service"
                            </label>
                            <input
                                type="text"
                                value=service
                                class="w-full p-2 rounded bg-background text-white border border-gray-600 focus:outline-none focus:border-primary-100"
                                on:input=move |ev| set_service.set(event_target_value(&ev))
                            />
                        </div>

                        <div>
                            <label class="block text-white text-sm font-bold mb-2 flex items-center">
                                <Icon icon=user_icon.into() class="w-4 h-4 mr-2 text-primary-100" />
                                "Benutzername"
                            </label>
                            <input
                                type="text"
                                value=username
                                class="w-full p-2 rounded bg-background text-white border border-gray-600 focus:outline-none focus:border-primary-100"
                                on:input=move |ev| set_username.set(event_target_value(&ev))
                            />
                        </div>

                        <div>
                            <label class="block text-white text-sm font-bold mb-2 flex items-center">
                                <Icon icon=key_icon.into() class="w-4 h-4 mr-2 text-primary-100" />
                                "Passwort"
                            </label>
                            <div class="relative">
                                <input
                                    type=input_type
                                    value=password_value
                                    class="w-full p-2 rounded bg-background text-white border border-gray-600 focus:outline-none focus:border-primary-100 pr-20"
                                    on:input=move |ev| handle_password_change(event_target_value(&ev))
                                />
                                <div class="absolute inset-y-0 right-0 flex">
                                    <button
                                        type="button"
                                        class="px-2 text-gray-400 hover:text-primary-100"
                                        on:click=move |_| {
                                            if matches!(mode.get(), ModalMode::Edit(_)) && is_original_password.get() && !password_verified.get() {
                                                set_show_password_dialog.set(true);
                                            } else {
                                                set_show_password.update(|show| *show = !*show)
                                            }
                                        }
                                    >
                                        <Icon icon=eye_icon.into() class="w-5 h-5" />
                                    </button>
                                </div>
                            </div>
                        </div>

                        <button
                            class="w-full flex justify-center items-center bg-background border border-primary-100 hover:bg-primary-400/10 text-white font-bold py-2 px-4 rounded focus:outline-none transition-all duration-200"
                            on:click=move |_| {
                                spawn_local(async move {
                                    let args = serde_wasm_bindgen::to_value(&GeneratePasswordArgs {
                                        length: length.get(),
                                    }).unwrap();

                                    let response = invoke("generate_password", args).await;

                                    if let Ok(new_pass) = serde_wasm_bindgen::from_value(response) {
                                        handle_password_change(new_pass);
                                    }
                                });
                            }
                        >
                            <Icon icon=generate_icon.into() class="w-5 h-5 mr-2" />
                            "Generiere sicheres Passwort"
                        </button>

                        <div>
                            <label class="block text-white text-sm font-bold mb-2 flex items-center">
                                <Icon icon=link_icon.into() class="w-4 h-4 mr-2 text-primary-100" />
                                "URL"
                            </label>
                            <input
                                type="url"
                                value=url
                                class="w-full p-2 rounded bg-background text-white border border-gray-600 focus:outline-none focus:border-primary-100"
                                on:input=move |ev| set_url.set(event_target_value(&ev))
                            />
                        </div>

                        <div>
                            <label class="block text-white text-sm font-bold mb-2 flex items-center">
                                <Icon icon=note_icon.into() class="w-4 h-4 mr-2 text-primary-100" />
                                "Notizen"
                            </label>
                            <textarea
                                prop:value=notes
                                class="w-full p-2 rounded bg-background text-white border border-gray-600 focus:outline-none focus:border-primary-100 resize-none h-24"
                                on:input=move |ev| set_notes.set(event_target_value(&ev))
                            />
                        </div>
                    </div>
                </div>

                <div class="flex justify-end space-x-3 mt-6 pt-4 border-t border-gray-600">
                    <button
                        class="px-4 py-2 text-white hover:text-primary-100 flex items-center"
                        on:click=move |_| on_close.call(())
                    >
                        <Icon icon=cancel_icon.into() class="w-5 h-5 mr-2" />
                        "Abbrechen"
                    </button>
                    <button
                        class="bg-gradient-primary text-white px-4 py-2 rounded hover:opacity-90 flex items-center"
                        on:click=move |_| {
                            let new_item = TableItemArgs {
                                id: match mode.get() {
                                    ModalMode::Edit(ref item) => item.id,
                                    ModalMode::Add => 0,
                                },
                                service: service.get(),
                                username: username.get(),
                                password: password.get(),
                                url: url.get(),
                                notes: notes.get(),
                            };
                            on_save.call(new_item);
                        }
                    >
                        {move || {
                            let icon = if matches!(mode.get(), ModalMode::Add) {
                                add_icon
                            } else {
                                edit_icon
                            };
                            view! {
                                <>
                                    <Icon
                                        icon=icon.into()
                                        class="w-5 h-5 mr-2"
                                    />
                                    "Speichern"
                                </>
                            }
                        }}
                    </button>
                </div>
            </div>
            {move || {
                if show_password_dialog.get() {
                    view! {
                        <PasswordDialog
                        action=DialogAction::Verify
                        on_verify=move |verified| {
                            if verified {
                                let password = password.get().clone();
                                spawn_local(async move {
                                    let args = serde_wasm_bindgen::to_value(&DecryptPasswordArgs {
                                        encrypted_password: &password,
                                    })
                                    .unwrap();
                                    if let Ok(decrypted) =
                                        serde_wasm_bindgen::from_value(invoke("decrypt_password", args).await)
                                    {
                                        set_decrypted_password.set(decrypted);
                                    }
                                });
                            }
                            set_password_verified.set(verified);
                            set_show_password.set(true);
                        }
                            on_close=move |_| set_show_password_dialog.set(false)
                        />
                    }.into_view()
                } else {
                    view! { <div/> }.into_view()
                }
            }}
        </div>
    }
}
