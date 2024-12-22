import { ReactElement } from "react";
import { Role, roleJsonData } from "../game/roleState.d";
import React from "react";
import translate, { langText, translateChecked } from "../game/lang";
import StyledText, { DUMMY_NAMES_KEYWORD_DATA, DUMMY_NAMES_SENDER_KEYWORD_DATA, StyledTextProps } from "./StyledText";
import { ROLE_SETS, getRolesFromRoleSet } from "../game/roleListState.d";
import ChatElement, { ChatMessageVariant } from "./ChatMessage";
import DUMMY_NAMES from "../resources/dummyNames.json";
import { GeneratedArticle, WikiArticleLink } from "./WikiArticleLink";
import "./wiki.css";
import GAME_MANAGER, { replaceMentions } from "..";
import { useLobbyOrGameState } from "./useHooks";
import DetailsSummary from "./DetailsSummary";

function WikiStyledText(props: Omit<StyledTextProps, 'markdown' | 'playerKeywordData'>): ReactElement {
    return <StyledText {...props} markdown={true} playerKeywordData={DUMMY_NAMES_KEYWORD_DATA} />
}

export default function WikiArticle(props: {
    article: WikiArticleLink
}): ReactElement {
    
    const path = props.article.split('/');

    switch (path[0]) {
        case "role":
            const role = path[1] as Role;
            const roleData = roleJsonData()[role];
            const chatMessages = roleData.chatMessages as ChatMessageVariant[];

            return <section className="wiki-article">
                <div>
                    <WikiStyledText>
                        {"# "+translate("role."+role+".name")+"\n"}
                        {"### "+roleData.roleSets.map((roleSet)=>{return translate(roleSet)}).join(" | ")+"\n"}

                        {"### "+translate("wiki.article.role.reminder")+"\n"}
                        {replaceMentions(translateChecked("wiki.article.role."+role+".reminder") ?? translate("wiki.article.role.noReminder"), DUMMY_NAMES)+"\n"}

                        {"### "+translate("wiki.article.role.guide")+"\n"}
                        {replaceMentions(translateChecked("wiki.article.role."+role+".guide") ?? translate("wiki.article.role.noGuide"), DUMMY_NAMES)+"\n"}
                    </WikiStyledText>
                </div>
                <div>
                    {roleData.aura &&
                        <WikiStyledText>
                            {"### "+translate("wiki.article.standard.aura.title")+": "+translate(roleData.aura+"Aura")+"\n"}
                        </WikiStyledText>
                    }
                    {roleData.armor && 
                        <WikiStyledText>
                            {"### "+translate("defense")+": "+translate("defense.1")+"\n"}
                        </WikiStyledText>
                    }
                    {roleData.maxCount !== null &&
                        <WikiStyledText>
                        {"### "+translate("wiki.article.standard.roleLimit.title")+": "+(roleData.maxCount)+"\n"}
                        </WikiStyledText>
                    }
                </div>
                {chatMessages.length!==0 && <div className="wiki-message-section">
                    <WikiStyledText>
                        {"### "+translate("wiki.article.role.chatMessages")+"\n"}
                    </WikiStyledText>
                    {chatMessages.map((msgvariant, i)=>
                        <ChatElement key={i} 
                            message={
                                {
                                    variant: msgvariant,
                                    chatGroup: "all",
                                }
                            } 
                            playerNames={DUMMY_NAMES} 
                            playerKeywordData={DUMMY_NAMES_KEYWORD_DATA}
                            playerSenderKeywordData={DUMMY_NAMES_SENDER_KEYWORD_DATA}
                        />
                    )}
                </div>}
                <DetailsSummary 
                    summary={translate("wiki.article.role.details")}
                >
                    <WikiStyledText>
                        {"### "+translate("wiki.article.role.abilities")+"\n"}
                        {(translateChecked("wiki.article.role."+role+".abilities") ?? translate("wiki.article.role.noAbilities"))+"\n"}

                        {"### "+translate("wiki.article.role.attributes")+"\n"}
                        {(translateChecked("wiki.article.role."+role+".attributes") ?? translate("wiki.article.role.noAttributes"))+"\n"}

                        {"### "+translate("wiki.article.role.extra")+"\n"}
                        {(translateChecked("wiki.article.role."+role+".extra") ?? translate("wiki.article.role.noExtra"))+"\n"}

                        {"### "+translate("wiki.article.standard.roleLimit.title")+": "+(roleData.maxCount === null ? translate("none") : roleData.maxCount)+"\n"}
                        {"### "+translate("defense")+": "+translate("defense."+(roleData.armor ? "1" : "0"))+"\n"}
                        {"### "+translate("wiki.article.standard.aura.title")+": "+(roleData.aura?translate(roleData.aura+"Aura"):translate("none"))+"\n"}
                    </WikiStyledText>
                </DetailsSummary>
            </section>;
        case "standard":
            return <section className="wiki-article">
                <WikiStyledText className="wiki-article-standard">
                    {"# "+translate(`wiki.article.standard.${props.article.split("/")[1]}.title`)+"\n"}
                    {replaceMentions(translate(`wiki.article.standard.${props.article.split("/")[1]}.text`))}
                </WikiStyledText>
            </section>
        case "generated":
            return <section className="wiki-article">
                <GeneratedArticleElement article={path[1] as GeneratedArticle}/>
            </section>
    }

    return <></>;
}


