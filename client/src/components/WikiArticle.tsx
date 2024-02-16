import { ReactElement } from "react";
import { Role } from "../game/roleState.d";
import ROLES from "../resources/roles.json";
import React from "react";
import translate, { langJson, langText, translateChecked } from "../game/lang";
import StyledText from "./StyledText";
import { ROLE_SETS, RoleSet, getRolesFromRoleSet } from "../game/roleListState.d";
import ChatElement, { ChatMessage } from "./ChatMessage";
import DUMMY_NAMES from "../resources/dummyNames.json";

export type WikiArticleLink = 
    `role/${Role}` | 
    `standard/${StandardArticle}` |
    `generated/${GeneratedArticle}`;

const STANDARD_ARTICLES = 
    [...new Set(Object.keys(langJson).filter(key => key.startsWith("wiki.article.standard.")).map(key => key.split(".")[3]))];

type StandardArticle = typeof STANDARD_ARTICLES[number];

const GENERATED_ARTICLES = ["role_set", "all_text"] as const;
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
                return `<details><summary>${getArticleTitle("standard/"+key as WikiArticleLink)}</summary>${translate("wiki.article.standard."+key+".text")}</details>`;
            }).join('\n');
            return <section>
                <StyledText className="wiki-content-body" markdown={true}>
                    {"# "+translate("role."+role+".name")+"\n"}
                    {"### "+translate(roleData.faction)+"\n"}

                    {"### "+translate("wiki.article.role.guide")+"\n"}
                    {(translateChecked("wiki.article.role."+role+".guide") ?? translate("wiki.article.role.noGuide"))+"\n"}

                    {"### "+translate("wiki.article.role.abilities")+"\n"}
                    {(translateChecked("wiki.article.role."+role+".abilities") ?? translate("wiki.article.role.noAbilities"))+"\n"}
                    
                    {"### "+translate("wiki.article.role.attributes")+"\n"}
                    {(translateChecked("wiki.article.role."+role+".attributes") ?? translate("wiki.article.role.noAttributes"))+"\n"}
                    
                    {"### "+translate("wiki.article.role.extra")+"\n"}
                    {(translateChecked("wiki.article.role."+role+".extra") ?? translate("wiki.article.role.noExtra"))+"\n"}

                    {"### "+translate("wiki.article.standard.roleLimit.title")+": "+(roleData.maxCount === null ? translate("none") : roleData.maxCount)+"\n"}
                    {"### "+translate("defense")+": "+translate("defense."+(roleData.armor ? "1" : "0"))+"\n"}

                    {"### "+translate("wiki.article.standard.chat.title")+"\n"}
                </StyledText>
                <div className="wiki-message-section">
                    {roleData.chatMessages.map((msg: any)=>
                        <ChatElement message={msg as ChatMessage} playerNames={DUMMY_NAMES}/>
                    )}
                </div>
                <StyledText className="wiki-content-body" markdown={true}>
                    {keywords}
                </StyledText>
            </section>;
        case "standard":
            return <StyledText className="wiki-content-body" markdown={true}>
                {"# "+translate(`wiki.article.standard.${props.article.split("/")[1]}.title`)+"\n"}
                {translate(`wiki.article.standard.${props.article.split("/")[1]}.text`)}
            </StyledText>
        case "generated":
            return getGeneratedArticle(path[1] as GeneratedArticle);
    }

    return <></>;
}


function getGeneratedArticle(article: GeneratedArticle){
    switch(article){
        case "role_set":
            let mainElements = [
                <section key="title"><StyledText markdown={true}>
                    {"# "+translate("wiki.article.generated.role_set.title")}
                </StyledText></section>
            ];
            
            for(let set of ROLE_SETS){
                mainElements.push(<section key={set+"title"}><StyledText markdown={true}>
                    {"### "+translate(set)}
                </StyledText></section>);
                
                let elements = getRolesFromRoleSet(set as RoleSet).map((role)=>{
                    return <button key={role}>
                        <StyledText>
                            {translate("role."+role+".name")}
                        </StyledText>
                    </button>
                });
                mainElements.push(<blockquote key={set}>
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
        case "standard":
            return translate(`wiki.article.standard.${path[1]}.title`);
        case "generated":
            return translate(`wiki.article.generated.${path[1]}.title`);
        default:
            console.error("Invalid article type: "+path[0]);
            return "ERROR";
    }
}
