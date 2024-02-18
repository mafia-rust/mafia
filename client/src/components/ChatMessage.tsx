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

export default function ChatElement(
    props: {
        message: ChatMessage,
        playerNames?: string[]
    }, 
): ReactElement {
    const message = props.message;
    const playerNames = props.playerNames ?? GAME_MANAGER.getPlayerNames();
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
                    playerNames
                ))) ||
                (
                    GAME_MANAGER.state.stateType === "game" &&
                    GAME_MANAGER.state.myIndex !== null &&
                    find("" + (GAME_MANAGER.state.myIndex + 1)).test(sanitizePlayerMessage(replaceMentions(
                        message.text,
                        playerNames
                    )))
                ))
                
            ) {
                style += " mention";
            }
            break;
        case "targetsMessage":
            return <>
                <StyledText className={"chat-message " + style}>{translateChatMessage(message, playerNames)}</StyledText>
                <ChatElement message={message.message} playerNames={playerNames}/>
            </>
        case "playerDied":
            if(GAME_MANAGER.state.stateType === "game"){
                return <>
                    <StyledText className={"chat-message " + style}>{translate("chatMessage.playerDied",
                        playerNames[message.grave.playerIndex],
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

    return <StyledText className={"chat-message " + style}>{translateChatMessage(message, playerNames)}</StyledText>;
}

function playerListToString(playerList: PlayerIndex[], playerNames: string[]): string {

    return playerList.map((playerIndex) => {
        return playerNames[playerIndex];
    }).join(", ");
}

export function sanitizePlayerMessage(text: string): string {
    return DOMPurify.sanitize(text, { 
        ALLOWED_TAGS: []
    });
}

export function translateChatMessage(message: ChatMessage, playerNames?: string[]): string {

    if (playerNames === undefined) {
        playerNames = GAME_MANAGER.getPlayerNames();
    }

    switch (message.type) {
        case "normal":
            const icon = translateChecked("chatGroup."+message.chatGroup+".icon");

            if(message.messageSender.type === "player"){
                return (icon??"")+translate("chatMessage.normal",
                    "sender-"+playerNames[message.messageSender.player], 
                    sanitizePlayerMessage(replaceMentions(message.text, playerNames))
                );
            } else {
                return (icon??"")+translate("chatMessage.normal",
                    translate("role."+message.messageSender.type+".name"),
                    sanitizePlayerMessage(replaceMentions(message.text, playerNames))
                );
            }
        case "whisper":
            return translate("chatMessage.whisper", 
                playerNames[message.fromPlayerIndex],
                playerNames[message.toPlayerIndex],
                message.text
            );
        case "broadcastWhisper":
            return translate("chatMessage.broadcastWhisper",
                playerNames[message.whisperer],
                playerNames[message.whisperee],
            );
        case "roleAssignment":
            return translate("chatMessage.roleAssignment", 
                translate("role." + message.role + ".name")
            );
        case "playerQuit":
            return translate("chatMessage.playerQuit",
                playerNames[message.playerIndex]
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
                    playerNames[message.voter],
                    playerNames[message.votee],
                );
            } else {
                return translate("chatMessage.voted.cleared",
                    playerNames[message.voter],
                );
            }
        case "playerOnTrial":
            return translate("chatMessage.playerOnTrial",
                playerNames[message.playerIndex],
            );
        case "judgementVerdict":
            return translate("chatMessage.judgementVerdict",
                playerNames[message.voterPlayerIndex],
                translate("verdict."+message.verdict.toLowerCase())
            );
        case "trialVerdict":
            return translate("chatMessage.trialVerdict",
                playerNames[message.playerOnTrial],
                message.innocent>=message.guilty?translate("verdict.innocent"):translate("verdict.guilty"),
                message.innocent,
                message.guilty
            );
        case "targeted":
            if (message.targets.length > 0) {
                return translate("chatMessage.targeted",
                    playerNames[message.targeter],
                    playerListToString(message.targets, playerNames));
            } else {
                return translate("chatMessage.targeted.cleared",
                    playerNames[message.targeter],
                );
            }
        case "mayorRevealed":
            return translate("chatMessage.mayorRevealed",
                playerNames[message.playerIndex],
            );
        case "martyrRevealed":
            return translate("chatMessage.martyrRevealed",
                playerNames[message.martyr],
            );
        case "journalistJournal":
            return translate("chatMessage.journalistJournal",
                sanitizePlayerMessage(replaceMentions(message.journal))
            );
        case "youAreInterviewingPlayer":
            return translate("chatMessage.youAreInterviewingPlayer",
                playerNames[message.playerIndex],
            );
        case "playerIsBeingInterviewed":
            return translate("chatMessage.playerIsBeingInterviewed",
                playerNames[message.playerIndex],
            );
        case "jailedTarget":
            return translate("chatMessage.jailedTarget",
                playerNames[message.playerIndex],
            );
        case "jailedSomeone":
            return translate("chatMessage.jailedSomeone",
                playerNames[message.playerIndex]
            );
        case "deputyKilled":
            return translate("chatMessage.deputyKilled",
                playerNames[message.shotIndex]
            );
        case "jailorDecideExecute":
            if (message.targets.length > 0) {
                return translate("chatMessage.jailorDecideExecute", playerListToString(message.targets, playerNames));
            } else {
                return translate("chatMessage.jailorDecideExecute.nobody");
            }
        case "godfatherBackup":
            if (message.backup !== null) {
                return translate("chatMessage.godfatherBackup", playerNames[message.backup]);
            } else {
                return translate("chatMessage.godfatherBackup.nobody");
            }
        /* NIGHT */
        case "godfatherBackupKilled":
            return translate("chatMessage.godfatherBackupKilled", playerNames[message.backup]);
        case "roleBlocked":
            return translate("chatMessage.roleBlocked" + (message.immune ? ".immune" : ""));
        case "sheriffResult":
            return translate("chatMessage.sheriffResult." + (message.suspicious ? "suspicious" : "innocent"));
        case "lookoutResult":
            if (message.players.length === 0) {
                return translate("chatMessage.lookoutResult.nobody");
            } else {
                return translate("chatMessage.lookoutResult", playerListToString(message.players, playerNames));
            }
        case "spyMafiaVisit":
            if (message.players.length === 0) {
                return translate("chatMessage.spyMafiaVisit.nobody");
            } else {
                return translate("chatMessage.spyMafiaVisit", playerListToString(message.players, playerNames));
            }
        case "spyCultistCount":
            if(message.count === 1){
                return translate("chatMessage.spyCultistCount.one");
            }else{
                return translate("chatMessage.spyCultistCount", message.count);
            }
        case "spyBug":
            return translate("chatMessage.spyBug."+message.bug);
        case "trackerResult":
            if (message.players.length === 0) {
                return translate("chatMessage.trackerResult.nobody");
            } else {
                return translate("chatMessage.trackerResult", playerListToString(message.players, playerNames));
            }
        case "seerResult":
            return translate("chatMessage.seerResult." + (message.enemies ? "enemies" : "friends"));
        case "psychicEvil":
            return translate("chatMessage.psychicEvil", playerListToString(message.players, playerNames));
        case "psychicGood":
            return translate("chatMessage.psychicGood", playerListToString(message.players, playerNames));
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
                    : translate("chatMessage.consigliereResult.visited", playerListToString(message.visited, playerNames)),
                visitedByNobody 
                    ? translate("chatMessage.consigliereResult.visitedBy.nobody") 
                    : translate("chatMessage.consigliereResult.visitedBy", playerListToString(message.visitedBy, playerNames))
            );
        case "silenced":
            return translate("chatMessage.silenced");
        case "mediumSeance":
            return translate("chatMessage.mediumSeance", playerNames[message.player]);
        case "youWerePossessed":
            return translate("chatMessage.youWerePossessed" + (message.immune ? ".immune" : ""));
        case "werewolfTrackingResult":
            if(message.players.length === 0){
                return translate(
                    "chatMessage.werewolfTrackingResult.nobody", 
                    playerNames[message.trackedPlayer]
                );
            }else{
                return translate("chatMessage.werewolfTrackingResult", 
                    playerNames[message.trackedPlayer],
                    playerListToString(message.players, playerNames)
                );
            }
        case "cultSacrificesRequired":
            switch (message.required) {
                case 0:
                    return translate("chatMessage.cultSacrificesRequired.0");
                case 1:
                    return translate("chatMessage.cultSacrificesRequired.1");
                default:
                    return translate("chatMessage.cultSacrificesRequired", message.required);
            }
        case "playerWithNecronomicon":
            return translate("chatMessage.playerWithNecronomicon", playerNames[message.playerIndex]);
        case "deputyShotYou":
        case "deathCollectedSouls":
        case "targetWasAttacked":
        case "youWereProtected":
        case "executionerWon":
        case "gameOver":
        case "jesterWon":
        case "targetJailed":
        case "yourConvertFailed":
        case "apostleCanConvertTonight":
        case "apostleCantConvertTonight":
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
        case "martyrFailed":
        case "martyrWon":
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
    chatGroup: "all" | "dead" | "mafia" | "cult" | "seance" | "jail" | "interview"
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
    type: "yourConvertFailed"
} | {
    type: "apostleCanConvertTonight"
} | {
    type: "apostleCantConvertTonight"
} | {
    type: "cultSacrificesRequired"
    required: number
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
    type: "spyCultistCount",
    count: number
} | {
    type: "spyBug", 
    bug: "silenced" | "roleblocked" | "protected" | "transported" | "possessed"
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
    type: "godfatherBackupKilled",
    backup: PlayerIndex
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
} | {
    type: "martyrFailed"
} | {
    type: "martyrWon"
} | {
    type: "martyrRevealed",
    martyr: PlayerIndex
}

export type MessageSender = {
    type: "player", 
    player: PlayerIndex
} | {
    type: "jailor" | "medium" | "journalist"
}
