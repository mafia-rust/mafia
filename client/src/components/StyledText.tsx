import { marked } from "marked";
import React, { ReactElement, useContext, useEffect } from "react";
import ReactDOMServer from "react-dom/server";
import { find } from "..";
import translate, { translateChecked } from "../game/lang";
import { Role, getMainRoleSetFromRole, roleJsonData } from "../game/roleState.d";
import "./styledText.css";
import DUMMY_NAMES from "../resources/dummyNames.json";
import { ARTICLES, WikiArticleLink, getArticleLangKey } from "./WikiArticleLink";
import { setWikiSearchPage } from "./Wiki";
import { getRoleSetsFromRole } from "../game/roleListState.d";
import { AnchorContext } from "../menu/AnchorContext";
import { GameScreenMenuContext } from "../menu/game/GameScreenMenuContext";
import { Player } from "../menu/game/GameStateContext";

export type TokenData = {
    style?: string, 
    link?: WikiArticleLink,
    replacement?: string
};
type KeywordData = TokenData[];
export type KeywordDataMap = { [key: string]: KeywordData };

const MARKDOWN_OPTIONS = {
    breaks: true,
    mangle: false,
    headerIds: false,
    gfm: true
}

type Token = {
    type: "raw"
    string: string 
} | ({
    type: "data"
    string: string
} & KeywordData[number])

export type StyledTextProps = {
    children: string[] | string,
    className?: string,
    noLinks?: boolean,
    markdown?: boolean,
    playerKeywordData?: KeywordDataMap
};

/**
 * Styled Text
 * 
 * ***MAKE SURE TO SANITIZE TEXT INPUT INTO THIS ELEMENT*** (If it's from the user)
 * 
 * @param props.playerKeywordData  If omitted, defaults to {@link PLAYER_KEYWORD_DATA} 
 * @see sanitizePlayerMessage in ChatMessage.tsx
 */
export default function StyledText(props: Readonly<StyledTextProps>): ReactElement {
    const playerKeywordData = props.playerKeywordData ?? PLAYER_KEYWORD_DATA;
    const menuController = useContext(GameScreenMenuContext);
    const anchorController = useContext(AnchorContext)!;

    useEffect(() => {
        (window as any).setWikiSearchPage = (page: WikiArticleLink) => {
            setWikiSearchPage(page, anchorController, menuController)
        };
    })

    let tokens: Token[] = [{
        type: "raw",
        string: typeof props.children === "string" 
                ? props.children 
                : props.children.join("")
    }];

    if (props.markdown) {
        tokens[0].string = marked.parse(tokens[0].string, MARKDOWN_OPTIONS);
    } else {
        tokens[0].string = tokens[0].string.replace(/\n/g, '<br>');
    }

    tokens = styleKeywords(tokens, playerKeywordData);

    const jsxString = tokens.map(token => {
        if (token.type === "raw") {
            return token.string;
        } else if (token.link === undefined || props.noLinks) {
            return ReactDOMServer.renderToStaticMarkup(
                <span
                    className={token.style}
                    dangerouslySetInnerHTML={{ __html: token.string }}
                />
            );
        } else {
            return ReactDOMServer.renderToStaticMarkup(
                // eslint-disable-next-line jsx-a11y/anchor-is-valid
                <a
                    href={`javascript: window.setWikiSearchPage("${token.link}")`}
                    className={token.style + " keyword-link"}
                    dangerouslySetInnerHTML={{ __html: token.string }}
                />
            );
        }
    }).join("");
    
    return <span
        className={props.className}
        dangerouslySetInnerHTML={{__html: jsxString}}>
    </span>
}

const KEYWORD_DATA: KeywordDataMap = {};
computeKeywordData();

export function computeKeywordData() {
    for (const key in KEYWORD_DATA) {
        delete KEYWORD_DATA[key];
    }

    function addTranslatableKeywordData(langKey: string, data: KeywordData) {
        KEYWORD_DATA[translate(langKey)] = data;
        for (let i = 0, variant; (variant = translateChecked(`${langKey}:var.${i}`)) !== null; i++) {
            const variantData = data.map(datum => ({
                ...datum,
                replacement: datum.replacement === translate(langKey) ? translate(`${langKey}:var.${i}`) : datum.replacement
            }));
            KEYWORD_DATA[variant] = variantData;
        }
    }

    //add article keywords
    const SortedArticles = [...ARTICLES];
    for (const article of SortedArticles) {
        const keySplit = article.split("/");
        const key = getArticleLangKey(article);

        addTranslatableKeywordData(key, [{
            style: "keyword-info",
            link: `${keySplit[0]}/${keySplit[1]}` as WikiArticleLink,
        }]);
    }

    const KEYWORD_DATA_JSON = require("../resources/keywords.json");
    //add role keywords
    for(const role of Object.keys(roleJsonData())) {

        let data: KeywordData | undefined = undefined;

        const roleSets = getRoleSetsFromRole(role as Role);
        if (roleSets.length === 1) {
            data = KEYWORD_DATA_JSON[roleSets[0]];
        }else if (data === undefined) {
            data = KEYWORD_DATA_JSON[getMainRoleSetFromRole(role as Role)];
        }

        if (data === undefined || Array.isArray(data)) {
            console.error(`faction.${getMainRoleSetFromRole(role as Role)} has malformed keyword data!`);
            continue;
        }

        addTranslatableKeywordData(`role.${role}.name`, [{
            ...(data as KeywordData),
            link: `role/${role}` as WikiArticleLink,
            replacement: translate(`role.${role}.name`)   // Capitalize roles
        }]);
    }
    
    //add from keywords.json
    for (const [keyword, data] of Object.entries(KEYWORD_DATA_JSON)) {
        addTranslatableKeywordData(keyword, (Array.isArray(data) ? data : [data]).map(data => {
            return {
                ...data,
                replacement: data.replacement === undefined ? undefined : translate(data.replacement)
            }
        }));
    }
}

