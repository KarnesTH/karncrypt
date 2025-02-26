use crate::{
    app::invoke,
    components::{
        icons::Icon,
        password_manager::{ModalMode, PasswordModal, TableItem, TableItemArgs},
    },
};
use leptos::*;
use serde::Serialize;

#[derive(Serialize)]
struct AddPasswordArgs {
    service: String,
    username: String,
    password: String,
    url: String,
    notes: String,
}

#[derive(Serialize)]
struct UpdatePasswordArgs {
    id: i32,
    service: String,
    username: String,
    password: String,
    url: String,
    notes: String,
}

#[derive(Serialize)]
struct DeletePasswordArgs {
    id: i32,
}

#[derive(Serialize)]
struct GetPasswordsArgs {}

#[component]
pub fn PasswordManager() -> impl IntoView {
    let (passwords, set_passwords) = create_signal(Vec::<TableItemArgs>::new());
    let (is_loading, set_is_loading) = create_signal(true);
    let (show_modal, set_show_modal) = create_signal(false);
    let (modal_mode, set_modal_mode) = create_signal::<ModalMode>(ModalMode::Add);

    let plus_icon = create_memo(move |_| "plus");
    let key_icon = create_memo(move |_| "key");

    spawn_local(async move {
        let args = serde_wasm_bindgen::to_value(&GetPasswordsArgs {}).unwrap();
        let response = invoke("get_passwords", args).await;
        if let Ok(passwords) = serde_wasm_bindgen::from_value::<Vec<TableItemArgs>>(response) {
            set_passwords.set(passwords);
        }

        set_is_loading.set(false);
    });

    let handle_add = move |_| {
        set_modal_mode.set(ModalMode::Add);
        set_show_modal.set(true);
    };

    let handle_edit = move |id: i32| {
        if let Some(password) = passwords.get().iter().find(|p| p.id == id) {
            set_modal_mode.set(ModalMode::Edit(password.clone()));
            set_show_modal.set(true);
        }
    };

    let handle_save = move |item: TableItemArgs| {
        spawn_local(async move {
            let response = match modal_mode.get() {
                ModalMode::Add => {
                    let args = serde_wasm_bindgen::to_value(&AddPasswordArgs {
                        service: item.service,
                        username: item.username,
                        password: item.password,
                        url: item.url,
                        notes: item.notes,
                    })
                    .unwrap();
                    invoke("add_password", args).await
                }
                ModalMode::Edit(_) => {
                    let args = serde_wasm_bindgen::to_value(&UpdatePasswordArgs {
                        id: item.id,
                        service: item.service,
                        username: item.username,
                        password: item.password,
                        url: item.url,
                        notes: item.notes,
                    })
                    .unwrap();
                    invoke("update_password", args).await
                }
            };

            if serde_wasm_bindgen::from_value::<()>(response).is_ok() {
                let response = invoke("get_passwords", wasm_bindgen::JsValue::NULL).await;
                if let Ok(passwords) =
                    serde_wasm_bindgen::from_value::<Vec<TableItemArgs>>(response)
                {
                    set_passwords.set(passwords);
                }
            }
            set_show_modal.set(false);
        });
    };

    let handle_delete = move |id: i32| {
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&DeletePasswordArgs { id }).unwrap();
            let response = invoke("delete_password", args).await;

            if serde_wasm_bindgen::from_value::<()>(response).is_ok() {
                let response = invoke("get_passwords", wasm_bindgen::JsValue::NULL).await;
                if let Ok(passwords) =
                    serde_wasm_bindgen::from_value::<Vec<TableItemArgs>>(response)
                {
                    set_passwords.set(passwords);
                }
            }
        });
    };

    view! {
        <div class="w-full h-full flex flex-col">
            <div class="flex justify-between items-center mb-4">
                <h2 class="text-xl font-bold text-white">"Gespeicherte Passwörter"</h2>
                <button
                    class="bg-gradient-primary text-white px-4 py-2 rounded flex items-center hover:opacity-90 transition-opacity"
                    on:click=handle_add
                >
                    <Icon icon=plus_icon.into() class="w-5 h-5 mr-2" />
                    "Passwort hinzufügen"
                </button>
            </div>

            {move || {
                if is_loading.get() {
                    view! {
                        <div class="flex justify-center items-center h-64">
                            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-100"></div>
                        </div>
                    }.into_view()
                } else if passwords.get().is_empty() {
                    view! {
                        <div class="flex flex-col items-center justify-center h-64 text-gray-400">
                            <Icon icon=key_icon.into() class="w-16 h-16 mb-4 text-gray-600" />
                            <p class="text-lg mb-4">"Keine Passwörter gespeichert"</p>
                            <button
                                class="bg-gradient-primary text-white px-4 py-2 rounded flex items-center hover:opacity-90 transition-opacity"
                                on:click=handle_add
                            >
                                <Icon icon=plus_icon.into() class="w-5 h-5 mr-2" />
                                "Erstes Passwort hinzufügen"
                            </button>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="flex-1 overflow-hidden flex flex-col bg-background-card rounded-lg">
                            <div class="flex-1 overflow-hidden">
                                <table class="w-full">
                                    <thead class="bg-background">
                                        <tr class="border-b border-gray-600">
                                            <th class="text-left p-4 text-gray-400 font-medium">"Service"</th>
                                            <th class="text-left p-4 text-gray-400 font-medium">"Benutzername"</th>
                                            <th class="text-left p-4 text-gray-400 font-medium">"Passwort"</th>
                                            <th class="text-right p-4 text-gray-400 font-medium w-32">"Aktionen"</th>
                                        </tr>
                                    </thead>
                                    <tbody class="divide-y divide-gray-600">
                                        {move || passwords.get().into_iter().map(|password| {
                                            view! {
                                                <TableItem
                                                    item=password
                                                    on_edit=Callback::from(handle_edit)
                                                    on_delete=Callback::from(handle_delete)
                                                />
                                            }
                                        }).collect_view()}
                                    </tbody>
                                </table>
                            </div>
                        </div>
                    }.into_view()
                }
            }}

            {move || {
                if show_modal.get() {
                    view! {
                        <PasswordModal
                            mode=modal_mode.get()
                            on_save=Callback::from(handle_save)
                            on_close=move |_| set_show_modal.set(false)
                        />
                    }.into_view()
                } else {
                    view! { <div/> }.into_view()
                }
            }}
        </div>
    }
}
