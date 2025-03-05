use crate::{
    app::invoke,
    components::{
        auth::Register,
        icons::Icon,
        password_manager::{DialogAction, PasswordDialog},
    },
};
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct CompleteSetupArgs<'a> {
    #[serde(rename = "dbPath")]
    db_path: &'a str,
    #[serde(rename = "dbName")]
    db_name: &'a str,
    #[serde(rename = "backupPath")]
    backup_path: &'a str,
}

#[derive(Clone, Copy, PartialEq)]
enum InitStep {
    Welcome,
    Restore,
    Database,
    MasterPassword,
}

#[derive(Deserialize)]
struct Config {
    db_name: String,
    db_path: String,
    backup_path: String,
}

#[component]
pub fn Init(#[prop(into)] on_complete: Callback<()>) -> impl IntoView {
    let (current_step, set_current_step) = create_signal(InitStep::Welcome);
    let (db_path, set_db_path) = create_signal(String::new());
    let (db_name, set_db_name) = create_signal(String::new());
    let (backup_path, set_backup_path) = create_signal(String::new());
    let (restore_flow, set_restore_flow) = create_signal(false);
    let (show_password_dialog, set_show_password_dialog) = create_signal(false);
    let (current_action, set_current_action) = create_signal(DialogAction::Verify);

    let flag_icon = create_memo(move |_| "flag");
    let database_icon = create_memo(move |_| "circle-stack");
    let key_icon = create_memo(move |_| "key");
    let folder_icon = create_memo(move |_| "folder-open");
    let arrow_right_icon = create_memo(move |_| "arrow-right");
    let backup_icon = create_memo(move |_| "archive-box");
    let arrow_left_icon = create_memo(move |_| "arrow-left");

    spawn_local(async move {
        let response = invoke("get_default_config", wasm_bindgen::JsValue::NULL).await;
        if let Ok(config) = serde_wasm_bindgen::from_value::<Config>(response) {
            set_db_name.set(config.db_name);
            set_db_path.set(config.db_path);
            set_backup_path.set(config.backup_path);
        }
    });

    let handle_select_db_folder = move || {
        let path = db_path.get();
        spawn_local(async move {
            let response = invoke("select_folder", wasm_bindgen::JsValue::NULL).await;
            if let Ok(path) = serde_wasm_bindgen::from_value::<String>(response) {
                set_db_path.set(path);
            } else {
                set_db_path.set(path);
            }
        });
    };

    let handle_select_backup_folder = move || {
        let path = backup_path.get();
        spawn_local(async move {
            let response = invoke("select_folder", wasm_bindgen::JsValue::NULL).await;
            if let Ok(path) = serde_wasm_bindgen::from_value::<String>(response) {
                set_backup_path.set(path);
            } else {
                set_backup_path.set(path);
            }
        });
    };

    let handle_dialog_close = move |_| {
        set_show_password_dialog.set(false);
    };

    view! {
        <div class="min-h-screen flex items-center justify-center bg-background">
            <div class="w-full h-screen py-8 px-8 flex flex-col">
                <div class="mb-8 flex-shrink-0">
                    {move || {
                        if !restore_flow.get() {
                            view! {
                                <div class="grid grid-flow-col auto-col-max">
                                    <div class="flex flex-col items-center">
                                        <div class=move || format!("rounded-full h-12 w-12 flex items-center justify-center border-2 {}",
                                            match current_step.get() {
                                                InitStep::Welcome => "border-primary-100 text-primary-100",
                                                _ => "border-gray-400 text-gray-400"
                                            }
                                        )>
                                            <Icon icon=flag_icon.into() class="w-5 h-5" />
                                        </div>
                                        <div class="text-xs mt-1 text-white">"Willkommen"</div>
                                    </div>
                                    <div class="flex flex-col items-center">
                                        <div class=move || format!("rounded-full h-12 w-12 flex items-center justify-center border-2 {}",
                                            match current_step.get() {
                                                InitStep::Database => "border-primary-100 text-primary-100",
                                                _ => "border-gray-400 text-gray-400"
                                            }
                                        )>
                                            <Icon icon=database_icon.into() class="w-5 h-5" />
                                        </div>
                                        <div class="text-xs mt-1 text-white">"Datenbank"</div>
                                    </div>
                                    <div class="flex flex-col items-center">
                                        <div class=move || format!("rounded-full h-12 w-12 flex items-center justify-center border-2 {}",
                                            match current_step.get() {
                                                InitStep::MasterPassword => "border-primary-100 text-primary-100",
                                                _ => "border-gray-400 text-gray-400"
                                            }
                                        )>
                                            <Icon icon=key_icon.into() class="w-5 h-5" />
                                        </div>
                                        <div class="text-xs mt-1 text-white">"Master-Passwort"</div>
                                    </div>
                                </div>
                            }.into_view()
                        } else {
                            view! {
                                <div class="grid grid-flow-col auto-col-max">
                                    <div class="flex flex-col items-center">
                                        <div class=move || format!("rounded-full h-12 w-12 flex items-center justify-center border-2 {}",
                                            match current_step.get() {
                                                InitStep::Welcome => "border-primary-100 text-primary-100",
                                                _ => "border-gray-400 text-gray-400"
                                            }
                                        )>
                                            <Icon icon=flag_icon.into() class="w-5 h-5" />
                                        </div>
                                        <div class="text-xs mt-1 text-white">"Willkommen"</div>
                                    </div>
                                    <div class="flex flex-col items-center">
                                        <div class=move || format!("rounded-full h-12 w-12 flex items-center justify-center border-2 {}",
                                            match current_step.get() {
                                                InitStep::Restore => "border-primary-100 text-primary-100",
                                                _ => "border-gray-400 text-gray-400"
                                            }
                                        )>
                                            <Icon icon=backup_icon.into() class="w-5 h-5" />
                                        </div>
                                        <div class="text-xs mt-1 text-white">"Wiederherstellen"</div>
                                    </div>
                                </div>
                            }.into_view()
                        }
                    }}
                </div>

                <div class="flex-1 overflow-y-auto">
                    {move || match current_step.get() {
                        InitStep::Welcome => view! {
                            <div class="flex gap-6">
                                <div class="flex-1 bg-background-card rounded-lg p-6 shadow-lg">
                                    <h2 class="text-2xl font-bold mb-4 bg-gradient-primary bg-clip-text text-transparent flex items-center">
                                        <Icon icon=flag_icon.into() class="w-8 h-8 mr-3 text-primary-100" />
                                        "Willkommen bei KarnCrypt"
                                    </h2>
                                    <p class="text-gray-300 text-lg mb-6">
                                        "Dein sicherer Passwort-Manager"
                                    </p>
                                    <div class="space-y-4">
                                        <div class="p-4 border border-gray-700 rounded-lg">
                                            <h3 class="font-semibold text-white mb-2 flex items-center">
                                                <Icon icon=key_icon.into() class="w-5 h-5 mr-2 text-primary-100" />
                                                "Sichere Verschlüsselung"
                                            </h3>
                                            <p class="text-gray-300 text-sm">
                                                "Deine Passwörter werden lokal und verschlüsselt gespeichert."
                                            </p>
                                        </div>
                                        <div class="p-4 border border-gray-700 rounded-lg">
                                            <h3 class="font-semibold text-white mb-2 flex items-center">
                                                <Icon icon=backup_icon.into() class="w-5 h-5 mr-2 text-primary-100" />
                                                "Backup & Restore"
                                            </h3>
                                            <p class="text-gray-300 text-sm">
                                                "Sichere deine Daten mit automatischen Backups."
                                            </p>
                                        </div>
                                    </div>
                                </div>

                                <div class="flex-1 space-y-4">
                                    <div class="bg-background-card rounded-lg p-6 shadow-lg">
                                        <h3 class="font-semibold text-white mb-2">"Neue Installation"</h3>
                                        <p class="text-gray-300 text-sm mb-4">
                                            "Richte deinen neuen Passwort-Manager ein:"
                                        </p>
                                        <ul class="list-disc list-inside text-gray-300 mb-4 space-y-2 text-sm">
                                            <li>"Erstelle eine verschlüsselte Datenbank"</li>
                                            <li>"Wähle ein sicheres Master-Passwort"</li>
                                            <li class="text-primary-100 font-semibold">
                                                "WICHTIG: Dein Master-Passwort kann nicht wiederhergestellt werden!"
                                            </li>
                                        </ul>
                                        <button
                                            class="w-full bg-gradient-primary text-white font-bold py-2 px-4 rounded transition-all hover:opacity-90 flex items-center justify-center"
                                            on:click=move |_| set_current_step.set(InitStep::Database)
                                        >
                                            <span>"Installation starten"</span>
                                            <Icon icon=arrow_right_icon.into() class="w-5 h-5 ml-2" />
                                        </button>
                                    </div>

                                    <div class="bg-background-card rounded-lg p-6 shadow-lg">
                                        <h3 class="font-semibold text-white mb-2">"Backup wiederherstellen"</h3>
                                        <p class="text-gray-300 text-sm mb-4">
                                            "Stelle deine bestehende KarnCrypt Installation wieder her:"
                                        </p>
                                        <ul class="list-disc list-inside text-gray-300 mb-4 space-y-2 text-sm">
                                            <li>"Importiere dein Backup (.pmbackup)"</li>
                                            <li>"Übernehme alle deine Daten und Einstellungen"</li>
                                        </ul>
                                        <button
                                            class="w-full border-2 border-primary-100 text-primary-100 font-bold py-2 px-4 rounded transition-all hover:bg-primary-100/10 flex items-center justify-center"
                                            on:click=move |_|{
                                                set_restore_flow.set(true);
                                                set_current_step.set(InitStep::Restore)
                                            }
                                        >
                                            <span>"Backup wiederherstellen"</span>
                                            <Icon icon=arrow_right_icon.into() class="w-5 h-5 ml-2" />
                                        </button>
                                    </div>
                                </div>
                            </div>
                        }.into_view(),

                        InitStep::Restore => view! {
                            <div class="flex gap-6">
                                <div class="flex-1 bg-background-card rounded-lg p-6 shadow-lg">
                                    <h2 class="text-2xl font-bold mb-4 bg-gradient-primary bg-clip-text text-transparent flex items-center">
                                        <Icon icon=backup_icon.into() class="w-8 h-8 mr-3 text-primary-100" />
                                        "Backup wiederherstellen"
                                    </h2>
                                    <p class="text-gray-300 text-lg mb-6">
                                        "Stelle deine vorherige Installation wieder her"
                                    </p>
                                    <div class="space-y-4">
                                        <div class="p-4 border border-gray-700 rounded-lg">
                                            <h3 class="font-semibold text-white mb-2 flex items-center">
                                                <Icon icon=key_icon.into() class="w-5 h-5 mr-2 text-primary-100" />
                                                "Datenbank & Einstellungen"
                                            </h3>
                                            <p class="text-gray-300 text-sm">
                                                "Deine Datenbank und alle Einstellungen werden aus dem Backup wiederhergestellt."
                                            </p>
                                        </div>
                                        <div class="p-4 border border-gray-700 rounded-lg">
                                            <h3 class="font-semibold text-white mb-2 flex items-center">
                                                <Icon icon=folder_icon.into() class="w-5 h-5 mr-2 text-primary-100" />
                                                ".pmbackup Format"
                                            </h3>
                                            <p class="text-gray-300 text-sm">
                                                "Wähle eine vorhandene .pmbackup Datei aus, die alle deine Daten enthält."
                                            </p>
                                        </div>
                                    </div>
                                </div>

                                <div class="flex-1 bg-background-card rounded-lg p-6 shadow-lg">
                                    <button
                                        class="mb-6 text-sm flex items-center text-primary-100"
                                        on:click=move |_| {
                                            set_restore_flow.set(false);
                                            set_current_step.set(InitStep::Welcome);
                                        }
                                    >
                                        <Icon icon=arrow_left_icon.into() class="w-4 h-4 mr-2 text-primary-100" />
                                        "Zurück"
                                    </button>

                                    <div class="flex w-full justify-center mb-6">
                                        <Icon icon=backup_icon.into() class="w-16 h-16 text-gray-600" />
                                    </div>

                                    <button
                                        class="w-full bg-gradient-primary text-white font-bold py-2 px-4 rounded transition-all hover:opacity-90 flex items-center justify-center"
                                        disabled=move || backup_path.get().is_empty()
                                        on:click=move |_| {
                                            set_current_action.set(DialogAction::RestoreBackup);
                                            set_show_password_dialog.set(true);
                                        }
                                    >
                                        <span>"Backup wiederherstellen"</span>
                                        <Icon icon=arrow_right_icon.into() class="w-5 h-5 ml-2" />
                                    </button>
                                </div>
                            </div>
                        }.into_view(),

                        InitStep::Database => view! {
                            <div class="flex gap-6">
                                <div class="flex-1 bg-background-card rounded-lg p-6 shadow-lg">
                                    <h2 class="text-2xl font-bold mb-4 bg-gradient-primary bg-clip-text text-transparent flex items-center">
                                        <Icon icon=database_icon.into() class="w-8 h-8 mr-3 text-primary-100" />
                                        "Datenbank Setup"
                                    </h2>
                                    <p class="text-gray-300 text-lg mb-6">
                                        "Konfiguriere deine Datenbank"
                                    </p>
                                    <div class="space-y-4">
                                        <div class="p-4 border border-gray-700 rounded-lg">
                                            <h3 class="font-semibold text-white mb-2 flex items-center">
                                                <Icon icon=database_icon.into() class="w-5 h-5 mr-2 text-primary-100" />
                                                "Lokale Speicherung"
                                            </h3>
                                            <p class="text-gray-300 text-sm">
                                                "Deine Datenbank wird lokal auf deinem Gerät gespeichert und ist nur für dich zugänglich."
                                            </p>
                                        </div>
                                        <div class="p-4 border border-gray-700 rounded-lg">
                                            <h3 class="font-semibold text-white mb-2 flex items-center">
                                                <Icon icon=backup_icon.into() class="w-5 h-5 mr-2 text-primary-100" />
                                                "Automatische Backups"
                                            </h3>
                                            <p class="text-gray-300 text-sm">
                                                "Lege fest, wo deine Backup-Dateien gespeichert werden sollen."
                                            </p>
                                        </div>
                                    </div>
                                </div>

                                <div class="flex-1 bg-background-card rounded-lg p-6 shadow-lg">
                                    <button
                                        class="mb-6 text-sm flex items-center text-primary-100"
                                        on:click=move |_| {
                                            set_current_step.set(InitStep::Welcome);
                                        }
                                    >
                                        <Icon icon=arrow_left_icon.into() class="w-4 h-4 mr-2 text-primary-100" />
                                        "Zurück"
                                    </button>

                                    <div class="mb-4">
                                        <label class="block text-white text-sm font-bold mb-2">
                                            "Datenbankname"
                                        </label>
                                        <input
                                            type="text"
                                            placeholder="Name der Datenbank"
                                            class="w-full p-2 rounded bg-background text-white border border-gray-600 focus:border-primary-100 focus:outline-none"
                                            on:input=move |ev| set_db_name.set(event_target_value(&ev))
                                            prop:value=db_name
                                        />
                                        <p class="text-sm text-primary-100 mt-1">
                                            "Standard: pass.db"
                                        </p>
                                    </div>

                                    <div class="mb-4">
                                        <label class="block text-white text-sm font-bold mb-2">
                                            "Datenbank-Pfad (Optional)"
                                        </label>
                                        <div class="flex space-x-2">
                                            <input
                                                type="text"
                                                placeholder="Pfad zur Datenbank"
                                                class="flex-1 p-2 rounded bg-background text-white border border-gray-600 focus:border-primary-100 focus:outline-none"
                                                on:input=move |ev| set_db_path.set(event_target_value(&ev))
                                                prop:value=db_path
                                            />
                                            <button
                                                type="button"
                                                class="px-4 py-2 bg-background border border-primary-100 hover:bg-primary-400/10 text-white rounded focus:outline-none transition-all duration-200"
                                                on:click=move |_| handle_select_db_folder()
                                            >
                                                <Icon icon=folder_icon.into() class="w-5 h-5 text-primary-100" />
                                            </button>
                                        </div>
                                        <p class="text-sm text-primary-100 mt-1">
                                            "Standard: Konfigurationsverzeichnis"
                                        </p>
                                    </div>

                                    <div class="mb-6">
                                        <label class="block text-white text-sm font-bold mb-2">
                                            "Backup-Pfad (Optional)"
                                        </label>
                                        <div class="flex space-x-2">
                                            <input
                                                type="text"
                                                placeholder="Pfad für Backups"
                                                class="flex-1 p-2 rounded bg-background text-white border border-gray-600 focus:border-primary-100 focus:outline-none"
                                                on:input=move |ev| set_backup_path.set(event_target_value(&ev))
                                                prop:value=backup_path
                                            />
                                            <button
                                                type="button"
                                                class="px-4 py-2 bg-background border border-primary-100 hover:bg-primary-400/10 text-white rounded focus:outline-none transition-all duration-200"
                                                on:click=move |_| handle_select_backup_folder()
                                            >
                                                <Icon icon=folder_icon.into() class="w-5 h-5 text-primary-100" />
                                            </button>
                                        </div>
                                        <p class="text-sm text-primary-100 mt-1">
                                            "Standard: Konfigurationsverzeichnis/backups"
                                        </p>
                                    </div>

                                    <button
                                        class="w-full bg-gradient-primary text-white font-bold py-2 px-4 rounded transition-all hover:opacity-90 flex items-center justify-center"
                                        on:click=move |_| set_current_step.set(InitStep::MasterPassword)
                                    >
                                        <span>"Weiter"</span>
                                        <Icon icon=arrow_right_icon.into() class="w-5 h-5 ml-2" />
                                    </button>
                                </div>
                            </div>
                        }.into_view(),

                        InitStep::MasterPassword => view! {
                            <div class="flex gap-6">
                                <div class="flex-1 bg-background-card rounded-lg p-6 shadow-lg">
                                    <h2 class="text-2xl font-bold mb-4 bg-gradient-primary bg-clip-text text-transparent flex items-center">
                                        <Icon icon=key_icon.into() class="w-8 h-8 mr-3 text-primary-100" />
                                        "Master-Passwort erstellen"
                                    </h2>
                                    <p class="text-gray-300 text-lg mb-6">
                                        "Dein Schlüssel zu allen Passwörtern"
                                    </p>
                                    <div class="space-y-4">
                                        <div class="p-4 border border-gray-700 rounded-lg">
                                            <h3 class="font-semibold text-white mb-2 flex items-center">
                                                <Icon icon=key_icon.into() class="w-5 h-5 mr-2 text-primary-100" />
                                                "Sicherheit ist wichtig"
                                            </h3>
                                            <p class="text-gray-300 text-sm">
                                                "Wähle ein starkes Passwort, das du dir gut merken kannst. Es ist der einzige Weg, auf deine Daten zuzugreifen."
                                            </p>
                                        </div>
                                        <div class="p-4 border border-gray-700 rounded-lg text-primary-100">
                                            <h3 class="font-semibold mb-2 flex items-center">
                                                <Icon icon=flag_icon.into() class="w-5 h-5 mr-2" />
                                                "Wichtiger Hinweis"
                                            </h3>
                                            <p class="text-sm">
                                                "Das Master-Passwort kann nicht wiederhergestellt werden. Bewahre es sicher auf!"
                                            </p>
                                        </div>
                                    </div>
                                </div>

                                <div class="flex-1 bg-background-card rounded-lg p-6 shadow-lg">
                                    <button
                                        class="mb-6 text-sm flex items-center text-primary-100"
                                        on:click=move |_| {
                                            set_current_step.set(InitStep::Database);
                                        }
                                    >
                                        <Icon icon=arrow_left_icon.into() class="w-4 h-4 mr-2 text-primary-100" />
                                        "Zurück"
                                    </button>

                                    <Register
                                        on_success=move |_| {
                                            let args = serde_wasm_bindgen::to_value(&CompleteSetupArgs {
                                                db_path: db_path.get().as_str(),
                                                db_name: db_name.get().as_str(),
                                                backup_path: backup_path.get().as_str(),
                                            }).unwrap();

                                            spawn_local(async move {
                                                let response = invoke("complete_setup", args).await;
                                                if serde_wasm_bindgen::from_value::<()>(response).is_ok() {
                                                    on_complete.call(());
                                                }
                                            });
                                        }
                                    />
                                </div>
                            </div>
                        }.into_view(),
                    }}
                </div>
            </div>
            {move || show_password_dialog.get().then(|| view! {
                <PasswordDialog
                    action=current_action.get()
                    on_close=handle_dialog_close
                    on_verify=move |_| ()
                />
            })}
        </div>
    }
}
