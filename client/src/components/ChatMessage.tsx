import { ReactElement } from "react";
import translate from "../game/lang";
import React from "react";
import GAME_MANAGER, { find } from "..";
import StyledText from "./StyledText";
import "./chatMessage.css"
import { Phase, PlayerIndex, Verdict } from "../game/gameState.d";
import { Role } from "../game/roleState.d";
import { Grave } from "../game/grave";

export default function ChatElement(props: {message: ChatMessage}): ReactElement {
    const message = props.message;
    const chatMessageStyles = require("../resources/styling/chatMessage.json");
    let style = typeof chatMessageStyles[message.type] === "string" ? chatMessageStyles[message.type] : "";
    const text = translateChatMessage(message);

    // Special chat messages that don't play by the rules
    if (message.type === "normal") {
        if(message.messageSender.type !== "player"){
            style += " discreet";
        } else if (message.chatGroup === "dead") {
            style += " dead player";
        } else {
            style += " player"
        }
        
        if (
            GAME_MANAGER.state.stateType === "game" &&
            (find(GAME_MANAGER.state.players[GAME_MANAGER.state.myIndex!].name ?? "").test(message.text) ||
            (
                GAME_MANAGER.state.stateType === "game" &&
                GAME_MANAGER.state.myIndex !== null &&
                find("" + (GAME_MANAGER.state.myIndex + 1)).test(message.text)
            ))
            
        ) {
            style += " mention";
        } 
    } else if (message.type === "retributionistMessage" || message.type === "necromancerMessage" || message.type === "witchMessage") {
        return <>
            <StyledText className={"chat-message " + style}>{text}</StyledText>
            <ChatElement message={message.message}/>
        </>
    } else if (message.type === "playerDied") {
        return <>
            <StyledText className={"chat-message " + style}>{text}</StyledText>
            {message.grave.will.length !== 0 
                && <StyledText className={"chat-message will"}>{message.grave.will}</StyledText>}
            {message.grave.deathNotes.length !== 0 && message.grave.deathNotes.map(note => <>
                <StyledText className={"chat-message " + style}>{translate("chatMessage.deathNote")}</StyledText>
                <StyledText className={"chat-message deathNote"}>{note}</StyledText>
            </>)}
        </>
    }

    return <StyledText className={"chat-message " + style}>{text}</StyledText>;
}

function playerListToString(playerList: PlayerIndex[]): string {

    return playerList.map((playerIndex) => {
        if(GAME_MANAGER.state.stateType === "game")
            return GAME_MANAGER.state.players[playerIndex].toString();
        else
            return "ERROR: outside game"
    }).join(", ");
}

