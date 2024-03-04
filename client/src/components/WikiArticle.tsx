import { ReactElement } from "react";
import { Role } from "../game/roleState.d";
import ROLES from "../resources/roles.json";
import React from "react";
import translate, { langText, translateChecked } from "../game/lang";
import StyledText from "./StyledText";
import { ROLE_SETS, RoleSet, getRolesFromRoleSet } from "../game/roleListState.d";
import ChatElement, { ChatMessage } from "./ChatMessage";
import DUMMY_NAMES from "../resources/dummyNames.json";
import { GeneratedArticle, WikiArticleLink } from "./WikiArticleLink";
import { replaceMentions } from "..";
    

export default function WikiArticle(props: {
    article: WikiArticleLink
}): ReactElement {
    
    const path = props.article.split('/');

    switch (path[0]) {
        case "role":
            const role = path[1] as Role;
            const roleData = ROLES[role];

            return <section className="wiki-article">
                <div>
                    <StyledText markdown={true}>
                        {"# "+translate("role."+role+".name")+"\n"}
                        {roleData.roleSet!==null?("### "+translateChecked(roleData.roleSet)+"\n"):"### "+translate(roleData.faction)+"\n"}
                    </StyledText>
                </div>
                <div>
                    <StyledText markdown={true}>
                        {"### "+translate("wiki.article.role.guide")+"\n"}
                        {replaceMentions(translateChecked("wiki.article.role."+role+".guide") ?? translate("wiki.article.role.noGuide"), DUMMY_NAMES)}
                    </StyledText>
                </div>
                <div className="wiki-message-section">
                    <StyledText markdown={true}>
                        {"### "+translate("wiki.article.role.chatMessages")+"\n"}
                    </StyledText>
                    {roleData.chatMessages.map((msg: any, i)=>
                        <ChatElement key={i} message={msg as ChatMessage} playerNames={DUMMY_NAMES}/>
                    )}
                </div>
                {roleData.armor && <div>
                    <StyledText markdown={true}>
                        {"### "+translate("defense")+": "+translate("defense.1")+"\n"}
                    </StyledText>
                </div>}
                <details>
                    <summary>{translate("wiki.article.role.details")}</summary>
                    <div>
                        <StyledText markdown={true}>
                            {"### "+translate("wiki.article.role.abilities")+"\n"}
                            {(translateChecked("wiki.article.role."+role+".abilities") ?? translate("wiki.article.role.noAbilities"))+"\n"}

                            {"### "+translate("wiki.article.role.attributes")+"\n"}
                            {(translateChecked("wiki.article.role."+role+".attributes") ?? translate("wiki.article.role.noAttributes"))+"\n"}

                            {"### "+translate("wiki.article.role.extra")+"\n"}
                            {(translateChecked("wiki.article.role."+role+".extra") ?? translate("wiki.article.role.noExtra"))+"\n"}

                            {"### "+translate("wiki.article.standard.roleLimit.title")+": "+(roleData.maxCount === null ? translate("none") : roleData.maxCount)+"\n"}
                            {"### "+translate("defense")+": "+translate("defense."+(roleData.armor ? "1" : "0"))+"\n"}
                        </StyledText>
                    </div>
                </details>
            </section>;
        case "standard":
            return <section className="wiki-article">
                <StyledText className="wiki-article-standard" markdown={true}>
                    {"# "+translate(`wiki.article.standard.${props.article.split("/")[1]}.title`)+"\n"}
                    {translate(`wiki.article.standard.${props.article.split("/")[1]}.text`)}
                </StyledText>
            </section>
        case "generated":
            return <section className="wiki-article">{getGeneratedArticle(path[1] as GeneratedArticle)}</section>
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
            return <div>{mainElements}</div>;
        case "all_text":
            return <>
                <h1>{translate("wiki.article.generated.all_text.title")}</h1>
                {langText}
            </>;
    }
}
