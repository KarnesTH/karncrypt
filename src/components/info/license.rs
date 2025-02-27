use leptos::*;

#[component]
pub fn License() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div>
                <h3 class="text-lg font-semibold bg-gradient-primary bg-clip-text text-transparent mb-2">
                    "MIT Lizenz"
                </h3>
                <p class="text-gray-300">
                    "Copyright © 2025 Patrick Hähnel"
                </p>
            </div>

            <div>
                <h4 class="text-md font-semibold text-primary-200 mb-2">
                    "Nutzungsbedingungen"
                </h4>
                <p class="text-gray-300 whitespace-pre-line">
                    "Hiermit wird unentgeltlich jeder Person, die eine Kopie der Software und der zugehörigen Dokumentationen (die \"Software\") erhält, die Erlaubnis erteilt, sie uneingeschränkt zu nutzen, inklusive und ohne Ausnahme dem Recht, sie zu verwenden, zu kopieren, zu ändern, zu fusionieren, zu verlegen, zu verbreiten, zu unterlizenzieren und/oder zu verkaufen, und Personen, denen diese Software überlassen wird, diese Rechte zu verschaffen, unter den folgenden Bedingungen:"
                </p>
            </div>

            <div>
                <h4 class="text-md font-semibold text-primary-200 mb-2">
                    "Bedingungen"
                </h4>
                <p class="text-gray-300 mb-4">
                    "Der obige Urheberrechtsvermerk und dieser Erlaubnisvermerk sind in allen Kopien oder Teilkopien der Software beizulegen."
                </p>
            </div>

            <div>
                <h4 class="text-md font-semibold text-primary-200 mb-2">
                    "Haftungsausschluss"
                </h4>
                <p class="text-gray-300">
                    "DIE SOFTWARE WIRD OHNE JEDE AUSDRÜCKLICHE ODER IMPLIZIERTE GARANTIE BEREITGESTELLT, EINSCHLIESSLICH DER GARANTIE ZUR BENUTZUNG FÜR DEN VORGESEHENEN ODER EINEM BESTIMMTEN ZWECK SOWIE JEGLICHER RECHTSVERLETZUNG, JEDOCH NICHT DARAUF BESCHRÄNKT. IN KEINEM FALL SIND DIE AUTOREN ODER COPYRIGHTINHABER FÜR JEGLICHEN SCHADEN ODER SONSTIGE ANSPRÜCHE HAFTBAR ZU MACHEN, OB INFOLGE DER ERFÜLLUNG EINES VERTRAGES, EINES DELIKTES ODER ANDERS IM ZUSAMMENHANG MIT DER SOFTWARE ODER SONSTIGER VERWENDUNG DER SOFTWARE ENTSTANDEN."
                </p>
            </div>
        </div>
    }
}