export function translateChatMessage(message: ChatMessage): string {
    if(GAME_MANAGER.state.stateType !== "game"){
        return "ERROR: outside game"
    }

    switch (message.type) {
        case "normal":
            if(message.messageSender.type === "player"){
                return translate("chatMessage.normal",
                    GAME_MANAGER.state.players[message.messageSender.player].toString(), 
                    message.text
                );
            } else {
                return translate("chatMessage.normal",
                    translate("role."+message.messageSender.type+".name"),
                    message.text
                );
            }
        case "whisper":
            return translate("chatMessage.whisper", 
                GAME_MANAGER.state.players[message.fromPlayerIndex].toString(),
                GAME_MANAGER.state.players[message.toPlayerIndex].toString(),
                message.text
            );
        case "broadcastWhisper":
            return translate("chatMessage.broadcastWhisper",
                GAME_MANAGER.state.players[message.whisperer].toString(),
                GAME_MANAGER.state.players[message.whisperee].toString(),
            );
        case "roleAssignment":
            return translate("chatMessage.roleAssignment", 
                translate("role." + message.role + ".name")
            );
        case "playerDied":
            let graveRoleString: string;
            if (message.grave.role.type === "role") {
                graveRoleString = translate(`role.${message.grave.role.role}.name`);
            } else {
                graveRoleString = translate(`grave.role.${message.grave.role.type}`);
            }

            let deathCause: string;
            if (message.grave.deathCause.type === "lynching") {
                deathCause = translate("grave.deathCause.lynching")
            } else if (message.grave.deathCause.type === "killers"){
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
                deathCause = killers.join(", ");
            }else{
                deathCause = translate(`grave.deathCause.${message.grave.deathCause.type}`)
            }

            return translate(message.grave.will.length === 0 ? "chatMessage.playerDied.noWill": "chatMessage.playerDied",
                GAME_MANAGER.state.players[message.grave.playerIndex].toString(),
                graveRoleString,
                deathCause
            );
        case "youDied":
            return translate("chatMessage.youDied");
        case "phaseChange":
            return translate("chatMessage.phaseChange",
                translate("phase."+message.phase),
                message.dayNumber
            );
        case "trialInformation":
            return translate("chatMessage.trialInformation",
                message.requiredVotes,
                message.trialsLeft
            );
        case "voted":
            if (message.votee !== null) {
                return translate("chatMessage.voted",
                    GAME_MANAGER.state.players[message.voter].toString(),
                    GAME_MANAGER.state.players[message.votee].toString(),
                );
            } else {
                return translate("chatMessage.voted.cleared",
                    GAME_MANAGER.state.players[message.voter].toString(),
                );
            }
        case "playerOnTrial":
            return translate("chatMessage.playerOnTrial",
                GAME_MANAGER.state.players[message.playerIndex].toString(),
            );
        case "judgementVote":
            return translate("chatMessage.judgementVote",
                GAME_MANAGER.state.players[message.voterPlayerIndex].toString(),
            );
        case "judgementVerdict":
            return translate("chatMessage.judgementVerdict",
                GAME_MANAGER.state.players[message.voterPlayerIndex].toString(),
                translate("verdict."+message.verdict.toLowerCase())
            );
        case "trialVerdict":
            return translate("chatMessage.trialVerdict",
                GAME_MANAGER.state.players[message.playerOnTrial].toString(),
                message.innocent>=message.guilty?translate("verdict.innocent"):translate("verdict.guilty"),
                message.innocent,
                message.guilty
            );
        case "targeted":
            if (message.targets.length > 0) {
                return translate("chatMessage.targeted",
                    GAME_MANAGER.state.players[message.targeter].toString(),
                    playerListToString(message.targets));
            } else {
                return translate("chatMessage.targeted.cleared",
                    GAME_MANAGER.state.players[message.targeter].toString(),
                );
            }
        case "mayorRevealed":
            return translate("chatMessage.mayorRevealed",
                GAME_MANAGER.state.players[message.playerIndex].toString(),
            );
        case "jailedTarget":
            return translate("chatMessage.jailedTarget",
                GAME_MANAGER.state.players[message.playerIndex].toString(),
            );
        case "jailedSomeone":
            return translate("chatMessage.jailedSomeone",
                GAME_MANAGER.state.players[message.playerIndex].toString()
            );
        case "deputyKilled":
            return translate("chatMessage.deputyKilled",
                GAME_MANAGER.state.players[message.shotIndex].toString()
            );
        case "jailorDecideExecute":
            if (message.targets.length > 0) {
                return translate("chatMessage.jailorDecideExecute", playerListToString(message.targets));
            } else {
                return translate("chatMessage.jailorDecideExecute.nobody");
            }
        case "godfatherBackup":
            if (message.backup !== null) {
                return translate("chatMessage.godfatherBackup", GAME_MANAGER.state.players[message.backup].toString());
            } else {
                return translate("chatMessage.godfatherBackup.nobody");
            }
        /* NIGHT */
        case "roleBlocked":
            return translate("chatMessage.roleBlocked" + (message.immune ? ".immune" : ""));
        case "sheriffResult":
            return translate("chatMessage.sheriffResult." + (message.suspicious ? "suspicious" : "innocent"));
        case "lookoutResult":
            if (message.players.length === 0) {
                return translate("chatMessage.lookoutResult.nobody");
            } else {
                return translate("chatMessage.lookoutResult", playerListToString(message.players));
            }
        case "spyMafiaVisit":
            if (message.players.length === 0) {
                return translate("chatMessage.spyMafiaVisit.nobody");
            } else {
                return translate("chatMessage.spyMafiaVisit", playerListToString(message.players));
            }
        case "spyBug":
            return translate("chatMessage.spyBug."+message.bug);
        case "trackerResult":
            if (message.players.length === 0) {
                return translate("chatMessage.trackerResult.nobody");
            } else {
                return translate("chatMessage.trackerResult", playerListToString(message.players));
            }
        case "seerResult":
            return translate("chatMessage.seerResult." + (message.enemies ? "enemies" : "friends"));
        case "psychicEvil":
            return translate("chatMessage.psychicEvil", playerListToString(message.players));
        case "psychicGood":
            return translate("chatMessage.psychicGood", playerListToString(message.players));
        case "playerRoleAndWill":
            return translate("chatMessage.playersRoleAndWill", 
                translate("role."+message.role+".name"), 
                message.will
            );
        case "consigliereResult":
            const visitedNobody = message.visited.length === 0;
            const visitedByNobody = message.visitedBy.length === 0;

            return translate("chatMessage.consigliereResult", 
                translate("chatMessage.consigliereResult.role", translate("role."+message.role+".name")),
                visitedNobody 
                    ? translate("chatMessage.consigliereResult.visited.nobody") 
                    : translate("chatMessage.consigliereResult.visited", playerListToString(message.visited)),
                visitedByNobody 
                    ? translate("chatMessage.consigliereResult.visitedBy.nobody") 
                    : translate("chatMessage.consigliereResult.visitedBy", playerListToString(message.visitedBy))
            );
        case "silenced":
            return translate("chatMessage.silenced");
        case "mediumSeance":
            return translate("chatMessage.mediumSeance", GAME_MANAGER.state.players[message.player].toString());
        case "witchedYou":
            return translate("chatMessage.witchedYou" + (message.immune ? ".immune" : ""));
        case "playerWithNecronomicon":
            return translate("chatMessage.playerWithNecronomicon", GAME_MANAGER.state.players[message.playerIndex].toString());
        case "deputyShotSomeoneSurvived":
        case "deathCollectedSouls":
        case "arsonistCleanedSelf":
        case "arsonistDousedPlayers":
        case "targetWasAttacked":
        case "youWereProtected":
        case "executionerWon":
        case "gameOver":
        case "jesterWon":
        case "mayorCantWhisper":
        case "targetJailed":
        case "targetSurvivedAttack":
        case "transported":
        case "veteranAttackedVisitor":
        case "veteranAttackedYou":
        case "vigilanteSuicide":
        case "witchTargetImmune":
        case "youSurvivedAttack":
        case "doomsayerFailed":
        case "doomsayerWon":
        case "retributionistMessage":
        case "necromancerMessage":
        case "witchMessage":
        case "psychicFailed":
            return translate("chatMessage."+message.type);
        default:
            console.error("Unknown message type " + (message as any).type + ":");
            console.error(message);
            return "FIXME: " + translate("chatMessage." + message);
    }
}

