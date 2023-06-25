import React from "react";
import GAME_MANAGER from "../index";
import ROLES from "../resources/roles.json";
import { ChatMessage } from "./chatMessage";
import { Role, getFactionFromRole } from "./roleState.d";
import { marked } from "marked";
import DOMPurify from 'dompurify';

let lang: ReadonlyMap<string, string>;
switchLanguage("en_us");

export function switchLanguage(language: string) {
    let json = require("../resources/lang/" + language + ".json");
    lang = new Map<string, string>(Object.entries(json))
}

export default function translate(langKey: string, ...valuesList: (string | number)[]): string {
    let out = lang.get(langKey);
    if(out===undefined){
        console.error("Attempted to use non existant lang key: "+langKey);
        return "ERROR: "+langKey;
    }
    for(let i = 0; i < valuesList.length; i++){
        out = out.replace("\\"+(i), valuesList[i] as string);
    }
    return out;
}

export function getChatElement(message: ChatMessage, key: number): JSX.Element {
    const SPECIAL = { text: { color: "violet" } };
    const DISCREET = { text: { color: "turquoise" } };
    const IMPORTANT = { text: { color: "yellow" } };
    const TRIAL = { text: { color: "orange" } };
    const TARGET = { text: { color: "orange" } };

    
    const RESULT_STYLE = { text: { color: "green" } };
    const WARNING_STYLE = { box: { backgroundColor: "#660000" } };

    switch (message.type) {
        case "normal":
            if(message.messageSender.type === "player"){
                let playerIndex = message.messageSender.player;
                if(message.chatGroup !== "dead"){
                    return createChatElement(key, translate("chatmessage.normal",
                        GAME_MANAGER.gameState.players[playerIndex].toString(), 
                        message.text
                    ), { indent: true });
                }else{
                    return createChatElement(key, translate("chatmessage.normal",
                        GAME_MANAGER.gameState.players[playerIndex].toString(),
                        message.text
                    ), {
                        box: { backgroundColor: "black", borderRadius: "5px" },
                        text: { color: "grey" },
                        indent: true
                    });
                }
            } else {
                //TODO, this only works because jailor and medium are the only options
                return createChatElement(key, translate("chatmessage.normal",
                    translate("role."+message.messageSender.type+".name"),
                    message.text
                ), {...DISCREET, indent: true});
            }
        case "whisper":
            return createChatElement(key, translate("chatmessage.whisper", 
                GAME_MANAGER.gameState.players[message.fromPlayerIndex].toString(),
                GAME_MANAGER.gameState.players[message.toPlayerIndex].toString(),
                message.text
            ), {...DISCREET, indent: true});
        case "broadcastWhisper":
            return createChatElement(key, translate("chatmessage.broadcastWhisper",
                GAME_MANAGER.gameState.players[message.whisperer].toString(),
                GAME_MANAGER.gameState.players[message.whisperee].toString(),
            ), DISCREET);
        case "roleAssignment":
            return createChatElement(key, translate("chatmessage.roleAssignment", 
                translate("role." + message.role + ".name")
            ), { 
                box: { textAlign: "center" }, 
                ...IMPORTANT
            });
        case "playerDied":
            //TODO, role doesnt work properly
            let graveRoleString: string;
            if (message.grave.role.type === "role") {
                graveRoleString = translate(`role.${message.grave.role.role}.name`);
            } else {
                graveRoleString = translate(`grave.role.${message.grave.role.type}`);
            }

            let deathCause: string;
            if (message.grave.deathCause.type === "lynching") {
                deathCause = translate("grave.deathCause.lynching")
            } else {
                let killers: string[] = [];
                for (let killer of message.grave.deathCause.killers) {
                    if(killer.type === "role") {
                        killers.push(translate(`role.${killer.value}.name`))
                    }else if(killer.type === "faction") {
                        killers.push(translate(`faction.${killer.value}`))
                    }else{
                        killers.push(translate(`grave.killer.${killer.type}`))
                    }
                }
                deathCause = killers.join();
            }

            return createChatElement(key, translate("chatmessage.playerDied",
                GAME_MANAGER.gameState.players[message.grave.playerIndex].toString(),
                graveRoleString,
                deathCause,
                message.grave.will
            ), IMPORTANT);
        case "youDied":
            return createChatElement(key, translate("chatmessage.youDied"), SPECIAL);
        case "phaseChange":
            return createChatElement(key, translate("chatmessage.phaseChange",
                translate("phase."+message.phase),
                message.dayNumber
            ), {
                box: { textAlign: "center", backgroundColor: "var(--primary-color)" },
                text: { color: "yellow", textDecoration: "underline" }
            });
        case "trialInformation":
            return createChatElement(key, translate("chatmessage.trialInformation",
                message.requiredVotes,
                message.trialsLeft
            ), TRIAL);
        case "voted":
            if (message.votee !== null) {
                return createChatElement(key, translate("chatmessage.voted",
                    GAME_MANAGER.gameState.players[message.voter].toString(),
                    GAME_MANAGER.gameState.players[message.votee].toString(),
                ), TRIAL);
            } else {
                return createChatElement(key, translate("chatmessage.voted.cleared",
                    GAME_MANAGER.gameState.players[message.voter].toString(),
                ), TRIAL);
            }
        case "playerOnTrial":
            return createChatElement(key, translate("chatmessage.playerOnTrial",
                GAME_MANAGER.gameState.players[message.playerIndex].toString(),
            ), IMPORTANT);
        case "judgementVote":
            return createChatElement(key, translate("chatmessage.judgementVote",
                GAME_MANAGER.gameState.players[message.voterPlayerIndex].toString(),
            ), TRIAL);
        case "judgementVerdict":
            return createChatElement(key, translate("chatmessage.judgementVerdict",
                GAME_MANAGER.gameState.players[message.voterPlayerIndex].toString(),
                translate("verdict."+message.verdict.toLowerCase())
            ), TRIAL);
        case "trialVerdict":
            return createChatElement(key, translate("chatmessage.trialVerdict",
                GAME_MANAGER.gameState.players[message.playerOnTrial].toString(),
                message.innocent>=message.guilty?translate("verdict.innocent"):translate("verdict.guilty"),
                message.innocent,
                message.guilty
            ), IMPORTANT);
        case "targeted":
            if (message.targets.length > 0) {
                return createChatElement(key, translate("chatmessage.targeted",
                    GAME_MANAGER.gameState.players[message.targeter].toString(),
                    message.targets.map((target) => GAME_MANAGER.gameState.players[target].toString()).join(", ")
                ), TARGET);
            } else {
                return createChatElement(key, translate("chatmessage.targeted.cleared",
                    GAME_MANAGER.gameState.players[message.targeter].toString(),
                ), TARGET);
            }
        case "mayorRevealed":
            return createChatElement(key, translate("chatmessage.mayorRevealed",
                GAME_MANAGER.gameState.players[message.playerIndex].toString(),
            ), SPECIAL);
        case "jailedTarget":
            return createChatElement(key, translate("chatmessage.jailedTarget",
                GAME_MANAGER.gameState.players[message.playerIndex].toString(),
            ), SPECIAL);
        case "jailedSomeone":
            return createChatElement(key, translate("chatmessage.jailedSomeone",
                GAME_MANAGER.gameState.players[message.playerIndex].toString()
            ), SPECIAL);
        case "deputyKilled":
            return createChatElement(key, translate("chatmessage.deputyKilled",
                GAME_MANAGER.gameState.players[message.shotIndex].toString()
            ), SPECIAL);
        case "jailorDecideExecute":
            if (message.targets.length > 0) {
                return createChatElement(key, translate("chatmessage.jailorDecideExecute",
                    message.targets.map((target) => GAME_MANAGER.gameState.players[target].toString()).join(", ")
                ), SPECIAL);
            } else {
                return createChatElement(key, translate("chatmessage.jailorDecideExecute.nobody"), SPECIAL);
            }
            ////////////////////// NIGHT
        case "roleBlocked":
            return createChatElement(key, 
                translate("chatmessage.roleBlocked" + (message.immune ? ".immune" : "")),
                RESULT_STYLE
            );
        case "sheriffResult":
            return createChatElement(key, 
                translate("chatmessage.sheriffResult." + (message.suspicious ? "suspicious" : "innocent")), 
                RESULT_STYLE
            );
        case "lookoutResult":
            if (message.players.length === 0) {
                return createChatElement(key, translate("chatmessage.lookoutResult.nobody"), RESULT_STYLE)
            } else {
                return createChatElement(key, translate("chatmessage.lookoutResult", 
                    message.players.map(playerIndex => 
                        GAME_MANAGER.gameState.players[playerIndex].toString()
                    ).join(", ")
                ), RESULT_STYLE);
            }
        case "spyMafiaVisit":
            if (message.players.length === 0) {
                return createChatElement(key, translate("chatmessage.spyMafiaVisit.nobody"), RESULT_STYLE)
            } else {
                return createChatElement(key, translate("chatmessage.spyMafiaVisit", 
                    message.players.map(playerIndex => 
                        GAME_MANAGER.gameState.players[playerIndex].toString()
                    ).join(", ")
                ), RESULT_STYLE);
            }
        case "spyCovenVisit":
            if (message.players.length === 0) {
                return createChatElement(key, translate("chatmessage.spyCovenVisit.nobody"), RESULT_STYLE)
            } else {
                return createChatElement(key, translate("chatmessage.spyCovenVisit", 
                    message.players.map(playerIndex => 
                        GAME_MANAGER.gameState.players[playerIndex].toString()
                    ).join(", ")
                ), RESULT_STYLE);
            }
        case "spyBug":
            return createChatElement(key, translate("chatmessage.spyBug."+message.bug), RESULT_STYLE)
        case "trackerResult":
            if (message.players.length === 0) {
                return createChatElement(key, translate("chatmessage.trackerResult.nobody"), RESULT_STYLE)
            } else {
                return createChatElement(key, translate("chatmessage.trackerResult", 
                    message.players.map(playerIndex => 
                        GAME_MANAGER.gameState.players[playerIndex].toString()
                    ).join(", ")
                ), RESULT_STYLE);
            }
        case "seerResult":
            return createChatElement(key, 
                translate("chatmessage.seerResult." + (message.enemies ? "enemies" : "friends")),
                RESULT_STYLE
            );
        case "retributionistBug":
            return <>{[
                createChatElement(0, 
                    translate("chatmessage.retributionistBug"),
                    RESULT_STYLE
                ), 
                getChatElement(message.message, 1)
            ]}</>;
        case "playerRoleAndWill":
            return createChatElement(key, translate("chatmessage.playersRoleAndWill", 
                translate("role."+message.role+".name"), 
                message.will
            ), RESULT_STYLE);
        case "consigliereResult":
            return createChatElement(key, translate("chatmessage.consigliereResult", 
                translate("role."+message.role+".name"),
                (message.visited.map((playerIndex) => GAME_MANAGER.gameState.players[playerIndex].toString()).join(", ")), 
                (message.visitedBy.map((playerIndex) => GAME_MANAGER.gameState.players[playerIndex].toString()).join(", "))
            ), RESULT_STYLE);
        /* Messages that players must not miss */
        case "silenced":
            return createChatElement(key, translate("chatmessage.silenced"), WARNING_STYLE);
        case "mediumSeance":
            return createChatElement(key, translate("chatmessage.mediumSeance", GAME_MANAGER.gameState.players[message.player].toString()), SPECIAL);
        case "arsonistCleanedSelf":
        case "arsonistDousedPlayers":
        case "doctorHealed":
        case "bodyguardProtected":
        case "protectedYou":
        case "executionerWon":
        case "framerFramedPlayers":
        case "gameOver":
        case "godfatherForcedMafioso":
        case "godfatherForcedYou":
        case "jesterWon":
        case "mayorCantWhisper":
        case "playerWithNecronomicon":
        case "roleData":
        case "targetJailed":
        case "targetSurvivedAttack":
        case "transported":
        case "veteranAttackedVisitor":
        case "veteranAttackedYou":
        case "vigilanteSuicide":
        case "witchBug":
        case "witchTargetImmune":
        case "witchedYou":
        case "youSurvivedAttack":
            return createChatElement(key, translate("chatmessage."+message.type), RESULT_STYLE);
        default:
            console.error("Unknown message type " + (message as any).type + ":");
            console.error(message);
            return createChatElement(key, "FIXME: " + translate("chatmessage." + message), {
                box: { borderStyle: "thick", borderColor: "purple" },
                ...SPECIAL
            });
    }
}

