use leptos::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div>
                <h3 class="text-lg font-semibold bg-gradient-primary bg-clip-text text-transparent mb-2">
                    "Passwort Manager"
                </h3>
                <p class="text-gray-300">
                "Dein persönlicher Passwort-Manager, der Sicherheit und Benutzerfreundlichkeit vereint."
                </p>
                <p class="text-gray-300">
                "Verwalte deine Passwörter sicher und effizient, während deine Daten durch modernste Verschlüsselung geschützt sind."
                </p>
            </div>

            <div>
                <h4 class="text-md font-semibold text-primary-200 mb-2">
                    "Version"
                </h4>
                <p class="text-gray-300">
                    {env!("CARGO_PKG_VERSION")}
                </p>
            </div>

            <div>
                <h4 class="text-md font-semibold text-primary-200 mb-2">
                    "Entwickler"
                </h4>
                <p class="text-gray-300">
                    "Patrick Hähnel"
                </p>
            </div>

            <div>
                <h4 class="text-md font-semibold text-primary-200 mb-2">
                    "Features"
                </h4>
                <ul class="list-disc list-inside text-gray-300 space-y-1">
                    <li>"Maximale Sicherheit durch lokale Verschlüsselung"</li>
                    <li>"Intelligenter Passwort-Generator für starke Passwörter"</li>
                    <li>"Geschützter Zugriff durch Master-Passwort"</li>
                    <li>"Einfache Passwort-Verwaltung"</li>
                </ul>
            </div>

            <div>
                <h4 class="text-md font-semibold text-primary-200 mb-2">
                    "Support"
                </h4>
                <p class="text-gray-300 mb-2">
                    "Dir gefällt dieser Passwort-Manager? Unterstütze mich gerne mit einem Kaffee ☕"
                </p>
                <a
                    href="https://ko-fi.com/karnesdevelopment"
                    target="_blank"
                    class="inline-flex items-center px-4 py-2 bg-gradient-primary text-white rounded hover:opacity-90 transition-opacity"
                >
                    <img
                        src="https://storage.ko-fi.com/cdn/cup-border.png"
                        alt="Ko-fi Logo"
                        class="w-6 h-6 mr-2"
                    />
                    "Support auf Ko-fi"
                </a>
            </div>
        </div>
    }
}