export type ChatMessage = {
    type: "normal", 
    messageSender: MessageSender, 
    text: string, 
    chatGroup: "all" | "mafia" | "dead" | "vampire"
} | {
    type: "whisper", 
    fromPlayerIndex: PlayerIndex, 
    toPlayerIndex: PlayerIndex, 
    text: string
} | {
    type: "broadcastWhisper", 
    whisperer: PlayerIndex, 
    whisperee: PlayerIndex 
} | 
// System
{
    type: "roleAssignment", 
    role: Role
} | {
    type: "playerDied", 
    grave: Grave
} | {
    type: "gameOver"
} | {
    type: "phaseChange", 
    phase: Phase, 
    dayNumber: number
} | 
// Trial
{
    type: "trialInformation", 
    requiredVotes: number, 
    trialsLeft: number
} | {
    type: "voted", 
    voter: PlayerIndex, 
    votee: PlayerIndex | null 
} | {
    type: "playerOnTrial", 
    playerIndex: PlayerIndex
} | {
    type: "judgementVote", 
    voterPlayerIndex: PlayerIndex
} | {
    type: "judgementVerdict", 
    voterPlayerIndex: PlayerIndex, 
    verdict: Verdict
} | {
    type: "trialVerdict", 
    playerOnTrial: PlayerIndex, 
    innocent: number, 
    guilty: number
} | 
// Misc.
{
    type: "targeted", 
    targeter: PlayerIndex, 
    targets: PlayerIndex[]
} | 
// Role-specific
{
    type: "mayorRevealed", 
    playerIndex: PlayerIndex
} | {
    type: "mayorCantWhisper"
} | {
    type: "jailedTarget"
    playerIndex: PlayerIndex
} | {
    type: "jailedSomeone",
    playerIndex: PlayerIndex
} | {
    type: "jailorDecideExecute"
    targets: PlayerIndex[]
} | {
    type: "mediumSeance",
    player: PlayerIndex
} | {
    type: "deputyKilled",
    shotIndex: PlayerIndex
} | {
    type: "deputyShotSomeoneSurvived"
} | {
    type: "playerWithNecronomicon",
    playerIndex: PlayerIndex
} | {
    type: "roleBlocked", 
    immune : boolean
} | {
    type: "targetSurvivedAttack"
} | {
    type: "youSurvivedAttack"
} |
/* Role-specific */
{
    type: "targetJailed"
} | {
    type: "sheriffResult", 
    suspicious: boolean
} | {
    type: "lookoutResult", 
    players: PlayerIndex[]
} | {
    type: "spyMafiaVisit", 
    players: PlayerIndex[]
} | {
    type: "spyBug", 
    bug: String
} | {
    type: "trackerResult",
    players: PlayerIndex[]
} | {
    type: "seerResult",
    enemies: boolean
} | {
    type: "psychicGood",
    players: PlayerIndex[]
} | {
    type: "psychicEvil",
    players: PlayerIndex[]
} | {
    type: "psychicFailed"
} | {
    type: "retributionistMessage", 
    message: ChatMessage
} | {
    type: "necromancerMessage", 
    message: ChatMessage
} | {
    type: "veteranAttackedYou"
} | {
    type: "veteranAttackedVisitor"
} | {
    type: "vigilanteSuicide"
} | {
    type: "targetWasAttacked"
} | {
    type: "youWereProtected"
} | {
    type: "youDied"
} | {
    type: "transported"
} | {
    type: "godfatherBackup",
    backup: PlayerIndex | null
} | {
    type: "silenced"
} | {
    type: "playerRoleAndWill", 
    role: Role,
    will: string
} | {
    type: "consigliereResult", 
    role: Role,
    visitedBy: PlayerIndex[],
    visited: PlayerIndex[]
} | {
    type: "witchTargetImmune"
} | {
    type: "witchedYou",
    immune: boolean
} | {
    type: "witchMessage",
    message: ChatMessage
} | {
    type: "arsonistCleanedSelf"
} | {
    type: "arsonistDousedPlayers", 
    players: PlayerIndex[]
} | {
    type: "jesterWon"
} | {
    type: "deathCollectedSouls"
} | {
    type: "executionerWon"
} | {
    type: "doomsayerFailed"
} | {
    type: "doomsayerWon"
}

export type MessageSender = {
    type: "player", 
    player: PlayerIndex
} | {
    type: "jailor" | "medium"
}