function GeneratedArticleElement(props: Readonly<{ article: GeneratedArticle }>): ReactElement {
    switch(props.article){
        case "roleSet":
            return <RoleSetArticle />
        case "all_text":
            return <pre>
                <h1>{translate("wiki.article.generated.all_text.title")}</h1>
                <StyledText className="code">{langText.substring(1, langText.length - 1)}</StyledText>
            </pre>;
    }
}

function RoleSetArticle(): ReactElement {
    const enabledRoles = useLobbyOrGameState(
        state => state.enabledRoles,
        ["enabledRoles"]
    );

    return <div>
        <section key="title">
            <WikiStyledText>{"# "+translate("wiki.article.generated.roleSet.title")}</WikiStyledText>
        </section>
        {ROLE_SETS.map(set => {
            const description = translateChecked(`${set}.description`);
            return <>
                <h3 key={set+"title"} className="wiki-search-divider">
                    <StyledText>{translate(set)}</StyledText>
                </h3>
                {description && <p><StyledText>{description}</StyledText></p>}
                {getRolesFromRoleSet(set).map((role)=>{
                    let className = "";
                    if(enabledRoles !== undefined && !enabledRoles.includes(role)) {
                        className = "keyword-disabled";
                    }

                    return <button key={role} className={className} 
                        onClick={() => GAME_MANAGER.setWikiArticle(`role/${role}`)}
                    >
                        <StyledText noLinks={true}>{translate("role."+role+".name")}</StyledText>
                    </button>
                })}
            </>
        })}
        <WikiStyledText key={"extra"}>
            {translate("wiki.article.generated.roleSet.extra", Object.keys(roleJsonData()).length)}
        </WikiStyledText>
    </div>;
}

function getSearchStringsGenerated(article: GeneratedArticle): string[]{
    switch(article){
        case "roleSet":
            let out = [translate("wiki.article.generated.roleSet.title")];
            for(let set of ROLE_SETS){
                out.push(translate(set));
            }
            return out;
        case "all_text":
            return [];
    }
}

export function getSearchStrings(article: WikiArticleLink): string[]{
    const path = article.split('/');

    switch (path[0]) {
        case "role":

            const role = path[1] as Role;
            const roleData = roleJsonData()[role];
            let out = [];

            out.push(translate("role."+role+".name"));

            for(let roleSet of roleData.roleSets){
                out.push(translate(roleSet));
            }

            let guide = translateChecked("wiki.article.role."+role+".guide");
            if(guide)
                out.push(guide);
            if(roleData.armor){
                out.push(translate("defense.1"));
                out.push(translate("defense"));
            }
            let abilities = translateChecked("wiki.article.role."+role+".abilities");
            if(abilities)
                out.push(abilities);
            let attributes = translateChecked("wiki.article.role."+role+".attributes");
            if(attributes)
                out.push(attributes);
            let extra = translateChecked("wiki.article.role."+role+".extra");
            if(extra)
                out.push(extra);
            let roleLimit = roleData.maxCount !== null;
            if(roleLimit)
                out.push(translate("wiki.article.standard.roleLimit.title"));

            return out;            
            
        case "standard":
            return [
                translate(`wiki.article.standard.${path[1]}.title`),
                translate(`wiki.article.standard.${path[1]}.text`),
            ]
        case "generated":
            return getSearchStringsGenerated(path[1] as GeneratedArticle);
        default:
            return [];
    }
}