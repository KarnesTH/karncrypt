use crate::{
    app::invoke,
    components::password_manager::{DialogAction, PasswordDialog},
};
use leptos::{ev::SubmitEvent, *};
use serde::{Deserialize, Serialize};

use crate::components::icons::Icon;

#[derive(Clone, Copy, PartialEq)]
enum BackupInterval {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Deserialize)]
struct DatabaseSettingsResponse {
    db_path: String,
    db_name: String,
    auto_backup: bool,
    backup_interval: String,
    max_backups: usize,
    backup_path: String,
    export_path: String,
}

#[derive(Serialize)]
struct DatabaseSettingsArgs<'a> {
    #[serde(rename = "dbPath")]
    db_path: &'a str,
    #[serde(rename = "dbName")]
    db_name: &'a str,
    #[serde(rename = "autoBackup")]
    auto_backup: bool,
    #[serde(rename = "backupInterval")]
    backup_interval: &'a str,
    #[serde(rename = "maxBackups")]
    max_backups: usize,
    #[serde(rename = "backupPath")]
    backup_path: &'a str,
    #[serde(rename = "exportPath")]
    export_path: &'a str,
}

#[component]
pub fn DatabaseSettings() -> impl IntoView {
    let (db_path, set_db_path) = create_signal(String::new());
    let (db_name, set_db_name) = create_signal(String::new());
    let (auto_backup, set_auto_backup) = create_signal(false);
    let (backup_interval, set_backup_interval) = create_signal(BackupInterval::Weekly);
    let (export_path, set_export_path) = create_signal(String::new());
    let (max_backup, set_max_backup) = create_signal(0);
    let (backup_path, set_backup_path) = create_signal(String::new());
    let (error, set_error) = create_signal(String::new());
    let (im_export_status, set_im_export_status) = create_signal(String::new());
    let (show_password_dialog, set_show_password_dialog) = create_signal(false);
    let (current_action, set_current_action) = create_signal(DialogAction::Verify);

    let folder_icon = create_memo(move |_| "folder-open");
    let backup_icon = create_memo(move |_| "archive-box");
    let restore_icon = create_memo(move |_| "archive-box-arrow-down");
    let chevron_down_icon = create_memo(move |_| "chevron-down");
    let database_icon = create_memo(move |_| "circle-stack");
    let import_icon = create_memo(move |_| "arrow-up-tray");
    let export_icon = create_memo(move |_| "arrow-down-tray");

    spawn_local(async move {
        let response = invoke("get_database_settings", wasm_bindgen::JsValue::NULL).await;
        if let Ok(settings) = serde_wasm_bindgen::from_value::<DatabaseSettingsResponse>(response) {
            set_db_name.set(settings.db_name);
            set_db_path.set(settings.db_path);
            set_auto_backup.set(settings.auto_backup);
            match settings.backup_interval.as_str() {
                "daily" => set_backup_interval.set(BackupInterval::Daily),
                "weekly" => set_backup_interval.set(BackupInterval::Weekly),
                "monthly" => set_backup_interval.set(BackupInterval::Monthly),
                "yearly" => set_backup_interval.set(BackupInterval::Yearly),
                _ => (),
            }
            set_export_path.set(settings.export_path);
            set_max_backup.set(settings.max_backups);
            set_backup_path.set(settings.backup_path);
        }
    });

    let handle_export = move |_| {
        set_current_action.set(DialogAction::ExportPasswords);
        set_show_password_dialog.set(true);
    };

    let handle_backup = move |_| {
        set_current_action.set(DialogAction::CreateBackup);
        set_show_password_dialog.set(true);
    };

    let handle_restore_backup = move |_| {
        set_current_action.set(DialogAction::RestoreBackup);
        set_show_password_dialog.set(true);
    };

    let handle_dialog_close = move |_| {
        set_show_password_dialog.set(false);
    };

    let handle_select_database_folder = move |_| {
        spawn_local(async move {
            let response = invoke("select_folder", wasm_bindgen::JsValue::NULL).await;
            if let Ok(path) = serde_wasm_bindgen::from_value::<String>(response) {
                set_db_path.set(path);
            }
        });
    };

    let handle_select_backup_folder = move |_| {
        spawn_local(async move {
            let response = invoke("select_folder", wasm_bindgen::JsValue::NULL).await;
            if let Ok(path) = serde_wasm_bindgen::from_value::<String>(response) {
                set_backup_path.set(path);
            }
        });
    };

    let handle_save_settings = move |ev: SubmitEvent| {
        ev.prevent_default();

        let args = serde_wasm_bindgen::to_value(&DatabaseSettingsArgs {
            db_path: &db_path.get().as_str(),
            db_name: &db_name.get(),
            auto_backup: auto_backup.get(),
            backup_interval: match backup_interval.get() {
                BackupInterval::Daily => "Täglich",
                BackupInterval::Weekly => "Wöchentlich",
                BackupInterval::Monthly => "Monatlich",
                BackupInterval::Yearly => "Jährlich",
            },
            max_backups: max_backup.get(),
            backup_path: &backup_path.get(),
            export_path: &export_path.get(),
        })
        .unwrap();
        spawn_local(async move {
            let response = invoke("save_database_settings", args).await;
            match serde_wasm_bindgen::from_value::<()>(response) {
                Ok(_) => {
                    set_error.set("Einstellungen gespeichert".to_string());
                }
                Err(_) => {
                    set_error.set("Fehler beim Speichern der Einstellungen".to_string());
                }
            }
        });
    };

    view! {
        <div class="flex justify-center">
            <div class="max-w-xl w-full space-y-8">
                <form class="space-y-8" on:submit=handle_save_settings>
                    {move || (!error.get().is_empty()).then(||
                        view! {
                            <div class="text-primary-100 text-sm text-center">
                                {error.get()}
                            </div>
                        }
                    )}

                    <fieldset class="space-y-4">
                        <h3 class="text-lg font-semibold bg-gradient-primary bg-clip-text text-transparent mb-4">
                            <div class="flex items-center space-x-2">
                                <Icon icon=database_icon.into() class="w-5 h-5 text-primary-100" />
                                <span>"Datenbank-Einstellungen"</span>
                            </div>
                        </h3>

                        <div>
                            <label class="block text-white text-sm font-bold mb-2">
                                "Datenbankname"
                            </label>
                            <input
                                type="text"
                                class="w-full shadow appearance-none border border-gray-600 rounded py-2 px-3 bg-background text-white leading-tight focus:outline-none focus:border-primary-100"
                                placeholder="Name der Datenbank"
                                on:input=move |ev| set_db_name.set(event_target_value(&ev))
                                prop:value=db_name
                            />
                            <p class="mt-1 text-sm text-gray-400">
                                "Der Name der Datenbank, in der die Passwörter gespeichert werden"
                            </p>
                        </div>

                        <div>
                            <label class="block text-white text-sm font-bold mb-2">
                                "Datenbank-Pfad"
                            </label>
                            <div class="flex space-x-2">
                                <input
                                    type="text"
                                    class="flex-1 shadow appearance-none border border-gray-600 rounded py-2 px-3 bg-background text-white leading-tight focus:outline-none focus:border-primary-100"
                                    placeholder="Pfad zur Datenbank"
                                    on:input=move |ev| set_db_path.set(event_target_value(&ev))
                                    prop:value=db_path
                                />
                                <button
                                    type="button"
                                    class="px-4 py-2 bg-background border border-primary-100 hover:bg-primary-400/10 text-white rounded focus:outline-none transition-all duration-200"
                                    on:click=handle_select_database_folder
                                >
                                    <Icon icon=folder_icon.into() class="w-5 h-5 text-primary-100" />
                                </button>
                            </div>
                            <p class="mt-1 text-sm text-gray-400">
                                "Der Pfad, in dem die Datenbank gespeichert wird"
                            </p>
                        </div>
                    </fieldset>

                    <fieldset class="space-y-4">
                        <h3 class="text-lg font-semibold bg-gradient-primary bg-clip-text text-transparent mb-4">
                            <div class="flex items-center space-x-2">
                                <Icon icon=backup_icon.into() class="w-5 h-5 text-primary-100" />
                                <span>"Backup-Einstellungen"</span>
                            </div>
                        </h3>

                        <div>
                            <label class="block text-white text-sm font-bold mb-2">
                                "Backup-Pfad"
                            </label>
                            <div class="flex space-x-2">
                                <input
                                    type="text"
                                    class="flex-1 shadow appearance-none border border-gray-600 rounded py-2 px-3 bg-background text-white leading-tight focus:outline-none focus:border-primary-100"
                                    placeholder="Pfad zum Backup"
                                    on:input=move |ev| set_backup_path.set(event_target_value(&ev))
                                    prop:value=backup_path
                                />
                                <button
                                    type="button"
                                    class="px-4 py-2 bg-background border border-primary-100 hover:bg-primary-400/10 text-white rounded focus:outline-none transition-all duration-200"
                                    on:click=handle_select_backup_folder
                                >
                                    <Icon icon=folder_icon.into() class="w-5 h-5 text-primary-100" />
                                </button>
                            </div>
                            <p class="mt-1 text-sm text-gray-400">
                                "Der Pfad, in dem die Backups gespeichert werden"
                            </p>
                        </div>

                        <div class="flex items-center space-x-3">
                            <input
                                type="checkbox"
                                id="auto-backup"
                                class="w-4 h-4 bg-background border-gray-600 rounded focus:ring-primary-100"
                                on:change=move |ev| set_auto_backup.set(event_target_checked(&ev))
                                prop:checked=auto_backup
                            />
                            <label for="auto-backup" class="text-white text-sm font-bold">
                                "Automatische Backups aktivieren"
                            </label>
                        </div>
                        <p class="text-sm text-gray-400">
                            "Aktiviere automatische Backups, um deine Daten regelmäßig zu sichern"
                        </p>

                        {move || auto_backup.get().then(||
                            view! {
                                <div class="grid grid-cols-2 gap-6">
                                    <div>
                                        <label class="block text-white text-sm font-bold mb-2">
                                            "Backup-Intervall"
                                        </label>
                                        <div class="relative">
                                            <select
                                                class="w-full p-2 pr-8 rounded bg-background text-white border border-gray-600 focus:border-primary-100 focus:outline-none
                                                    appearance-none cursor-pointer
                                                    [&>option]:bg-background
                                                    [&>option]:text-white
                                                    [&>option]:border-gray-600
                                                    [&>option]:hover:bg-primary-400/10"
                                                on:change=move |ev| {
                                                    match event_target_value(&ev).as_str() {
                                                        "daily" => set_backup_interval.set(BackupInterval::Daily),
                                                        "weekly" => set_backup_interval.set(BackupInterval::Weekly),
                                                        "monthly" => set_backup_interval.set(BackupInterval::Monthly),
                                                        "yearly" => set_backup_interval.set(BackupInterval::Yearly),
                                                        _ => ()
                                                    }
                                                }
                                            >
                                                <option
                                                    value="daily"
                                                    selected=move || backup_interval.get() == BackupInterval::Daily
                                                >
                                                    "Täglich"
                                                </option>
                                                <option
                                                    value="weekly"
                                                    selected=move || backup_interval.get() == BackupInterval::Weekly
                                                >
                                                    "Wöchentlich"
                                                </option>
                                                <option
                                                    value="monthly"
                                                    selected=move || backup_interval.get() == BackupInterval::Monthly
                                                >
                                                    "Monatlich"
                                                </option>
                                                <option
                                                    value="yearly"
                                                    selected=move || backup_interval.get() == BackupInterval::Yearly
                                                >
                                                    "Jährlich"
                                                </option>
                                            </select>
                                            <div class="absolute inset-y-0 right-0 flex items-center px-2 pointer-events-none text-gray-400">
                                                <Icon icon=chevron_down_icon.into() class="w-5 h-5" />
                                            </div>
                                        </div>
                                        <p class="mt-1 text-sm text-gray-400">
                                            "Wie häufig sollen automatische Backups erstellt werden"
                                        </p>
                                    </div>

                                    <div>
                                        <label class="block text-white text-sm font-bold mb-2">
                                            "Maximale Anzahl Backups"
                                        </label>
                                        <input
                                            type="number"
                                            min="1"
                                            max="100"
                                            class="w-full shadow appearance-none border border-gray-600 rounded py-2 px-3 bg-background text-white leading-tight focus:outline-none focus:border-primary-100"
                                            on:input=move |ev| {
                                                if let Ok(value) = event_target_value(&ev).parse::<usize>() {
                                                    set_max_backup.set(value);
                                                }
                                            }
                                            prop:value=move || max_backup.get().to_string()
                                        />
                                        <p class="mt-1 text-sm text-gray-400">
                                            "Maximale Anzahl der Backups, die aufbewahrt werden sollen"
                                        </p>
                                    </div>
                                </div>
                            }
                        )}

                        <div class="grid grid-cols-2 gap-4 pt-2">
                            <button
                                type="button"
                                class="flex items-center justify-center space-x-2 bg-background border border-primary-100 hover:bg-primary-400/10 text-white py-2 px-4 rounded focus:outline-none transition-all duration-200"
                                on:click=handle_backup
                            >
                                <Icon icon=backup_icon.into() class="w-5 h-5 text-primary-100" />
                                <span>"Backup erstellen"</span>
                            </button>
                            <button
                                type="button"
                                class="flex items-center justify-center space-x-2 bg-background border border-primary-100 hover:bg-primary-400/10 text-white py-2 px-4 rounded focus:outline-none transition-all duration-200"
                                on:click=handle_restore_backup
                            >
                                <Icon icon=restore_icon.into() class="w-5 h-5 text-primary-100" />
                                <span>"Backup wiederherstellen"</span>
                            </button>
                        </div>
                    </fieldset>

                    <fieldset class="space-y-4">
                        <h3 class="text-lg font-semibold bg-gradient-primary bg-clip-text text-transparent mb-4">
                            <div class="flex items-center space-x-2">
                                <Icon icon=database_icon.into() class="w-5 h-5 text-primary-100" />
                                <span>"Import/Export"</span>
                            </div>
                        </h3>

                        <div>
                            <label class="block text-white text-sm font-bold mb-2">
                                "Export-Pfad"
                            </label>
                            <div class="flex space-x-2">
                                <input
                                    type="text"
                                    class="flex-1 shadow appearance-none border border-gray-600 rounded py-2 px-3 bg-background text-white leading-tight focus:outline-none focus:border-primary-100"
                                    placeholder="Pfad zum Export"
                                    on:input=move |ev| set_export_path.set(event_target_value(&ev))
                                    prop:value=export_path
                                />
                                <button
                                    type="button"
                                    class="px-4 py-2 bg-background border border-primary-100 hover:bg-primary-400/10 text-white rounded focus:outline-none transition-all duration-200"
                                >
                                    <Icon icon=folder_icon.into() class="w-5 h-5 text-primary-100" />
                                </button>
                            </div>
                            <p class="mt-1 text-sm text-gray-400">
                                "Der Pfad, in dem die Datenbank als csv-Datei exportiert wird"
                            </p>
                        </div>

                        <div class="grid grid-cols-2 gap-4">
                            <button
                                type="button"
                                class="flex items-center justify-center space-x-2 bg-background border border-primary-100 hover:bg-primary-400/10 text-white py-2 px-4 rounded focus:outline-none transition-all duration-200"
                                on:click=move |_| {
                                    spawn_local(async move {
                                        let response = invoke("import_passwords", wasm_bindgen::JsValue::NULL).await;
                                        match serde_wasm_bindgen::from_value(response) {
                                            Ok(()) => {
                                                set_im_export_status.set("Import erfolgreich!".to_string());
                                            }
                                            Err(_) => {
                                                set_im_export_status.set("Import fehlgeschlagen!".to_string());
                                            }
                                        }
                                    });
                                }
                            >
                                <Icon icon=import_icon.into() class="w-5 h-5 text-primary-100" />
                                <span>"CSV importieren"</span>
                            </button>
                            <button
                                type="button"
                                class="flex items-center justify-center space-x-2 bg-background border border-primary-100 hover:bg-primary-400/10 text-white py-2 px-4 rounded focus:outline-none transition-all duration-200"
                                on:click=handle_export
                            >
                                <Icon icon=export_icon.into() class="w-5 h-5 text-primary-100" />
                                <span>"Als CSV exportieren"</span>
                            </button>
                        </div>
                        {move || (!im_export_status.get().is_empty()).then(||
                            view! {
                                <div class="text-primary-100 text-sm text-center">
                                    {im_export_status.get()}
                                </div>
                            }
                        )}
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
