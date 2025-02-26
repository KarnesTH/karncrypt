use crate::{app::invoke, components::icons::Icon};
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

#[component]
pub fn TableItem(
    item: TableItemArgs,
    on_edit: Callback<i32>,
    on_delete: Callback<i32>,
) -> impl IntoView {
    let (show_password_dialog, set_show_password_dialog) = create_signal(false);
    let (is_copied_username, set_is_copied_username) = create_signal(false);

    let eye_icon = create_memo(move |_| "eye");
    let pencil_icon = create_memo(move |_| "pencil-square");
    let trash_icon = create_memo(move |_| "trash");
    let username_icon = create_memo(move |_| {
        if is_copied_username.get() {
            "check"
        } else {
            "clipboard"
        }
    });

    let item = create_memo(move |_| item.clone());

    view! {
        <tr class="hover:bg-background">
            <td class="p-4 text-white whitespace-nowrap">{item.get().service}</td>
            <td class="p-4">
                <div class="flex items-center text-white whitespace-nowrap">
                    {item.get().username}
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
                    <span class="whitespace-nowrap">"••••••••"</span>
                    <button
                        class="ml-2 text-gray-400 hover:text-primary-100"
                        on:click=move |_| set_show_password_dialog.set(true)
                    >
                        <Icon icon=eye_icon.into() class="w-4 h-4" />
                    </button>
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

            {if show_password_dialog.get() {
                view! {
                    <PasswordDialog
                        password=item.get().password
                        on_close=move |_| set_show_password_dialog.set(false)
                    />
                }.into_view()
            } else {
                view! { <div/> }.into_view()
            }}
        </tr>
    }
}

#[derive(Serialize)]
struct ClipboardArgs<'a> {
    text: &'a str,
}
