import React from "react";
import GAME_MANAGER from "../index";
import { ChatMessage, NightInformation } from "./net/chatMessage";
import ROLES from "../resources/roles.json";

let lang: ReadonlyMap<string, string>;
switchLanguage("en_us");

export function switchLanguage(language: string) {
    let json = require("../resources/lang/" + language + ".json");
    lang = new Map<string, string>(Object.entries(json))
}

export default function translate(langKey: string, ...valuesList: any[]): string {
    let out = lang.get(langKey);
    if(out===undefined){
        console.log("Error: Attempted to use non existant lang key: "+langKey);
        console.trace();
        return langKey;
    }
    for(let i = 0; i < valuesList.length; i++){
        out = out.replace("\\"+(i), valuesList[i]);
    }
    return out;
}

// TODO, make message union type (& make an interface) & make getChatString a method
export function getChatElement(message: ChatMessage, key: number): JSX.Element {
    switch (message.type) {
        case "normal":
            if(message.messageSender.type === "player"){
                let playerIndex = message.messageSender.player;
                return <span key={key}>{styleText(translate("chatmessage.normal",
                    GAME_MANAGER.gameState.players[playerIndex].toString(),
                    message.text
                ))}</span>;
            } else {
                //TODO
                return <span key={key}></span>;
            }
        case "whisper":
            return <span key={key}>{styleText(translate("chatmessage.whisper", 
                GAME_MANAGER.gameState.players[message.fromPlayerIndex].toString(),
                GAME_MANAGER.gameState.players[message.toPlayerIndex].toString(),
                message.text
            ), {color:"turquoise"})}</span>;
        case "broadcastWhisper":
            return <span key={key}>{styleText(translate("chatmessage.broadcastWhisper",
                GAME_MANAGER.gameState.players[message.whisperer].toString(),
                GAME_MANAGER.gameState.players[message.whisperee].toString(),
            ), {color:"turquoise"})}</span>;
        case "roleAssignment":
            let role = message.role;
            let name = translate("role."+role+".name")
            
            return <span key={key} style={{textAlign:"center"}}>{styleText(translate("chatmessage.roleAssignment", name), {color:"yellow"})}</span>;
        case "playerDied":
            //TODO, role doesnt work properly
            let graveRole: string;
            if (message.grave.role.type === "role") {
                graveRole = translate(`role.${message.grave.role.role}.name`);
            } else {
                graveRole = translate(`grave.role.${message.grave.role.type}`);
            }
            let deathCause: string;
            if (message.grave.deathCause.type === "lynching") {
                deathCause = translate("grave.deathCause.lynching")
            } else {
                let killers: string[] = [];
                for (let killer of message.grave.deathCause.killers) {
                    if (killer.type === "role") {
                        killers.push(translate(`role.${killer.role}.name`))
                    } else {
                        killers.push(translate(`grave.killer.${killer.type}`))
                    }
                }
                deathCause = killers.join();
            }

            return <span key={key}>{styleText(translate("chatmessage.playerDied",
                GAME_MANAGER.gameState.players[message.grave.playerIndex].toString(),
                graveRole,
                deathCause,
                message.grave.will
            ), {color:"yellow"})}</span>;
        case "phaseChange":
            return <span key={key} style={{textAlign:"center"}}>{styleText(translate("chatmessage.phaseChange",
                translate("phase."+message.phase),
                message.dayNumber
            ), {color:"yellow"})}</span>;
        case "trialInformation":
            return <span key={key}>{styleText(translate("chatmessage.trialInformation",
                message.requiredVotes,
                message.trialsLeft
            ), {color:"orange"})}</span>;
        case "voted":
            if (message.votee !== undefined) {
                return <span key={key}>{styleText(translate("chatmessage.voted",
                    GAME_MANAGER.gameState.players[message.voter],
                    GAME_MANAGER.gameState.players[message.votee],
                ), {color:"orange"})}</span>;
            } else {
                return <span key={key}>{styleText(translate("chatmessage.voted.cleared",
                    GAME_MANAGER.gameState.players[message.voter],
                ), {color:"orange"})}</span>;
            }
        case "playerOnTrial":
            return <span key={key}>{styleText(translate("chatmessage.playerOnTrial",
                GAME_MANAGER.gameState.players[message.playerIndex],
            ), {color:"yellow"})}</span>;
        case "judgementVote":
            return <span key={key}>{styleText(translate("chatmessage.judgementVote",
                GAME_MANAGER.gameState.players[message.voterPlayerIndex],
            ), {color:"orange"})}</span>;
        case "judgementVerdict":
            return <span key={key}>{styleText(translate("chatmessage.judgementVerdict",
                GAME_MANAGER.gameState.players[message.voterPlayerIndex],
                translate("verdict."+message.verdict)
            ), {color:"orange"})}</span>;
        case "trialVerdict":
            return <span key={key}>{styleText(translate("chatmessage.trialVerdict",
                GAME_MANAGER.gameState.players[GAME_MANAGER.gameState.playerOnTrial!].toString(),
                message.innocent>=message.guilty?translate("verdict.innocent"):translate("verdict.guilty"),
                message.innocent,
                message.guilty
            ), {color:"yellow"})}</span>;
        case "nightInformation":
            return <span key={key}>{getNightInformationString(message.nightInformation)}</span>;
        case "targeted":
            if (message.target !== undefined) {
                return <span key={key}>{styleText(translate("chatmessage.targeted",
                    GAME_MANAGER.gameState.players[message.targeter],
                    GAME_MANAGER.gameState.players[message.target],
                ), {color:"orange"})}</span>;
            } else {
                return <span key={key}>{styleText(translate("chatmessage.targeted.cleared",
                    GAME_MANAGER.gameState.players[message.targeter],
                ), {color:"orange"})}</span>;
            }
        case "mayorRevealed":
            return <span key={key}>{styleText(translate("chatmessage.mayorRevealed",
                GAME_MANAGER.gameState.players[message.playerIndex],
            ), {color:"violet"})}</span>;
        default:
            return <span key={key}>{styleText(translate("chatmessage."+message))}</span>;
    }
}

