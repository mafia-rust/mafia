import { ReactElement } from "react";
import { Role } from "../game/roleState.d";
import ROLES from "../resources/roles.json";
import React from "react";
import translate, { langText, translateChecked } from "../game/lang";
import StyledText from "./StyledText";
import { ROLE_SETS, RoleSet, getRolesFromRoleSet } from "../game/roleListState.d";

export type WikiArticleLink = 
    `role/${Role}` | 
    `standard/${StandardArticle}` |
    `generated/${GeneratedArticle}`;

const STANDARD_ARTICLES = ["how_to_play", "phases_and_timeline", "priority"] as const;
type StandardArticle = typeof STANDARD_ARTICLES[number];

const GENERATED_ARTICLES = ["role_sets", "all_text"] as const;
type GeneratedArticle = typeof GENERATED_ARTICLES[number];

export const ARTICLES: WikiArticleLink[] = 
    Object.keys(ROLES).map(role => `role/${role}`)
    .concat(STANDARD_ARTICLES.map(article => `standard/${article}`))
    .concat(GENERATED_ARTICLES.map(article => `generated/${article}`)) as WikiArticleLink[];
    

export default function WikiArticle(props: {
    article: WikiArticleLink
}): ReactElement {
    
    const path = props.article.split('/');

    switch (path[0]) {
        case "role":
            const role = path[1] as Role;
            const roleData = ROLES[role];
            const keywords = roleData.keywords.map(key => {
                return `<details><summary>${translate("keyword."+key)}</summary>${translate("wiki.keyword." + key)}</details>`;
            }).join('\n');

            return <StyledText className="wiki-content-body" markdown={true}>{
                translate("wiki.article.role",
                    translate("role."+role+".name"),
                    translateChecked("wiki.article.role."+role+".guide") ?? translate("wiki.article.role.noGuide"),
                    translateChecked("wiki.article.role."+role+".abilities") ?? translate("wiki.article.role.noAbilities"),
                    translateChecked("wiki.article.role."+role+".attributes") ?? translate("wiki.article.role.noAttributes"),
                    translateChecked("wiki.article.role."+role+".extra") ?? translate("wiki.article.role.noExtra"),
                    roleData.maxCount === null ? translate("none") : roleData.maxCount,
                    translate("defense."+roleData.defense),
                    keywords
                )
            }</StyledText>
        case "standard":
            return <StyledText className="wiki-content-body" markdown={true}>
                {"# "+translate(`wiki.article.${props.article.replace('/', '.')}.title`)+"\n"}
                {translate(`wiki.article.${props.article.replace('/', '.')}.text`)}
            </StyledText>
        case "generated":
            return getGeneratedArticle(path[1] as GeneratedArticle);
    }

    return <></>;
}


function getGeneratedArticle(article: GeneratedArticle){
    switch(article){
        case "role_sets":
            let mainElements = [
                <StyledText key="role_sets" className="wiki-content-body" markdown={true}>
                    {"# "+translate("wiki.article.generated.role_sets.title")}
                </StyledText>
            ];
            
            for(let set of ROLE_SETS){
                mainElements.push(<StyledText key={set} className="wiki-content-body" markdown={true}>
                    {"### "+translate(set)}
                </StyledText>);
                
                let elements = getRolesFromRoleSet(set as RoleSet).map((role)=>{
                    return <button key={role}>
                        <StyledText key={set} className="wiki-content-body">
                            {translate("role."+role+".name")}
                        </StyledText>
                    </button>
                });
                mainElements.push(<blockquote>
                    {elements}
                </blockquote>);
            }
            return <div className="wiki-content-body">{mainElements}</div>;
        case "all_text":
            return <>
                <h1>{translate("wiki.article.generated.all_text.title")}</h1>
                {langText}
            </>;
    }
}

export function getArticleTitle(page: WikiArticleLink): string {
    const path = page.split('/');

    switch (path[0]) {
        case "role":
            return translate(`role.${path[1]}.name`);
        default:
            return translate(`wiki.article.${page.replace('/', '.')}.title`)
    }
}
