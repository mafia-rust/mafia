import { MODIFIERS, ModifierType } from "../game/gameState.d";
import translate, { langJson } from "../game/lang";
import { Role } from "../game/roleState.d";
import { WIKI_CATEGORIES, WikiCategory } from "./Wiki";
import "./wiki.css";
import { getAllRoles } from "../game/roleListState.d"

export type WikiArticleLink = 
    `role/${Role}` | 
    `modifier/${ModifierType}` |
    `category/${WikiCategory}` |
    `standard/${StandardArticle}` |
    `generated/${GeneratedArticle}`;

const STANDARD_ARTICLES = 
    [...new Set(Object.keys(langJson).filter(key => key.startsWith("wiki.article.standard.")).map(key => key.split(".")[3]))];

export type StandardArticle = typeof STANDARD_ARTICLES[number];

const GENERATED_ARTICLES = ["roleSet", "all_text"] as const;
export type GeneratedArticle = typeof GENERATED_ARTICLES[number];

export const ARTICLES: WikiArticleLink[] = 
    WIKI_CATEGORIES.map(category => `category/${category}`)
    .concat(getAllRoles().map(role => `role/${role}`))
    .concat(MODIFIERS.map(modifier => `modifier/${modifier}`))
    .concat(STANDARD_ARTICLES.map(article => `standard/${article}`))
    .concat(GENERATED_ARTICLES.map(article => `generated/${article}`)) as WikiArticleLink[];


export function getArticleLangKey(page: WikiArticleLink): string {
    const path = page.split('/');


    switch (path[0]) {
        case "role":
            return `role.${path[1]}.name`;
        case "modifier":
            return `wiki.article.modifier.${path[1]}.title`;
        case "category":
            return `wiki.category.${path[1]}`;
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

export function wikiPageIsEnabled(
    page: WikiArticleLink,
    enabledRoles: Role[],
    enabledModifiers: ModifierType[]
): boolean {
    switch (page.split("/")[0]) {
        case "role":
            return enabledRoles.map(role => `role/${role}`).includes(page)
        case "modifiers":
            return enabledModifiers.map(modifier => `modifier/${modifier}`).includes(page)
        default:
            return true;
    }
}