// TODO make night information message union type (& make an interface) and make this a method
export function getNightInformationString(info: NightInformation){
    switch (info.type) {
        case "roleBlocked":
            return translate("chatmessage.night.roleBlocked" + (info.immune ? ".immune" : ""));
        case "sheriffResult":
            return translate("chatmessage.night.sheriffResult." + (info.suspicious ? "suspicious" : "innocent"));
        default:
            return translate("chatmessage.night."+info.type);
    }
}

function styleSubstring(string: string, stringsToStyle: {string: string, style: React.CSSProperties}[], defaultStyle: React.CSSProperties = {}): JSX.Element[]{

    type StyledOrNot = string | {string: string, style: React.CSSProperties};

    let finalOutList: StyledOrNot[] = [];
    finalOutList.push(string);

    for(let i in stringsToStyle){
        for(let j in finalOutList){

            if(typeof finalOutList[j] !== "string"){
                continue;
            }

            
            const regEscape = (v: string) => v.replace(/[-[\]{}()*+?.,\\^$|#\s]/g, '\\$&');

            let stringSplit = (finalOutList[j] as string).split(RegExp(regEscape(stringsToStyle[i].string), "gi"));
            let outList: StyledOrNot[] = []; 

            for(let k in stringSplit){
                if(stringSplit[k] !== "") outList.push(stringSplit[k]);
                outList.push({string: stringsToStyle[i].string, style: stringsToStyle[i].style});
            }
            outList.pop();

            //inject outlist into finaloutlist at position j, without using splice
            finalOutList = 
                finalOutList.slice(0, Number(j))
                .concat(outList)
                .concat(finalOutList.slice(Number(j)+1));
        }
    }
    


    //turn into jsx
    let outJsxList = [];
    for(let i in finalOutList){
        if(typeof finalOutList[i] === "string"){

            outJsxList.push(
            <span key={i} style={
                defaultStyle
            }>
                {finalOutList[i] as string}
            </span>);

        }else if(typeof finalOutList[i] === "object"){
            outJsxList.push(
            <span key={i} style={
                (finalOutList[i] as {string: string, style: string}).style as React.CSSProperties
            }>
                {(finalOutList[i] as {string: string, style: string}).string as string}
            </span>);
        }
    }

    return outJsxList;
}

export function styleText(string: string, defaultStyle: React.CSSProperties = {}): JSX.Element[]{
    let stringsToStyle: {string: string, style: React.CSSProperties}[] = [];

    for(let role in (ROLES as any)){
        let roleObject = (ROLES as any)[role];

        switch(roleObject.faction){
            case "Coven":
                stringsToStyle.push({string:translate("role."+role+".name"), style:{
                    color: "magenta"
                }});
                break;
            case "Town":
                stringsToStyle.push({string:translate("role."+role+".name"), style:{
                    color: "lime"
                }});
                break;
            case "Mafia":
                stringsToStyle.push({string:translate("role."+role+".name"), style:{
                    color: "red"
                }});
                break;
        }
    }

    stringsToStyle = stringsToStyle.concat(
        GAME_MANAGER.gameState.players.map((player)=>{
            return {string:player.toString(), style:{
                fontStyle: "italic",
                fontWeight: "bold"
            }};
        })
    );

    stringsToStyle = stringsToStyle.concat([
        {string:translate("verdict.guilty"), style:{color:"red"}},
        {string:translate("verdict.innocent"), style:{color:"lime"}},
        {string:translate("verdict.abstain"), style:{color:"cyan"}},
        
        {string:translate("faction.Town"), style:{color:"lime"}},
        {string:translate("faction.Mafia"), style:{color:"red"}},
        {string:translate("faction.Neutral"), style:{color:"cyan"}},
        {string:translate("faction.Coven"), style:{color:"magenta"}},
        {string:translate("faction.Random"), style:{color:"lightblue"}},

        {string:translate("alignment.Killing"), style:{color:"lightblue"}},
        {string:translate("alignment.Investigative"), style:{color:"lightblue"}},
        {string:translate("alignment.Protective"), style:{color:"lightblue"}},
        {string:translate("alignment.Support"), style:{color:"lightblue"}},
        {string:translate("alignment.Deception"), style:{color:"lightblue"}},
        {string:translate("alignment.Evil"), style:{color:"lightblue"}},
        {string:translate("alignment.Chaos"), style:{color:"lightblue"}},
        {string:translate("alignment.Random"), style:{color:"lightblue"}},
        {string:translate("alignment.Utility"), style:{color:"lightblue"}},
        {string:translate("alignment.Power"), style:{color:"lightblue"}},

    ]);

    

    return styleSubstring(string, stringsToStyle, defaultStyle);
}
