use crate::{app::invoke, components::icons::Icon};
use leptos::*;
use serde::Serialize;

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

#[component]
pub fn PasswordModal(
    #[prop(into)] mode: ModalMode,
    #[prop(into)] on_save: Callback<TableItemArgs>,
    #[prop(into)] on_close: Callback<()>,
) -> impl IntoView {
    let (service, set_service) = create_signal(String::new());
    let (username, set_username) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (url, set_url) = create_signal(String::new());
    let (notes, set_notes) = create_signal(String::new());
    let (show_password, set_show_password) = create_signal(false);

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

    if let ModalMode::Edit(ref item) = mode.get() {
        set_service.set(item.service.clone());
        set_username.set(item.username.clone());
        set_password.set(item.password.clone());
        set_url.set(item.url.clone());
        set_notes.set(item.notes.clone());
    }

    view! {
        <div class="absolute inset-0 flex items-center justify-center bg-black bg-opacity-50 overflow-auto p-4">
            <div class="bg-background-card rounded-lg p-6 w-full max-w-md max-h-[90%] flex flex-col">
                <div class="flex justify-between items-center mb-6">
                    <h2 class="text-xl font-bold bg-gradient-primary bg-clip-text text-transparent flex items-center">
                        <div class="flex items-center">
                            <Icon
                                icon={if matches!(mode.get(), ModalMode::Add) {
                                    add_icon.into()
                                } else {
                                    edit_icon.into()
                                }}
                                class="w-8 h-8 mr-2 text-primary-100"
                            />
                            {move || match mode.get() {
                                ModalMode::Add => "Passwort hinzufügen",
                                ModalMode::Edit(_) => "Passwort bearbeiten",
                            }}
                        </div>
                    </h2>
                    <button
                        class="text-gray-400 hover:text-white"
                        on:click=move |_| on_close.call(())
                    >
                        <Icon icon=create_memo(move |_| "x-mark").into() class="w-5 h-5" />
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
                                    type=move || if show_password.get() { "text" } else { "password" }
                                    prop:value=password
                                    class="w-full p-2 rounded bg-background text-white border border-gray-600 focus:outline-none focus:border-primary-100 pr-20"
                                    on:input=move |ev| set_password.set(event_target_value(&ev))
                                />
                                <div class="absolute inset-y-0 right-0 flex">
                                    <button
                                        type="button"
                                        class="px-2 text-gray-400 hover:text-primary-100"
                                        on:click=move |_| set_show_password.update(|show| *show = !*show)
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
                                        length: 16
                                    }).unwrap();

                                    let response = invoke("generate_password", args).await;

                                    if let Ok(new_pass) = serde_wasm_bindgen::from_value(response) {
                                        set_password.set(new_pass);
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
                                value=notes
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
        </div>
    }
}