export const PLAYER_SENDER_KEYWORD_DATA: KeywordDataMap = {};
export const PLAYER_KEYWORD_DATA: KeywordDataMap = {};

export function computePlayerKeywordData(players: Player[]) {
    for (const key in PLAYER_KEYWORD_DATA) {
        delete PLAYER_KEYWORD_DATA[key];
    }
    for (const key in PLAYER_SENDER_KEYWORD_DATA) {
        delete PLAYER_SENDER_KEYWORD_DATA[key];
    }

    for(const player of players) {
        PLAYER_SENDER_KEYWORD_DATA[player.toString()] = [
            { style: "keyword-player-number", replacement: (player.index + 1).toString() },
            { replacement: " " },
            { style: "keyword-player-sender", replacement: player.name }
        ];
        
        PLAYER_KEYWORD_DATA[player.toString()] = [
            { style: "keyword-player-number", replacement: (player.index + 1).toString() },
            { replacement: " " },
            { style: "keyword-player", replacement: player.name }
        ];
        
    }
}

export function computePlayerKeywordDataForLobby(playerNames: string[]) {
    for (const key in PLAYER_KEYWORD_DATA) {
        delete PLAYER_KEYWORD_DATA[key];
    }
    for (const key in PLAYER_SENDER_KEYWORD_DATA) {
        delete PLAYER_SENDER_KEYWORD_DATA[key];
    }

    for(const name of playerNames) {
        PLAYER_SENDER_KEYWORD_DATA[name] = [{ style: "keyword-player-sender", replacement: name }];
        PLAYER_KEYWORD_DATA[name] = [{ style: "keyword-player", replacement: name }];
    }
}

export const DUMMY_NAMES_SENDER_KEYWORD_DATA: KeywordDataMap = {};
export const DUMMY_NAMES_KEYWORD_DATA: KeywordDataMap = {};
computeDummyNamesKeywordData();

function computeDummyNamesKeywordData() {
    for (const key in DUMMY_NAMES_KEYWORD_DATA) {
        delete DUMMY_NAMES_KEYWORD_DATA[key];
    }
    for(let i = 0; i < DUMMY_NAMES.length; i++) {
        const name = DUMMY_NAMES[i];
        DUMMY_NAMES_SENDER_KEYWORD_DATA[name] = [
            { style: "keyword-player-number", replacement: (i + 1).toString() },
            { replacement: " " },
            { style: "keyword-player-sender", replacement: name }
        ];
        DUMMY_NAMES_KEYWORD_DATA[name] = [
            { style: "keyword-player-number", replacement: (i + 1).toString() },
            { replacement: " " },
            { style: "keyword-player", replacement: name }
        ];
    }
}

function styleKeywords(tokens: Token[], extraData?: KeywordDataMap): Token[] {
    const keywordDataMap = { ...KEYWORD_DATA, ...extraData };

    for(const [keyword, data] of Object.entries(keywordDataMap).sort((a, b) => b[0].length - a[0].length)){
        for(let index = 0; index < tokens.length; index++) {
            const token = tokens[index];
            if (token.type !== "raw") continue;
            
            const stringSplit = token.string.split(RegExp('('+find(keyword).source+')', 'gi'));

            if (stringSplit.length === 1) continue;

            // Insert the styled string into where we just removed the unstyled string from
            let replacement: Token[] = []; 
            for(const string of stringSplit){
                if(string === "") continue;
                if (!find(keyword).test(string)) {
                    replacement.push({
                        type: "raw",
                        string: string
                    });
                    continue;
                }
                for (const datum of data) {
                    replacement.push({
                        type: "data",
                        string: datum.replacement ?? string,
                        ...datum
                    });
                }
            }

            tokens = 
                tokens.slice(0, index)
                    .concat(replacement)
                    .concat(tokens.slice(index+1));
            
            // Skip elements we've already checked
            index += replacement.length - 1;
        }
    }

    return tokens;
}