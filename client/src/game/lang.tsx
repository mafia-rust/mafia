import React from "react";
import GAME_MANAGER from "../index";
import ROLES from "../resources/roles.json";
import { FactionAlignment, getFactionFromFactionAlignment } from "./gameState.d";
import { ChatMessage } from "./chatMessage";

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
    const PLAYER_MESSAGE = { indent: true }; // 2rem
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
                    ), PLAYER_MESSAGE);
                }else{
                    return createChatElement(key, translate("chatmessage.normal",
                        GAME_MANAGER.gameState.players[playerIndex].toString(),
                        message.text
                    ), {
                        box: { backgroundColor: "black", borderRadius: "5px" },
                        text: { color: "grey" },
                        ...PLAYER_MESSAGE
                    });
                }
            } else {
                //TODO, this only works because jailor and medium are the only options
                return createChatElement(key, translate("chatmessage.normal",
                    translate("role."+message.messageSender.type+".name"),
                    message.text
                ), {...PLAYER_MESSAGE, ...DISCREET});
            }
        case "whisper":
            return createChatElement(key, translate("chatmessage.whisper", 
                GAME_MANAGER.gameState.players[message.fromPlayerIndex].toString(),
                GAME_MANAGER.gameState.players[message.toPlayerIndex].toString(),
                message.text
            ), {...PLAYER_MESSAGE, ...DISCREET});
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
            return createChatElement(key, translate("chatmessage.youDied"), IMPORTANT);
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
        case "deputyShot":
            return createChatElement(key, translate("chatmessage.deputyShot",
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
                return createChatElement(key, translate("chatmessage.night.lookoutResult.nobody"), RESULT_STYLE)
            } else {
                return createChatElement(key, translate("chatmessage.night.lookoutResult", 
                    message.players.map(playerIndex => 
                        GAME_MANAGER.gameState.players[playerIndex].toString()
                    ).join(", ")
                ), RESULT_STYLE);
            }
        case "seerResult":
            return createChatElement(key, 
                translate("chatmessage.seerResult." + message.enemies ? "enemies" : "friends"),
                RESULT_STYLE
            );
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
            return createChatElement(key, translate("chatmessage."+message.type), WARNING_STYLE);
        default:
            return createChatElement(key, translate("chatmessage."+message.type), RESULT_STYLE);
        // default:
        //     console.error("Unknown message type " + message.type + ":");
        //     console.error(message);
        //     return createChatElement(key, "FIXME: " + translate("chatmessage." + message), {
        //         box: { borderStyle: "thick", borderColor: "purple" },
        //         ...SPECIAL
        //     });
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
    return <span key={key} style={style.box}>{styleText(text, {
        defaultStyle: style.text,
        indentStyle: style.indent ? { marginLeft: "2rem" } : undefined
    })}</span>
}

