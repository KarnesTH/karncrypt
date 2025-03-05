use crate::{
    app::invoke,
    components::{icons::Icon, password_manager::password_dialog::DialogAction},
};
use leptos::*;
use serde::{Deserialize, Serialize};

use super::password_dialog::PasswordDialog;

#[derive(Clone, Serialize, PartialEq, Deserialize)]
pub struct TableItemArgs {
    pub id: i32,
    pub service: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
}

#[derive(Serialize)]
struct DecryptPasswordArgs<'a> {
    #[serde(rename = "encryptedPassword")]
    encrypted_password: &'a str,
}

#[component]
pub fn TableItem(
    item: TableItemArgs,
    on_edit: Callback<i32>,
    on_delete: Callback<i32>,
) -> impl IntoView {
    let (show_password_dialog, set_show_password_dialog) = create_signal(false);
    let (is_copied_username, set_is_copied_username) = create_signal(false);
    let (password_verified, set_password_verified) = create_signal(false);
    let (is_copied_password, set_is_copied_password) = create_signal(false);
    let (decrypted_password, set_decrypted_password) = create_signal(String::new());

    let eye_icon = create_memo(move |_| {
        if password_verified.get() {
            "eye-slash"
        } else {
            "eye"
        }
    });
    let pencil_icon = create_memo(move |_| "pencil-square");
    let trash_icon = create_memo(move |_| "trash");
    let username_icon = create_memo(move |_| {
        if is_copied_username.get() {
            "check"
        } else {
            "clipboard"
        }
    });

    let clipboard_icon = create_memo(move |_| {
        if is_copied_password.get() {
            "check"
        } else {
            "clipboard"
        }
    });

    let item = create_memo(move |_| item.clone());

    view! {
        <tr class="hover:bg-background">
            <td class="p-4 text-white whitespace-nowrap">{move || item.get().service}</td>
            <td class="p-4">
                <div class="flex items-center text-white whitespace-nowrap">
                    {move || item.get().username}
                    <button
                        class="ml-2 text-gray-400 hover:text-primary-100"
                        on:click=move |_| {
                            let username = item.get().username.clone();
                            spawn_local(async move {
                                let args = serde_wasm_bindgen::to_value(&ClipboardArgs {
                                    text: &username
                                }).unwrap();
                                if serde_wasm_bindgen::from_value::<()>(invoke("copy_to_clipboard", args).await).is_ok() {
                                    set_is_copied_username.set(true);
                                }
                            });
                        }
                        on:mouseleave=move |_| set_is_copied_username.set(false)
                    >
                        <Icon icon=username_icon.into() class="w-4 h-4" />
                    </button>
                </div>
            </td>
            <td class="p-4">
                <div class="flex items-center text-white">
                    {move || {
                        if !password_verified.get() {
                            view! {
                                <span class="whitespace-nowrap">"••••••••"</span>
                            }.into_view()
                        } else {
                            view! {
                                <span class="whitespace-nowrap">{decrypted_password}</span>
                            }.into_view()
                        }
                    }}

                    <button
                        class="ml-2 text-gray-400 hover:text-primary-100"
                        on:click=move |_| {
                            if !password_verified.get() {
                                set_show_password_dialog.set(true);
                            } else {
                                set_password_verified.set(false);
                            }
                        }
                    >
                        <Icon icon=eye_icon.into() class="w-4 h-4" />
                    </button>
                    {move || {
                        if password_verified.get() {
                            view! {
                                <button
                                    class="ml-2 text-gray-400 hover:text-primary-100"
                                    on:click=move |_| {
                                        let password = decrypted_password.get().clone();
                                        spawn_local(async move {
                                            let args = serde_wasm_bindgen::to_value(&ClipboardArgs {
                                                text: &password
                                            }).unwrap();
                                            if serde_wasm_bindgen::from_value::<()>(invoke("copy_to_clipboard", args).await).is_ok() {
                                                set_is_copied_password.set(true);
                                            }
                                        });
                                    }
                                >
                                    <Icon icon=clipboard_icon.into() class="w-4 h-4" />
                                </button>
                            }.into_view()
                        } else {
                            view! { <div/> }.into_view()
                        }
                    }}
                </div>
            </td>
            <td class="p-4">
                <div class="flex justify-end space-x-2">
                    <button
                        class="text-gray-400 hover:text-primary-100"
                        on:click=move |_| on_edit.call(item.get().id)
                    >
                        <Icon icon=pencil_icon.into() class="w-5 h-5" />
                    </button>
                    <button
                        class="text-gray-400 hover:text-red-500"
                        on:click=move |_| on_delete.call(item.get().id)
                    >
                        <Icon icon=trash_icon.into() class="w-5 h-5" />
                    </button>
                </div>
            </td>

            {move || {
                if show_password_dialog.get() {
                    view! {
                        <PasswordDialog
                        action=DialogAction::Verify
                        on_verify=move |verified| {
                            if verified {
                                let password = item.get().password.clone();
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
                        }
                            on_close=move |_| set_show_password_dialog.set(false)
                        />
                    }.into_view()
                } else {
                    view! { <div/> }.into_view()
                }
            }}
        </tr>
    }
}

#[derive(Serialize)]
struct ClipboardArgs<'a> {
    text: &'a str,
}