function createChatElement(
    key: number,
    text: string,
    style: {
        box?: React.CSSProperties,
        text?: React.CSSProperties,
        indent?: boolean
    } = {},
): JSX.Element {
    return styleText(text, {
        boxStyle: style.box,
        defaultStyle: style.text,
        indent: style.indent
    }, key)
}

const BUILTIN_STYLES: {[key: string]: React.CSSProperties} = {
    EVIL: { color: "red" },
    GOOD: { color: "lime" },
    MAGIC: { color: "magenta" },
    NEUTRAL: { color: "orange" },
    INFO: { color: "lightblue" },
    HIDDEN: { color: "whitesmoke", fontStyle: "italic", fontWeight: "bold" },
}

type StyleMap = { [key: string]: React.CSSProperties };

const SANITIZATION_OPTIONS = {
    ALLOWED_TAGS: ['br', 'span', 'li', 'ol', 'ul', 'p', 'strong', 'em', 'del'],
}

const MARKDOWN_OPTIONS = {
    breaks: true,
    mangle: false,
    headerIds: false,
    gfm: true
}

marked.use(MARKDOWN_OPTIONS);

export function styleText(
    string: string, 
    styleOverride: {
        boxStyle?: React.CSSProperties,
        defaultStyle?: React.CSSProperties
        indent?: boolean
    } = {},
    key?: number
): JSX.Element {
    const KEYWORD_STYLE_MAP: StyleMap = getKeywordStyles();

    const boxStyle = styleOverride.boxStyle ?? {};
    const defaultStyle = styleOverride.defaultStyle ?? {};

    type Token = {
        type: "string"
        string: string 
    } | {
        type: "styled"
        string: string
        style: React.CSSProperties
    }

    const startString = DOMPurify.sanitize(marked.parseInline(string), SANITIZATION_OPTIONS);

    let tokens: Token[] = [{
        type: "string",
        string: styleOverride.indent ? startString.replace(/<br>/g, "<br>      ") : startString
    }];

    for(const [stringToStyle, style] of Object.entries(KEYWORD_STYLE_MAP)){
        // Using for..of or for..in is prone to errors, since we mutate the array as we loop through it,
        // which is why I've opted for a classical for loop to ensure completeness.
        for(let index = 0; index < tokens.length; index++) {
            const token = tokens[index];
            if (token.type !== "string") continue;

            const regEscape = (v: string) => v.replace(/[-[\]{}()*+?.,\\^$|#\s]/g, '\\$&');

            // Remove the stringToStyle and split so we can insert the styled text in its place
            const stringSplit = token.string.split(RegExp(`\\b${regEscape(stringToStyle)}\\b`, "gi"));

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
                    string: stringToStyle,
                    style: style
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
    let jsx = [];
    for(const [index, token] of tokens.entries()){
        jsx.push(<span key={index} 
            style={token.type === "styled" ? token.style : defaultStyle} 
            dangerouslySetInnerHTML={{ __html: token.string }}
        />);
    }
    
    return <pre key={key} style={{...boxStyle, whiteSpace: "pre-wrap"}}>{jsx}</pre>
}

// TODO: Memoize this - shouldn't need to change after a game has begun.
function getKeywordStyles(): StyleMap {
    let stringsToStyle: StyleMap = {};

    for(let player of GAME_MANAGER.gameState.players){
        stringsToStyle[player.toString()] = { color: "whitesmoke", fontStyle: "italic", fontWeight: "bold" };
    }

    const STYLES = require("../resources/lang/style.json");

    // Automatically color roles based on faction
    for(let role of Object.keys(ROLES)){
        stringsToStyle[translate("role." + role + ".name")] = getStyleFromValue(STYLES["faction." + getFactionFromRole(role as Role)]);
    }

    for (let [key, value] of Object.entries(STYLES)) {
        stringsToStyle[translate(key)] = getStyleFromValue(value);
    }

    return stringsToStyle;
}

function getStyleFromValue(value: any): React.CSSProperties {
    // Use sparingly!
    const gradient = (colors: string): React.CSSProperties => {
        return {
            backgroundImage: `linear-gradient(to left, ${colors})`,
            backgroundClip: "text",
            color: "rgba(255,255,255,.2)",
            WebkitBackgroundClip: "text",
            WebkitTextFillColor: "rgba(255,255,255,.2)"
        }
    };

    if (typeof value == "object") {
        return value as React.CSSProperties;
    } else if (typeof value == "string") {
        if (value.startsWith("GRADIENT::")) {
            return gradient(value.substring(10));
        } else if (Object.keys(BUILTIN_STYLES).includes(value)) {
            return BUILTIN_STYLES[value];
        }
    }

    console.log(`Found invalid style: ${value}`);
    // Mark as an error
    return gradient("magenta, black, white, magenta, black, white, magenta, black, white");
}