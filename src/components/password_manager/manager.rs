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

#[derive(Clone, PartialEq)]
enum SortOrder {
    ServiceAsc,
    ServiceDesc,
}

#[component]
pub fn PasswordManager() -> impl IntoView {
    let (passwords, set_passwords) = create_signal(Vec::<TableItemArgs>::new());
    let (is_loading, set_is_loading) = create_signal(true);
    let (show_modal, set_show_modal) = create_signal(false);
    let (modal_mode, set_modal_mode) = create_signal::<ModalMode>(ModalMode::Add);
    let (show_filter_dropdown, set_show_filter_dropdown) = create_signal(false);
    let (sort_order, set_sort_order) = create_signal(SortOrder::ServiceAsc);
    let (search_text, set_search_text) = create_signal(String::new());

    let plus_icon = create_memo(move |_| "plus");
    let key_icon = create_memo(move |_| "key");
    let vault_icon = create_memo(move |_| "key");
    let search_icon = create_memo(move |_| "magnifying-glass");
    let filter_icon = create_memo(move |_| "funnel");
    let refresh_icon = create_memo(move |_| "arrow-path");

    let head_service_icon = create_memo(move |_| "bookmark");
    let head_username_icon = create_memo(move |_| "user");
    let head_password_icon = create_memo(move |_| "key");

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

    let filtered_and_sorted_passwords = create_memo(move |_| {
        let mut results = passwords
            .get()
            .into_iter()
            .filter(|p| {
                let search = search_text.get().to_lowercase();
                if search.is_empty() {
                    return true;
                }
                p.service.to_lowercase().contains(&search)
                    || p.username.to_lowercase().contains(&search)
            })
            .collect::<Vec<_>>();

        match sort_order.get() {
            SortOrder::ServiceAsc => results.sort_by(|a, b| a.service.cmp(&b.service)),
            SortOrder::ServiceDesc => results.sort_by(|a, b| b.service.cmp(&a.service)),
        }

        results
    });

    let handle_refresh = move |_| {
        set_is_loading.set(true);
        spawn_local(async move {
            let response = invoke("get_passwords", wasm_bindgen::JsValue::NULL).await;
            if let Ok(passwords) = serde_wasm_bindgen::from_value::<Vec<TableItemArgs>>(response) {
                set_passwords.set(passwords);
            }

            set_is_loading.set(false);
        });
    };

    view! {
        <div class="w-full flex flex-col h-full">
            <div class="flex justify-between items-center mb-4">
                <h2 class="text-2xl font-bold mb-0 bg-gradient-primary bg-clip-text text-transparent flex items-center">
                    <Icon icon=vault_icon.into() class="w-8 h-8 mr-3 text-primary-100" />
                    "Gespeicherte Passwörter"
                </h2>
                <div class="flex gap-6">
                    <button
                        class="group relative flex items-center text-gray-400 hover:text-white transition-colors"
                        on:click=handle_add
                    >
                        <Icon icon=plus_icon.into() class="w-5 h-5" />
                        <span class="ml-2 whitespace-nowrap max-w-0 overflow-hidden group-hover:max-w-[100px] transition-all duration-300 text-primary-100">
                            "Hinzufügen"
                        </span>
                    </button>
                    <button
                        class="group relative flex items-center text-gray-400 hover:text-white transition-colors"
                        on:click=handle_refresh
                    >
                        <Icon icon=refresh_icon.into() class="w-5 h-5" />
                        <span class="ml-2 whitespace-nowrap max-w-0 overflow-hidden group-hover:max-w-[100px] transition-all duration-300 text-primary-100">
                            "Aktualisieren"
                        </span>
                    </button>
                </div>
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
                        <div class="flex-1 flex flex-col bg-background-card rounded-lg min-h-0">
                            <div class="w-full flex items-center gap-2 bg-background-card rounded-lg p-2">
                                <div class="flex items-center flex-1 relative">
                                    <Icon
                                        icon=search_icon.into()
                                        class="w-5 h-5 text-gray-400 absolute left-3 top-1/2 -translate-y-1/2"
                                    />
                                    <input
                                        type="text"
                                        placeholder="Suchen..."
                                        on:input=move |ev| set_search_text.set(event_target_value(&ev))
                                        class="w-full pl-10 pr-4 py-2 bg-background border border-gray-600 rounded-lg focus:outline-none focus:border-primary-100 text-white"
                                    />
                                </div>
                                <div class="relative">
                                    <button
                                        class="p-2 hover:bg-background rounded-lg text-gray-400 hover:text-white transition-colors"
                                        on:click=move |_| set_show_filter_dropdown.update(|v| *v = !*v)
                                    >
                                        <Icon icon=filter_icon.into() class="w-5 h-5" />
                                    </button>

                                    {move || show_filter_dropdown.get().then(|| view! {
                                        <div class="absolute right-0 top-full mt-2 w-48 bg-background border border-gray-600 rounded-lg shadow-lg p-2 z-50 text-white">
                                            <div class="text-sm text-gray-400 px-3 py-2 font-medium">
                                                "Sortierung"
                                            </div>
                                            <button
                                                class="w-full text-left px-3 py-2 hover:bg-background-light rounded transition-colors flex items-center gap-2"
                                                class:text-primary-100=move || sort_order.get() == SortOrder::ServiceAsc
                                                on:click=move |_| {
                                                    set_sort_order.set(SortOrder::ServiceAsc);
                                                    set_show_filter_dropdown.set(false);
                                                }
                                            >
                                                "Service (A-Z)"
                                            </button>
                                            <button
                                                class="w-full text-left px-3 py-2 hover:bg-background-light rounded transition-colors flex items-center gap-2"
                                                class:text-primary-100=move || sort_order.get() == SortOrder::ServiceDesc
                                                on:click=move |_| {
                                                    set_sort_order.set(SortOrder::ServiceDesc);
                                                    set_show_filter_dropdown.set(false);
                                                }
                                            >
                                                "Service (Z-A)"
                                            </button>
                                        </div>
                                    })}
                                </div>
                            </div>

                            <div class="w-full overflow-auto">
                                <table class="w-full">
                                    <thead class="bg-background sticky top-0">
                                        <tr class="border-b border-gray-600">
                                            <th class="text-left p-4 text-gray-400 font-medium">
                                                <div class="flex items-center">
                                                    <Icon icon=head_service_icon.into() class="w-4 h-4 mr-2" />
                                                    "Service"
                                                </div>
                                            </th>
                                            <th class="text-left p-4 text-gray-400 font-medium">
                                                <div class="flex items-center">
                                                    <Icon icon=head_username_icon.into() class="w-4 h-4 mr-2" />
                                                    "Benutzername"
                                                </div>
                                            </th>
                                            <th class="text-left p-4 text-gray-400 font-medium">
                                                <div class="flex items-center">
                                                    <Icon icon=head_password_icon.into() class="w-4 h-4 mr-2" />
                                                    "Passwort"
                                                </div>
                                            </th>
                                            <th class="text-right p-4 text-gray-400 font-medium w-32">
                                                "Aktionen"
                                            </th>
                                        </tr>
                                    </thead>
                                    <tbody class="divide-y divide-gray-600">
                                        {move || filtered_and_sorted_passwords.get().into_iter().map(|password| {
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
