import { marked } from "marked";
import React, { ReactElement } from "react";
import ReactDOMServer from "react-dom/server";
import GAME_MANAGER, { find } from "..";
import translate from "../game/lang";
import { Role, getFactionFromRole } from "../game/roleState.d";
import ROLES from "../resources/roles.json";
import "./styledText.css";
import WikiSearch from "./WikiSearch";
import { WikiArticleLink } from "./WikiArticle";

export type TokenData = {
    style?: string, 
    link?: WikiArticleLink,
    replacement?: string
};
type KeywordData = TokenData[];
type KeywordDataMap = { [key: string]: KeywordData };

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

/**
 * Styled Text
 * 
 * ***MAKE SURE TO SANITIZE TEXT INPUT INTO THIS ELEMENT*** (If it's from the user)
 * 
 * @see sanitizePlayerMessage in ChatMessage.tsx
 */
export default function StyledText(props: { children: string[] | string, className?: string, noLinks?: boolean, markdown?: boolean }): ReactElement {
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

    tokens = styleKeywords(tokens);

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
            // TODO: This is absolutely terrible. Don't do this.
            (window as any).setWikiSearchPage = WikiSearch.setPage;

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

const KEYWORD_DATA_MAP: KeywordDataMap = {};

function clearKeywordData() {
    for (const key in KEYWORD_DATA_MAP) {
        delete KEYWORD_DATA_MAP[key];
    }
}

function computeBasicKeywordData() {
    console.log("recomputed keyword data");
    const DATA = require("../resources/keywords.json");

    for(const role of Object.keys(ROLES)){
        const data = DATA[getFactionFromRole(role as Role)];
        if (data === undefined || Array.isArray(data)) {
            console.error(`faction.${getFactionFromRole(role as Role)} has malformed keyword data!`);
            continue;
        }
        KEYWORD_DATA_MAP[translate(`role.${role}.name`)] = [{
            ...data,
            link: `role/${role}` as WikiArticleLink,
            replacement: translate(`role.${role}.name`)   // Capitalize roles
        }]
    }

    for (const [keyword, data] of Object.entries(DATA)) {
        KEYWORD_DATA_MAP[translate(keyword)] = (Array.isArray(data) ? data : [data]).map(data => {
            return {
                ...data,
                replacement: data.replacement === undefined ? undefined : translate(data.replacement)
            }
        });
    }
}

export function computeKeywordDataWithPlayers() {
    clearKeywordData();
    computeBasicKeywordData();

    if(GAME_MANAGER.state.stateType === "game")
        for(const player of GAME_MANAGER.state.players) {
            KEYWORD_DATA_MAP["sender-"+player.toString()] = [
                { style: "keyword-player-number", replacement: (player.index + 1).toString() },
                { replacement: " " },
                { style: "keyword-player-sender", replacement: player.name }
            ];
            KEYWORD_DATA_MAP[player.toString()] = [
                { style: "keyword-player-number", replacement: (player.index + 1).toString() },
                { replacement: " " },
                { style: "keyword-player", replacement: player.name }
            ];
            
        }
}

computeBasicKeywordData();

function styleKeywords(tokens: Token[]): Token[] {
    for(const [keyword, data] of Object.entries(KEYWORD_DATA_MAP)) {
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