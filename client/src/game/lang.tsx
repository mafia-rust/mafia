import React from "react";
import GAME_MANAGER from "../index";
import { ChatMessage, NightInformation } from "./chatMessage";
import ROLES from "../resources/roles.json";
import { FactionAlignment, getFactionFromFactionAlignment } from "./gameState.d";

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
    switch (message.type) {
        case "normal":
            if(message.messageSender.type === "player"){
                let playerIndex = message.messageSender.player;
                if(message.chatGroup !== "dead"){
                    return <span key={key}>{styleText(translate("chatmessage.normal",
                        GAME_MANAGER.gameState.players[playerIndex].toString(),
                        message.text
                    ), { 
                        indentStyle: { marginLeft: "2rem" } 
                    })}</span>;
                }else{
                    return <span key={key} style={{backgroundColor:"black", borderRadius: "5px"}}>{styleText(translate("chatmessage.normal",
                        GAME_MANAGER.gameState.players[playerIndex].toString(),
                        message.text
                    ), {
                        defaultStyle: { color: "grey" },
                        indentStyle: { marginLeft: "2rem" } 
                    })}</span>;
                }
            } else {
                //TODO, this only works because jailor and medium are the only options
                return <span key={key}>{styleText(translate("chatmessage.normal",
                    translate("role."+message.messageSender.type+".name"),
                    message.text
                ), {
                    defaultStyle: {color:"turquoise"}
                })}</span>;
            }
        case "whisper":
            return <span key={key}>{styleText(translate("chatmessage.whisper", 
                GAME_MANAGER.gameState.players[message.fromPlayerIndex].toString(),
                GAME_MANAGER.gameState.players[message.toPlayerIndex].toString(),
                message.text
            ), {
                defaultStyle: {color:"turquoise"}
            })}</span>;
        case "broadcastWhisper":
            return <span key={key}>{styleText(translate("chatmessage.broadcastWhisper",
                GAME_MANAGER.gameState.players[message.whisperer].toString(),
                GAME_MANAGER.gameState.players[message.whisperee].toString(),
            ), {
                defaultStyle: {color:"turquoise"}
            })}</span>;
        case "roleAssignment":
            let role = message.role;
            let name = translate("role."+role+".name")
            
            return <span key={key} style={{textAlign:"center"}}>{styleText(translate("chatmessage.roleAssignment", name), {
                defaultStyle: {color:"yellow"}
            })}</span>;
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

            return <span key={key}>{styleText(translate("chatmessage.playerDied",
                GAME_MANAGER.gameState.players[message.grave.playerIndex].toString(),
                graveRoleString,
                deathCause,
                message.grave.will
            ), {
                defaultStyle: {color:"yellow"}
            })}</span>;
        case "phaseChange":
            return <span key={key} style={{textAlign:"center", backgroundColor:"var(--primary-color)"}}>{styleText(translate("chatmessage.phaseChange",
                translate("phase."+message.phase),
                message.dayNumber
            ), {
                defaultStyle: {color:"yellow", textDecoration:"underline"}
            })}</span >;
        case "trialInformation":
            return <span key={key}>{styleText(translate("chatmessage.trialInformation",
                message.requiredVotes,
                message.trialsLeft
            ), {
                defaultStyle: {color:"orange"}
            })}</span>;
        case "voted":
            if (message.votee !== null) {
                return <span key={key}>{styleText(translate("chatmessage.voted",
                    GAME_MANAGER.gameState.players[message.voter].toString(),
                    GAME_MANAGER.gameState.players[message.votee].toString(),
                ), {
                    defaultStyle: {color:"orange"}
                })}</span>;
            } else {
                return <span key={key}>{styleText(translate("chatmessage.voted.cleared",
                    GAME_MANAGER.gameState.players[message.voter].toString(),
                ), {
                    defaultStyle: {color:"orange"}
                })}</span>;
            }
        case "playerOnTrial":
            return <span key={key}>{styleText(translate("chatmessage.playerOnTrial",
                GAME_MANAGER.gameState.players[message.playerIndex].toString(),
            ), {
                defaultStyle: {color:"yellow"}
            })}</span>;
        case "judgementVote":
            return <span key={key}>{styleText(translate("chatmessage.judgementVote",
                GAME_MANAGER.gameState.players[message.voterPlayerIndex].toString(),
            ), {
                defaultStyle: {color:"orange"}
            })}</span>;
        case "judgementVerdict":
            return <span key={key}>{styleText(translate("chatmessage.judgementVerdict",
                GAME_MANAGER.gameState.players[message.voterPlayerIndex].toString(),
                translate("verdict."+message.verdict.toLowerCase())
            ), {
                defaultStyle: {color:"orange"}
            })}</span>;
        case "trialVerdict":
            return <span key={key}>{styleText(translate("chatmessage.trialVerdict",
                GAME_MANAGER.gameState.players[GAME_MANAGER.gameState.playerOnTrial!].toString(),
                message.innocent>=message.guilty?translate("verdict.innocent"):translate("verdict.guilty"),
                message.innocent,
                message.guilty
            ), {
                defaultStyle: {color:"yellow"}
            })}</span>;
        case "nightInformation":
            return getNightInformationChatElement(message.nightInformation, key);
        case "targeted":
            if (message.targets.length > 0) {
                return <span key={key}>{styleText(translate("chatmessage.targeted",
                    GAME_MANAGER.gameState.players[message.targeter].toString(),
                    message.targets.map((target) => GAME_MANAGER.gameState.players[target].toString()).join(", ")
                ), {
                    defaultStyle: {color:"orange"}
                })}</span>;
            } else {
                return <span key={key}>{styleText(translate("chatmessage.targeted.cleared",
                    GAME_MANAGER.gameState.players[message.targeter].toString(),
                ), {
                    defaultStyle: {color:"orange"}
                })}</span>;
            }
        case "mayorRevealed":
            return <span key={key}>{styleText(translate("chatmessage.mayorRevealed",
                GAME_MANAGER.gameState.players[message.playerIndex].toString(),
            ), {
                defaultStyle: {color:"violet"}
            })}</span>;
        case "jailedTarget":
            return <span key={key}>{styleText(translate("chatmessage.jailedTarget",
                GAME_MANAGER.gameState.players[message.playerIndex].toString(),
            ), {
                defaultStyle: {color:"violet"}
            })}</span>;
        case "jailedSomeone":
            return <span key={key}>{styleText(translate("chatmessage.jailedSomeone",
                GAME_MANAGER.gameState.players[message.playerIndex].toString()
            ), {
                defaultStyle: {color:"violet"}
            })}</span>;
        case "deputyShot":
            return <span key={key}>{styleText(translate("chatmessage.deputyShot",
                GAME_MANAGER.gameState.players[message.deputyIndex].toString(),
                GAME_MANAGER.gameState.players[message.shotIndex].toString()
            ), {
                defaultStyle: {color:"violet"}
            })}</span>;
        case "jailorDecideExecute":
            if (message.targets.length > 0) {
                return <span key={key}>{styleText(translate("chatmessage.jailorDecideExecute",
                    message.targets.map((target) => GAME_MANAGER.gameState.players[target].toString()).join(", ")
                ), {
                    defaultStyle: {color:"orange"}
                })}</span>;
            } else {
                return <span key={key}>{styleText(translate("chatmessage.jailorDecideExecute.null"), {
                    defaultStyle: {color:"orange"}
                })}</span>;
            }
        default:
            console.error("Unknown message type: "+message.type);
            console.error(message);
            return <span key={key}>{styleText(translate("chatmessage."+message))}</span>;
    }
}

