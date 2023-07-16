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

type StyleMap = { [key: string]: string };

const SANITIZATION_OPTIONS = {
    FORBID_TAGS: ['a', 'img']
}

const MARKDOWN_OPTIONS = {
    breaks: true,
    mangle: false,
    headerIds: false,
    gfm: true
}

export default function StyledText(props: { children: string[] | string, className?: string }): ReactElement {
    const KEYWORD_STYLE_MAP: StyleMap = useKeywordStyles();

    type Token = {
        type: "string"
        string: string 
    } | {
        type: "styled"
        string: string
        className: string
    }

    let tokens: Token[] = [{
        type: "string",
        string: marked.parse(
            typeof props.children === "string" 
                ? props.children 
                : props.children.join(""), 
            MARKDOWN_OPTIONS
        )
    }];

    for(const [keyword, style] of Object.entries(KEYWORD_STYLE_MAP)){
        // Using for..of or for..in is prone to errors, since we mutate the array as we loop through it,
        // which is why I've opted for a classical for loop to ensure completeness.
        for(let index = 0; index < tokens.length; index++) {
            const token = tokens[index];
            if (token.type !== "string") continue;
            
            // Remove the keyword and split so we can insert the styled text in its place
            const stringSplit = token.string.split(find(keyword));

            if (stringSplit.length === 1) continue;

            // Insert the styled string into where we just removed the unstyled string from
            let replacement: Token[] = []; 
            for(const string of stringSplit){
                if(string !== "")
                    replacement.push({
                        type: "string",
                        string: string
                    });

                replacement.push({
                    type: "styled",
                    string: keyword,
                    className: style
                });
            }
            replacement.pop();

            // Insert the new tokens in the place of the old one
            tokens = 
                tokens.slice(0, index)
                    .concat(replacement)
                    .concat(tokens.slice(index+1));
            
            // Skip elements we've already checked
            index += replacement.length - 1;
        }
    }

    // Convert to JSX
    const jsxString = tokens.map(token => 
        token.type === "string" 
            ? token.string 
            : ReactDOMServer.renderToStaticMarkup(
                <span className={token.className} 
                    dangerouslySetInnerHTML={{ __html: token.string }}
                />
            )
    ).join("");
    
    return <span
        className={props.className}
        // Sanitization isn't strictly necessary here, but it's good to be safe.
        dangerouslySetInnerHTML={{__html: DOMPurify.sanitize(
            jsxString, 
            SANITIZATION_OPTIONS
        )}}>
    </span>
}

function useKeywordStyles(): StyleMap {
    let stringsToStyle: StyleMap = {};

    let [players, setPlayers] = useState<Player[]>(GAME_MANAGER.gameState.players);
    useEffect(() => {
        const listener = () => setPlayers(GAME_MANAGER.gameState.players);

        GAME_MANAGER.addStateListener(listener);
        return () => GAME_MANAGER.removeStateListener(listener);
    }, [setPlayers]);

    for(const player of players){
        stringsToStyle[player.toString()] = "keyword-player";
    }

    const STYLES = require("../resources/styling/keywords.json");

    // Automatically color roles based on faction
    for(const role of Object.keys(ROLES)){
        stringsToStyle[translate("role." + role + ".name")] = STYLES["faction." + getFactionFromRole(role as Role)];
    }

    for (const [key, value] of Object.entries(STYLES)) {
        stringsToStyle[translate(key)] = value as string;
    }

    return stringsToStyle;
}
