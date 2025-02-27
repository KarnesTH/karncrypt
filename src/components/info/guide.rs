use leptos::*;

#[component]
pub fn Guide() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div>
                <h3 class="text-lg font-semibold bg-gradient-primary bg-clip-text text-transparent mb-2">
                    "Sichere Passwörter erstellen"
                </h3>
                <p class="text-gray-300">
                    "Ein sicheres Passwort ist der Grundstein für den Schutz deiner Daten. "
                </p>
                <p class="text-gray-300">
                    "Hier sind die wichtigsten Regeln für sichere Passwörter:"
                </p>
            </div>

            <div>
                <h4 class="text-md font-semibold text-primary-200 mb-2">
                    "Grundregeln"
                </h4>
                <ul class="list-disc list-inside text-gray-300 space-y-1">
                    <li>"Mindestens 12 Zeichen Länge"</li>
                    <li>"Kombiniere Groß- und Kleinbuchstaben"</li>
                    <li>"Verwende Zahlen und Sonderzeichen"</li>
                    <li>"Vermeide persönliche Informationen"</li>
                    <li>"Nutze für jeden Dienst ein eigenes Passwort"</li>
                </ul>
            </div>

            <div>
                <h4 class="text-md font-semibold text-primary-200 mb-2">
                    "Tipps"
                </h4>
                <ul class="list-disc list-inside text-gray-300 space-y-1">
                    <li>"Nutze den integrierten Passwort-Generator für maximale Sicherheit"</li>
                    <li>"Aktualisiere wichtige Passwörter regelmäßig"</li>
                    <li>"Aktiviere wenn möglich Zwei-Faktor-Authentifizierung"</li>
                    <li>"Speichere alle Passwörter sicher in diesem Passwort-Manager"</li>
                </ul>
            </div>

            <div>
                <h4 class="text-md font-semibold text-primary-200 mb-2">
                    "Warnsignale"
                </h4>
                <ul class="list-disc list-inside text-gray-300 space-y-1">
                    <li>"Verwende nie das gleiche Passwort mehrfach"</li>
                    <li>"Vermeide offensichtliche Muster (123456, qwerty)"</li>
                    <li>"Schreibe Passwörter nicht auf Papier oder in unverschlüsselte Dateien"</li>
                    <li>"Gib deine Passwörter niemals weiter"</li>
                </ul>
            </div>
        </div>
    }
}
