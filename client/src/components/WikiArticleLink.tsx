import translate, { langJson } from "../game/lang";
import { Role } from "../game/roleState.d";
import ROLES from "../resources/roles.json";

export type WikiArticleLink = 
    `role/${Role}` | 
    `standard/${StandardArticle}` |
    `generated/${GeneratedArticle}`;

const STANDARD_ARTICLES = 
    [...new Set(Object.keys(langJson).filter(key => key.startsWith("wiki.article.standard.")).map(key => key.split(".")[3]))];

export type StandardArticle = typeof STANDARD_ARTICLES[number];

const GENERATED_ARTICLES = ["role_set", "all_text"] as const;
export type GeneratedArticle = typeof GENERATED_ARTICLES[number];

export const ARTICLES: WikiArticleLink[] = 
    Object.keys(ROLES).map(role => `role/${role}`)
    .concat(STANDARD_ARTICLES.map(article => `standard/${article}`))
    .concat(GENERATED_ARTICLES.map(article => `generated/${article}`)) as WikiArticleLink[];


export function getArticleLangKey(page: WikiArticleLink): string {
    const path = page.split('/');


    switch (path[0]) {
        case "role":
            return `role.${path[1]}.name`;
        case "standard":
            return `wiki.article.standard.${path[1]}.title`;
        case "generated":
            return `wiki.article.generated.${path[1]}.title`;
        default:
            console.error("Invalid article type: "+path[0]);
            return "ERROR";
    }
}

export function getArticleTitle(page: WikiArticleLink): string {
    return translate(getArticleLangKey(page));
}