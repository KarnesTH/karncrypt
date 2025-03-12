use leptos::*;

use crate::components::icons::Icon;

#[derive(Clone, PartialEq)]
enum SortOrder {
    ScoreAsc,
    ScoreDesc,
}

#[component]
pub fn Dashboard() -> impl IntoView {
    let (show_filter_dropdown, set_show_filter_dropdown) = create_signal(false);
    let (sort_order, set_sort_order) = create_signal(SortOrder::ScoreDesc);

    let dashboard_icon = create_memo(move |_| "chart-pie");
    let refresh_icon = create_memo(move |_| "arrow-path");
    let warning_icon = create_memo(move |_| "exclamation-triangle");
    let service_icon = create_memo(move |_| "bookmark");
    let score_icon = create_memo(move |_| "chart-bar");
    let filter_icon = create_memo(move |_| "funnel");

    view! {
        <div class="w-full flex flex-col h-full">
            <div class="flex justify-between items-center mb-4">
                <h2 class="text-2xl font-bold mb-0 bg-gradient-primary bg-clip-text text-transparent flex items-center">
                    <Icon icon=dashboard_icon.into() class="w-8 h-8 mr-3 text-primary-100" />
                    "Dashboard"
                </h2>
                <div class="flex gap-6">
                    <button
                        class="group relative flex items-center text-gray-400 hover:text-white transition-colors"
                        on:click=move |_| {}
                    >
                        <Icon icon=refresh_icon.into() class="w-5 h-5" />
                        <span class="ml-2 whitespace-nowrap max-w-0 overflow-hidden group-hover:max-w-[100px] transition-all duration-300 text-primary-100">
                            "Aktualisieren"
                        </span>
                    </button>
                </div>
            </div>
            <div class="flex-1 overflow-y-auto">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
                    <div class="bg-background p-6 rounded-lg border border-gray-600">
                        <div class="flex flex-col items-center">
                            <div class="relative w-40 h-40">
                                <div class="absolute inset-0 rounded-full border-8 border-gray-600"></div>
                                <div
                                    class="absolute inset-0 rounded-full border-8 border-primary-100"
                                    style="clip-path: polygon(50% 50%, 50% 0, 100% 0, 100% 100%, 0 100%, 0 0, 50% 0)"
                                ></div>
                                <div class="absolute inset-0 flex flex-col items-center justify-center">
                                    <span class="text-4xl font-bold text-primary-100">"85"</span>
                                    <span class="text-sm text-gray-400">"Durchschnitt"</span>
                                </div>
                            </div>
                            <h3 class="mt-4 text-lg font-semibold">"Gesamt Score"</h3>
                        </div>
                    </div>

                    <div class="bg-background p-6 rounded-lg border border-gray-600">
                        <h3 class="text-lg font-semibold mb-4">"Status"</h3>
                        <div class="space-y-4">
                            <div class="flex items-center justify-between">
                                <span class="text-gray-400">"Starke Passwörter"</span>
                                <div class="flex items-center space-x-2">
                                    <div class="w-2 h-2 rounded-full bg-green-500"></div>
                                    <span class="text-sm">"12 von 20"</span>
                                </div>
                            </div>
                            <div class="flex items-center justify-between">
                                <span class="text-gray-400">"Schwache Passwörter"</span>
                                <div class="flex items-center space-x-2">
                                    <div class="w-2 h-2 rounded-full bg-red-500"></div>
                                    <span class="text-sm">"8 von 20"</span>
                                </div>
                            </div>
                            <div class="flex items-center justify-between">
                                <span class="text-gray-400">"Doppelte Passwörter"</span>
                                <div class="flex items-center space-x-2">
                                    <div class="w-2 h-2 rounded-full bg-yellow-500"></div>
                                    <span class="text-sm">"3"</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="bg-background rounded-lg border border-gray-600">
                    <div class="p-4 border-b border-gray-600 flex justify-between items-center">
                        <h3 class="text-lg font-semibold">"Passwort Analyse"</h3>
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
                                        class:text-primary-100=move || sort_order.get() == SortOrder::ScoreDesc
                                        on:click=move |_| {
                                            set_sort_order.set(SortOrder::ScoreDesc);
                                            set_show_filter_dropdown.set(false);
                                        }
                                    >
                                        "Score (Hoch zu Niedrig)"
                                    </button>
                                    <button
                                        class="w-full text-left px-3 py-2 hover:bg-background-light rounded transition-colors flex items-center gap-2"
                                        class:text-primary-100=move || sort_order.get() == SortOrder::ScoreAsc
                                        on:click=move |_| {
                                            set_sort_order.set(SortOrder::ScoreAsc);
                                            set_show_filter_dropdown.set(false);
                                        }
                                    >
                                        "Score (Niedrig zu Hoch)"
                                    </button>
                                </div>
                            })}
                        </div>
                    </div>
                    <div class="overflow-auto">
                        <table class="w-full">
                            <thead class="bg-background sticky top-0">
                                <tr class="border-b border-gray-600">
                                    <th class="text-left p-4 text-gray-400 font-medium">
                                        <div class="flex items-center">
                                            <Icon icon=service_icon.into() class="w-4 h-4 mr-2" />
                                            "Service"
                                        </div>
                                    </th>
                                    <th class="text-left p-4 text-gray-400 font-medium">
                                        <div class="flex items-center">
                                            <Icon icon=score_icon.into() class="w-4 h-4 mr-2" />
                                            "Score"
                                        </div>
                                    </th>
                                    <th class="text-left p-4 text-gray-400 font-medium">
                                        <div class="flex items-center">
                                            <Icon icon=warning_icon.into() class="w-4 h-4 mr-2" />
                                            "Status"
                                        </div>
                                    </th>
                                    <th class="text-left p-4 text-gray-400 font-medium">
                                        "Vorschläge"
                                    </th>
                                </tr>
                            </thead>
                            <tbody class="divide-y divide-gray-600">
                                <tr class="hover:bg-background-light">
                                    <td class="p-4">"Google"</td>
                                    <td class="p-4">
                                        <div class="flex items-center">
                                            <span class="text-primary-100 font-medium">"92"</span>
                                            <div class="w-24 h-2 bg-gray-700 rounded-full ml-2">
                                                <div class="h-2 bg-primary-100 rounded-full" style="width: 92%"></div>
                                            </div>
                                        </div>
                                    </td>
                                    <td class="p-4">
                                        <div class="flex items-center">
                                            <div class="w-2 h-2 rounded-full bg-green-500 mr-2"></div>
                                            "Sehr stark"
                                        </div>
                                    </td>
                                    <td class="p-4 text-gray-400">
                                        "Keine Verbesserungen notwendig"
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
    }
}
