import { ReactElement } from "react";
import translate, { translateChecked } from "../game/lang";
import React from "react";
import GAME_MANAGER, { find, replaceMentions } from "..";
import StyledText from "./StyledText";
import "./chatMessage.css"
import { Phase, PlayerIndex, Verdict } from "../game/gameState.d";
import { Role } from "../game/roleState.d";
import { Grave } from "../game/graveState";
import DOMPurify from "dompurify";
import GraveComponent from "./grave";

export default function ChatElement(props: {message: ChatMessage}): ReactElement {
    const message = props.message;
    const chatMessageStyles = require("../resources/styling/chatMessage.json");
    let style = typeof chatMessageStyles[message.type] === "string" ? chatMessageStyles[message.type] : "";

    // Special chat messages that don't play by the rules
    switch (message.type) {
        case "normal":
            if(message.messageSender.type !== "player"){
                style += " discreet";
            } else if (message.chatGroup === "dead") {
                style += " dead player";
            } else {
                style += " player"
            }
            
            if (
                GAME_MANAGER.state.stateType === "game" &&
                (find(GAME_MANAGER.state.players[GAME_MANAGER.state.myIndex!].name ?? "").test(sanitizePlayerMessage(replaceMentions(
                    message.text,
                    GAME_MANAGER.state.players
                ))) ||
                (
                    GAME_MANAGER.state.stateType === "game" &&
                    GAME_MANAGER.state.myIndex !== null &&
                    find("" + (GAME_MANAGER.state.myIndex + 1)).test(sanitizePlayerMessage(replaceMentions(
                        message.text,
                        GAME_MANAGER.state.players
                    )))
                ))
                
            ) {
                style += " mention";
            }
            break;
        case "targetsMessage":
            return <>
                <StyledText className={"chat-message " + style}>{translateChatMessage(message)}</StyledText>
                <ChatElement message={message.message}/>
            </>
        case "playerDied":
            if(GAME_MANAGER.state.stateType === "game"){
                return <>
                    <StyledText className={"chat-message " + style}>{translate("chatMessage.playerDied",
                        GAME_MANAGER.state.players[message.grave.playerIndex].toString(),
                    )}</StyledText>
                    <div className="grave-message">
                        <GraveComponent grave={message.grave} gameState={GAME_MANAGER.state}/>
                    </div>
                </>;
            }
            else{
                return <></>;
            }
    }

    return <StyledText className={"chat-message " + style}>{translateChatMessage(message)}</StyledText>;
}

function playerListToString(playerList: PlayerIndex[]): string {

    return playerList.map((playerIndex) => {
        if(GAME_MANAGER.state.stateType === "game")
            return GAME_MANAGER.state.players[playerIndex].toString();
        else
            return "ERROR: outside game"
    }).join(", ");
}

export function sanitizePlayerMessage(text: string): string {
    return DOMPurify.sanitize(text, { 
        ALLOWED_TAGS: []
    });
}

export function translateChatMessage(message: ChatMessage): string {
    if(GAME_MANAGER.state.stateType !== "game"){
        return "ERROR: chatmessages cant exist outside game"
    }

    switch (message.type) {
        case "normal":
            let icon = translateChecked("chatGroup."+message.chatGroup+".icon");

            if(message.messageSender.type === "player"){
                return (icon??"")+translate("chatMessage.normal",
                    "sender-"+GAME_MANAGER.state.players[message.messageSender.player].toString(), 
                    sanitizePlayerMessage(replaceMentions(message.text, GAME_MANAGER.state.players))
                );
            } else {
                return (icon??"")+translate("chatMessage.normal",
                    translate("role."+message.messageSender.type+".name"),
                    sanitizePlayerMessage(replaceMentions(message.text, GAME_MANAGER.state.players))
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
        case "playerQuit":
            return translate("chatMessage.playerQuit",
                GAME_MANAGER.state.players[message.playerIndex].toString()
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
        case "journalistJournal":
            return translate("chatMessage.journalistJournal",
                message.journal
            );
        case "youAreInterviewingPlayer":
            return translate("chatMessage.youAreInterviewingPlayer",
                GAME_MANAGER.state.players[message.playerIndex].toString(),
            );
        case "playerIsBeingInterviewed":
            return translate("chatMessage.playerIsBeingInterviewed",
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
        case "trapperVisitorsRole":
            return translate("chatMessage.trapperVisitorsRole", translate("role."+message.role+".name"));
        case "trapState":
            return translate("chatMessage.trapState."+message.state.type);
        case "playerRoleAndWill":
            return translate("chatMessage.playersRoleAndWill", 
                translate("role."+message.role+".name"), 
                sanitizePlayerMessage(message.will)
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
        case "youWerePossessed":
            return translate("chatMessage.youWerePossessed" + (message.immune ? ".immune" : ""));
        case "werewolfTrackingResult":
            if(message.players.length === 0){
                return translate(
                    "chatMessage.werewolfTrackingResult.nobody", 
                    GAME_MANAGER.state.players[message.trackedPlayer].toString()
                );
            }else{
                return translate("chatMessage.werewolfTrackingResult", 
                    GAME_MANAGER.state.players[message.trackedPlayer].toString(),
                    playerListToString(message.players)
                );
            }
        case "playerWithNecronomicon":
            return translate("chatMessage.playerWithNecronomicon", GAME_MANAGER.state.players[message.playerIndex].toString());
        case "deputyShotYou":
        case "deathCollectedSouls":
        case "targetWasAttacked":
        case "youWereProtected":
        case "executionerWon":
        case "gameOver":
        case "jesterWon":
        case "targetJailed":
        case "someoneSurvivedYourAttack":
        case "transported":
        case "veteranAttackedVisitor":
        case "veteranAttackedYou":
        case "trapperYouAttackedVisitor":
        case "vigilanteSuicide":
        case "targetIsPossessionImmune":
        case "youSurvivedAttack":
        case "doomsayerFailed":
        case "doomsayerWon":
        case "retributionistMessage":
        case "necromancerMessage":
        case "targetsMessage":
        case "psychicFailed":
            return translate("chatMessage."+message.type);
        case "playerDied":
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
    chatGroup: "all" | "dead" | "mafia" | "vampire" | "seance" | "jail" | "interview"
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
    type: "playerQuit",
    playerIndex: PlayerIndex
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
    type: "journalistJournal",
    journal: string
} | {
    type: "youAreInterviewingPlayer",
    playerIndex: PlayerIndex
} | {
    type: "playerIsBeingInterviewed",
    playerIndex: PlayerIndex
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
    type: "deputyShotYou"
} | {
    type: "playerWithNecronomicon",
    playerIndex: PlayerIndex
} | {
    type: "roleBlocked", 
    immune : boolean
} | {
    type: "someoneSurvivedYourAttack"
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
    type: "trapperVisitorsRole",
    role: Role
} | {
    type: "trapState",
    state: {
        type: "dismantled" | "ready" | "set"
    }
} | {
    type: "trapperYouAttackedVisitor"
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
    type: "targetIsPossessionImmune"
} | {
    type: "youWerePossessed",
    immune: boolean
} | {
    type: "targetsMessage",
    message: ChatMessage
} | {
    type: "werewolfTrackingResult",
    trackedPlayer: PlayerIndex
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
    type: "jailor" | "medium" | "journalist"
}
