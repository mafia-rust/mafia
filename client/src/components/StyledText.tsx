import DOMPurify from "dompurify";
import { marked } from "marked";
import React, { ReactElement, useEffect, useState } from "react";
import ReactDOMServer from "react-dom/server";
import { Player } from "../game/gameState.d";
import GAME_MANAGER, { find } from "..";
import translate from "../game/lang";
import { Role, getFactionFromRole } from "../game/roleState.d";
import ROLES from "../resources/roles.json";
import "./styledText.css";
import WikiSearch, { WikiPage } from "./WikiSearch";

type TextData = { 
    styleClass?: string, 
    link?: WikiPage
};
type TextDataMap = { [key: string]: TextData };

const SANITIZATION_OPTIONS = {
    FORBID_TAGS: ['a', 'img']
}

const MARKDOWN_OPTIONS = {
    breaks: true,
    mangle: false,
    headerIds: false,
    gfm: true
}

export default function StyledText(props: { children: string[] | string, className?: string, noLinks?: boolean }): ReactElement {
    const KEYWORD_DATA_MAP: TextDataMap = useKeywordData();

    type Token = {
        type: "raw"
        string: string 
    } | ({
        type: "data"
        string: string
    } & TextData)

    let tokens: Token[] = [{
        type: "raw",
        string: marked.parse(
            typeof props.children === "string" 
                ? props.children 
                : props.children.join(""), 
            MARKDOWN_OPTIONS
        )
    }];

    GAME_MANAGER.gameState.players.forEach((player, index) => {
        tokens[0].string = tokens[0].string.replace(
            find(player.toString()), 
            `<span class="keyword-player-number">${index + 1}</span> ${player.name}`
        );
    })

    for(const [keyword, data] of Object.entries(KEYWORD_DATA_MAP)) {
        for(let index = 0; index < tokens.length; index++) {
            const token = tokens[index];
            if (token.type !== "raw") continue;
            
            // Remove the keyword and split so we can insert the styled text in its place
            const stringSplit = token.string.split(RegExp('('+find(keyword).source+')', 'gi'));

            if (stringSplit.length === 1) continue;

            // Insert the styled string into where we just removed the unstyled string from
            let replacement: Token[] = []; 
            for(const string of stringSplit){
                if (find(keyword).test(string)) {
                    replacement.push({
                        type: "data",
                        string,
                        ...data
                    });
                } else if(string !== "") {
                    replacement.push({
                        type: "raw",
                        string: string
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

    const jsxString = tokens.map(token => {
        if (token.type === "raw") {
            return token.string;
        } else if (token.link === undefined || props.noLinks) {
            return ReactDOMServer.renderToStaticMarkup(
                <span
                    className={token.styleClass}
                    dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(
                        token.string, 
                        SANITIZATION_OPTIONS
                    )}}
                />
            );
        } else {
            // TODO: This is absolutely terrible. Don't do this.
            (window as any).setWikiSearchPage = WikiSearch.setPage;

            return ReactDOMServer.renderToStaticMarkup(
                // eslint-disable-next-line jsx-a11y/anchor-is-valid
                <a
                    href={`javascript: window.setWikiSearchPage("${token.link}")`}
                    className={token.styleClass + " keyword-link"}
                    dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(
                        token.string, 
                        SANITIZATION_OPTIONS
                    ) }}
                />
            );
        }
    }).join("");
    
    return <span
        className={props.className}
        dangerouslySetInnerHTML={{__html: jsxString}}>
    </span>
}

function useKeywordData(): TextDataMap {
    let keywordData: TextDataMap = {};

    let [players, setPlayers] = useState<Player[]>(GAME_MANAGER.gameState.players);
    useEffect(() => {
        const listener = () => setPlayers(GAME_MANAGER.gameState.players);

        GAME_MANAGER.addStateListener(listener);
        return () => GAME_MANAGER.removeStateListener(listener);
    }, [setPlayers]);

    for(const player of players){
        keywordData[player.toString()] = { styleClass: "keyword-player" };
    }

    const STYLES = require("../resources/styling/keywords.json");
    const LINKS = require("../resources/links/keywords.json")

    for(const role of Object.keys(ROLES)){
        const faction = "faction." + getFactionFromRole(role as Role);
        if (!STYLES[faction]) {
            console.error(`${STYLES[faction]} faction is missing a keyword style!`);
            continue;
        }
        keywordData[translate(`role.${role}.name`)] = {
            styleClass: STYLES[faction],
            link: `role/${role}` as WikiPage
        };
    }

    for (const keyword of Object.keys(STYLES).concat(Object.keys(LINKS))) {
        keywordData[translate(keyword)] = {};
    }

    for (const [keyword, styleClass] of Object.entries(STYLES)) {
        keywordData[translate(keyword)].styleClass = styleClass as string;
    }

    for (const [keyword, link] of Object.entries(LINKS)) {
        keywordData[translate(keyword)].link = link as WikiPage;
    }

    return keywordData;
}
