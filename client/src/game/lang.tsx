import React from "react";
import GAME_MANAGER from "../index";
import { ChatMessage, NightInformation } from "./net/chatMessage";

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
export function getChatString(message: ChatMessage): string {
    switch (message.type) {
        case "normal":
            if(message.messageSender.type === "player"){
                let playerIndex = message.messageSender.player;
                return translate("chatmessage.normal",
                    GAME_MANAGER.gameState.players[playerIndex],
                    message.text
                );
            } else {
                //TODO
                return "";
            }
        case "whisper":
            return translate("chatmessage.whisper", 
                GAME_MANAGER.gameState.players[message.fromPlayerIndex],
                GAME_MANAGER.gameState.players[message.toPlayerIndex],
                message.text
            );
        case "broadcastWhisper":
            return translate("chatmessage.broadcastWhisper",
                GAME_MANAGER.gameState.players[message.whisperer],
                GAME_MANAGER.gameState.players[message.whisperee],
            );
        case "roleAssignment":
            let role = message.role;
            let name = translate("role."+role+".name")
            
            return translate("chatmessage.roleAssignment", name);
        case "playerDied":
            //TODO, role doesnt work properly
            return translate("chatmessage.playerDied",
                GAME_MANAGER.gameState.players[message.grave.playerIndex].toString(),
                JSON.stringify(message.grave.role),
                JSON.stringify(message.grave.deathCause),
                message.grave.will
            );
        case "phaseChange":
            return translate("chatmessage.phaseChange",
                translate("phase."+message.phase),
                message.dayNumber
            );
        case "trialInformation":
            return translate("chatmessage.trialInformation",
                message.requiredVotes,
                message.trialsLeft
            );
        case "voted":
            if (message.votee !== undefined) {
                return translate("chatmessage.voted",
                    GAME_MANAGER.gameState.players[message.voter],
                    GAME_MANAGER.gameState.players[message.votee],
                );
            } else {
                return translate("chatmessage.voted.cleared",
                    GAME_MANAGER.gameState.players[message.voter],
                );
            }
        case "playerOnTrial":
            return translate("chatmessage.playerOnTrial",
                GAME_MANAGER.gameState.players[message.playerIndex],
            );
        case "judgementVote":
            return translate("chatmessage.judgementVote",
                GAME_MANAGER.gameState.players[message.voterPlayerIndex],
            );
        case "judgementVerdict":
            return translate("chatmessage.judgementVerdict",
                GAME_MANAGER.gameState.players[message.voterPlayerIndex],
                translate("verdict."+message.verdict)
            );
        case "trialVerdict":
            return translate("chatmessage.trialVerdict",
                GAME_MANAGER.gameState.players[GAME_MANAGER.gameState.playerOnTrial!].toString(),
                message.innocent>=message.guilty?translate("verdict.innocent"):translate("verdict.guilty"),
                message.innocent,
                message.guilty
            );
        case "nightInformation":
            return getNightInformationString(message.nightInformation);
        case "targeted":
            if (message.target !== undefined) {
                return translate("chatmessage.targeted",
                    GAME_MANAGER.gameState.players[message.targeter],
                    GAME_MANAGER.gameState.players[message.target],
                );
            } else {
                return translate("chatmessage.targeted.cleared",
                    GAME_MANAGER.gameState.players[message.targeter],
                );
            }
        case "mayorRevealed":
            return translate("chatmessage.mayorRevealed",
                GAME_MANAGER.gameState.players[message.playerIndex],
            );
        default:
            return translate("chatmessage."+message);
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

export function colorText(string: string, stringsToColor: {string: string, color: string}[]): JSX.Element[]{

    type ColorOrNot = string | {string: string, color: string};

    let finalOutList: ColorOrNot[] = [];
    finalOutList.push(string);

    for(let i in stringsToColor){
        for(let j in finalOutList){

            if(typeof finalOutList[j] !== "string"){
                continue;
            }

            let stringSplit = (finalOutList[j] as string).split(RegExp(stringsToColor[i].string, "gi"));
            let outList: ColorOrNot[] = []; 

            for(let k in stringSplit){
                outList.push(stringSplit[k]);
                outList.push({string: stringsToColor[i].string, color: stringsToColor[i].color});
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
            outJsxList.push(<span key={i}>{finalOutList[i] as string}</span>);
        }else if(typeof finalOutList[i] === "object"){
            outJsxList.push(
            <span key={i} style={
                {color: (finalOutList[i] as {string: string, color: string}).color as string}
            }>
                {(finalOutList[i] as {string: string, color: string}).string as string}
            </span>);
        }
    }

    return outJsxList;
}