function styleSubstrings(
    string: string, 
    stringsToStyle: {
        string: string, 
        style: React.CSSProperties,
    }[], 
    styleOverride: {
        defaultStyle?: React.CSSProperties, 
        indentStyle?: React.CSSProperties,
    } = {}
): JSX.Element[]{

    let defaultStyle = styleOverride.defaultStyle !== undefined ? styleOverride.defaultStyle : {};
    let indentStyle = styleOverride.indentStyle !== undefined ? styleOverride.indentStyle : {};

    type StyledOrNot = {
        type: "string"
        string: string 
    } | {
        type: "styled"
        string: string
        style: React.CSSProperties
    } | {
        type: "br"
    }

    let finalOutList: StyledOrNot[] = [];

    //add in br
    string.split("\n").forEach((v, i, a) => {
        finalOutList.push({type: "string", string: v});
        if(i !== a.length-1) 
            finalOutList.push({type: "br"});
    });


    for(let i in stringsToStyle){
        for(let j in finalOutList){

            let current = finalOutList[j];
            if(current === undefined){
                continue;
            }
            if(current.type !== "string"){
                continue;
            }

            
            const regEscape = (v: string) => v.replace(/[-[\]{}()*+?.,\\^$|#\s]/g, '\\$&');

            let currentStringSplit = current.string.split(RegExp("\\b" + regEscape(stringsToStyle[i].string) + "\\b", "gi"));


            let currentOutList: StyledOrNot[] = []; 

            for(let str of currentStringSplit){
                if(str !== "")
                    currentOutList.push({
                        type: "string",
                        string: str
                    });

                currentOutList.push({
                    type: "styled",
                    string: stringsToStyle[i].string, 
                    style: stringsToStyle[i].style
                });
            }
            currentOutList.pop();

            //inject outlist into finaloutlist at position j, without using splice
            finalOutList = 
                finalOutList.slice(0, Number(j))
                .concat(currentOutList)
                .concat(finalOutList.slice(Number(j)+1));
        }
    }

    //turn into jsx
    let outJsxList = [];
    let shouldIndent = false;
    for(let [i, current] of finalOutList.entries()){
        if(current.type === "br"){
            shouldIndent = true;
            outJsxList.push(<br key={i}/>);
        }else if(current.type === "string"){
            outJsxList.push(
            <span key={i} style={shouldIndent ? {...defaultStyle, ...indentStyle} : defaultStyle }>
                {current.string}
            </span>);
            shouldIndent = false;
        }else if(current.type === "styled"){
            outJsxList.push(
            <span key={i}
                style={shouldIndent ? {...current.style, ...indentStyle} : current.style }
            >
                {current.string}
            </span>);
            shouldIndent = false;
        }
    }

    return outJsxList;
}

// TODO: stringsToStyle should be calculated ONCE, statically, rather than every time this function is called.
export function styleText(
    string: string, 
    styleOverride: {
        defaultStyle?: React.CSSProperties, 
        indentStyle?: React.CSSProperties
    } = {}
): JSX.Element[]{
    let stringsToStyle: {string: string, style: React.CSSProperties}[] = [];

    // Use sparingly!
    let gradient = (colors: string): React.CSSProperties => {
        return {
            backgroundImage: `linear-gradient(to left, ${colors})`,
            backgroundClip: "text",
            color: "rgba(255,255,255,.2)",
            WebkitBackgroundClip: "text",
            WebkitTextFillColor: "rgba(255,255,255,.2)"
        }
    };

    const EVIL: React.CSSProperties = { color: "red" };
    const GOOD: React.CSSProperties = { color: "lime" };
    const MAGIC: React.CSSProperties = { color: "magenta" };
    const NEUTRAL: React.CSSProperties = { color: "orange" };
    const INFO: React.CSSProperties = { color: "lightblue" };
    const HIDDEN: React.CSSProperties = { color: "whitesmoke", fontStyle: "italic", fontWeight: "bold" };

    for(let i = 0; i < GAME_MANAGER.gameState.players.length; i++){
        stringsToStyle.push(
            {
                string: GAME_MANAGER.gameState.players[i].toString(), 
                style:HIDDEN
            }
        )
    }

    for(let role in ROLES){
        let roleObject = ROLES[role as keyof typeof ROLES];

        switch(getFactionFromFactionAlignment(roleObject.factionAlignment as FactionAlignment)){
            case "coven":
                stringsToStyle.push({string:translate("role."+role+".name"), style:MAGIC});
                break;
            case "town":
                stringsToStyle.push({string:translate("role."+role+".name"), style:GOOD});
                break;
            case "mafia":
                stringsToStyle.push({string:translate("role."+role+".name"), style:EVIL});
                break;
            case "neutral":
                stringsToStyle.push({string:translate("role."+role+".name"), style:NEUTRAL});
                break;
        }
    }

    stringsToStyle = stringsToStyle.concat([
        {string:translate("verdict.guilty"), style:EVIL},
        {string:translate("verdict.innocent"), style:GOOD},
        {string:translate("innocent.shorthand"), style:GOOD},
        {string:translate("verdict.abstain"), style:{color:"cyan"}},

        {string:translate("grave.role.cleaned"), style:HIDDEN},
        {string:translate("grave.role.petrified"), style:HIDDEN},
        {string:translate("suspicious"), style:EVIL},
        {string:translate("suspicious.shorthand"), style:EVIL},
        {string:translate("friends"), style:GOOD},
        {string:translate("enemies"), style:EVIL},

        {string:translate("faction.town"), style:GOOD},
        {string:translate("faction.mafia"), style:EVIL},
        {string:translate("faction.neutral"), style:NEUTRAL},
        {string:translate("faction.coven"), style:MAGIC},

        {string:translate("alignment.killing"), style:INFO},
        {string:translate("alignment.investigative"), style:INFO},
        {string:translate("alignment.protective"), style:INFO},
        {string:translate("alignment.support"), style:INFO},
        {string:translate("alignment.deception"), style:INFO},
        {string:translate("alignment.evil"), style:INFO},
        {string:translate("alignment.chaos"), style:INFO},
        {string:translate("alignment.utility"), style:INFO},
        {string:translate("alignment.power"), style:INFO},

        {string:translate("any"), style:INFO},
        {string:translate("none"), style:INFO},
        {string:translate("basic"), style:INFO},
        {string:translate("powerful"), style:INFO},
        {string:translate("unstoppable"), style:INFO},
        {string:translate("invincible"), style:INFO},
        {string:translate("dead"), style:{fontStyle: "italic", color:"gray"}},

        {string:translate("menu.wiki.abilities"), style:INFO},
        {string:translate("menu.wiki.attributes"), style:INFO},

        {string:translate("grave.killer.suicide"), style:INFO},

        // Special values:

        {string:translate("kys"), style:gradient("violet, indigo, cyan, green, yellow, orange, red")},
        
        {string:translate("pride.gay"), style:gradient("violet, indigo, cyan, green, yellow, orange, red")},
        {string:translate("pride.trans"), style:gradient("cyan, pink, white, pink, cyan")},
    ]);

    return styleSubstrings(string, stringsToStyle, styleOverride);
}
