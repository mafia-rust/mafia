export let langMap: ReadonlyMap<string, string>;
export let langText: string;
export let langJson: any;

export const LANGUAGES = ["en_us", "broken_keyboard", "dyslexic"] as const;
export type Language = typeof LANGUAGES[number]
switchLanguage("en_us");

export function switchLanguage(language: Language) {
    import("../resources/lang/" + language + ".json").then((langJson)=>{
        langMap = new Map<string, string>(Object.entries(langJson));
        langText = JSON.stringify(langJson, null, 1);
    })
}

/// Returns the translated string with the given key, replacing the placeholders with the given values.
export default function translate(langKey: string, ...valuesList: (string | number)[]): string {
    const translation = translateChecked(langKey, ...valuesList);

    if (translation === null) {
        console.error("Attempted to use non existent lang key: "+langKey);
        return "ERROR: "+langKey;
    } 

    return translation;
}

export function translateAny(langKeys: string[], ...valuesList: (string | number)[]): string {
    for (const key of langKeys) {
        const translation = translateChecked(key, ...valuesList);

        if (translation !== null) {
            return translation;
        }
    }

    console.error("Attempted to use non existent lang key: "+langKeys.at(-1));
    return "ERROR: "+langKeys.at(-1);
}

export function translateChecked(langKey: string, ...valuesList: (string | number)[]): string | null {
    let out = langMap.get(langKey);
    if(out===undefined){
        return null;
    }
    for(let i = 0; i < valuesList.length; i++){
        out = out.replace("\\"+(i), valuesList[i] as string);
    }
    return out;
}

// export function languageName(language: Language): string {
//     const json = require("../resources/lang/" + language + ".json");
//     return json.language;
// }
export function languageName(language: Language): string {
    let languageData: string = '';
    import(`../resources/lang/${language}.json`)
        .then((json) => {
            languageData = json.language;
        })
        .catch((err) => {
            console.error("Error loading language JSON:", err);
            languageData = 'default'; // Fallback value
        });

    return languageData;
}
