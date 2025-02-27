use leptos::*;

use crate::components::icons::Icon;

#[derive(Clone, Copy, PartialEq)]
enum BackupInterval {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[component]
pub fn DatabaseSettings() -> impl IntoView {
    let (db_path, _set_db_path) = create_signal(String::new());
    let (db_name, set_db_name) = create_signal(String::new());
    let (auto_backup, set_auto_backup) = create_signal(false);
    let (backup_interval, set_backup_interval) = create_signal(BackupInterval::Weekly);
    let (error, _set_error) = create_signal(String::new());

    let folder_icon = create_memo(move |_| "folder-open");
    let backup_icon = create_memo(move |_| "archive-box");
    let restore_icon = create_memo(move |_| "archive-box-arrow-down");
    let chevron_down_icon = create_memo(move |_| "chevron-down");
    let database_icon = create_memo(move |_| "circle-stack");
    let import_icon = create_memo(move |_| "arrow-up-tray");
    let export_icon = create_memo(move |_| "arrow-down-tray");

    view! {
        <div class="flex justify-center">
            <div class="max-w-xl w-full space-y-8">
                <form class="space-y-8">
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
                                    readonly
                                    prop:value=db_path
                                />
                                <button
                                    type="button"
                                    class="px-4 py-2 bg-background border border-primary-100 hover:bg-primary-400/10 text-white rounded focus:outline-none transition-all duration-200"
                                >
                                    <Icon icon=folder_icon.into() class="w-5 h-5 text-primary-100" />
                                </button>
                            </div>
                        </div>
                    </fieldset>

                    <fieldset class="space-y-4">
                        <h3 class="text-lg font-semibold bg-gradient-primary bg-clip-text text-transparent mb-4">
                            <div class="flex items-center space-x-2">
                                <Icon icon=backup_icon.into() class="w-5 h-5 text-primary-100" />
                                <span>"Backup-Einstellungen"</span>
                            </div>
                        </h3>

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

                        {move || auto_backup.get().then(||
                            view! {
                                <div class="relative w-40">
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
                            }
                        )}

                        <div class="space-y-4 pt-2">
                            <button
                                type="button"
                                class="w-full flex items-center justify-center space-x-2 bg-background border border-primary-100 hover:bg-primary-400/10 text-white py-2 px-4 rounded focus:outline-none transition-all duration-200"
                            >
                                <Icon icon=backup_icon.into() class="w-5 h-5 text-primary-100" />
                                <span>"Backup erstellen"</span>
                            </button>
                            <button
                                type="button"
                                class="w-full flex items-center justify-center space-x-2 bg-background border border-primary-100 hover:bg-primary-400/10 text-white py-2 px-4 rounded focus:outline-none transition-all duration-200"
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

                        <div class="space-y-4">
                            <button
                                type="button"
                                class="w-full flex items-center justify-center space-x-2 bg-background border border-primary-100 hover:bg-primary-400/10 text-white py-2 px-4 rounded focus:outline-none transition-all duration-200"
                            >
                                <Icon icon=import_icon.into() class="w-5 h-5 text-primary-100" />
                                <span>"CSV importieren"</span>
                            </button>
                            <button
                                type="button"
                                class="w-full flex items-center justify-center space-x-2 bg-background border border-primary-100 hover:bg-primary-400/10 text-white py-2 px-4 rounded focus:outline-none transition-all duration-200"
                            >
                                <Icon icon=export_icon.into() class="w-5 h-5 text-primary-100" />
                                <span>"Als CSV exportieren"</span>
                            </button>
                        </div>
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
        </div>
    }
}