export function getNightInformationChatElement(info: NightInformation, key: number): JSX.Element {
    const RESULT_STYLE = { defaultStyle: { color: "green" } };
    const WARNING_STYLE = { backgroundColor: "#660000" };
    switch (info.type) {
        case "roleBlocked":
            return <span key={key}>{styleText(
                translate("chatmessage.night.roleBlocked" + (info.immune ? ".immune" : "")),
                RESULT_STYLE
            )}</span>;
        case "sheriffResult":
            return <span key={key}>{styleText(
                translate("chatmessage.night.sheriffResult." + (info.suspicious ? "suspicious" : "innocent")), 
                RESULT_STYLE
            )}</span>;
        case "lookoutResult":
            return <span key={key}>{styleText(
                translate("chatmessage.night.lookoutResult", 
                    (info.players.map((playerIndex) => GAME_MANAGER.gameState.players[playerIndex].toString()).join(", "))
                ), 
                RESULT_STYLE
            )}</span>;
        case "seerResult":
            if(info.enemies){
                return <span key={key}>{styleText(
                    translate("chatmessage.night.seerResult.enemies"),
                    RESULT_STYLE
                )}</span>;
            }else{
                return <span key={key}>{styleText(
                    translate("chatmessage.night.seerResult.friends"),
                    RESULT_STYLE
                )}</span>;
            }
        case "playerRoleAndWill":
            return <span key={key}>{styleText(
                translate("chatmessage.night.playersRoleAndWill", 
                    translate("role."+info.role+".name"), 
                    info.will
                ),
                RESULT_STYLE
            )}</span>
        case "consigliereResult":
            return <span key={key}>{styleText(
                translate("chatmessage.night.consigliereResult", 
                    translate("role."+info.role+".name"),
                    (info.visited.map((playerIndex) => GAME_MANAGER.gameState.players[playerIndex].toString()).join(", ")), 
                    (info.visitedBy.map((playerIndex) => GAME_MANAGER.gameState.players[playerIndex].toString()).join(", "))
                ),
                RESULT_STYLE
            )}</span>
        case "youDied":
        case "silenced":
            return <span key={key} style={WARNING_STYLE}>{
                translate("chatmessage.night."+info.type)
            }</span>
        default:
            return <span key={key}>{styleText(
                translate("chatmessage.night."+info.type),
                RESULT_STYLE
            )}</span>
    }
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
