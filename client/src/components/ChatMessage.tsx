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
        // Could I write a regex that combines both of these? Yes. Will I? No.
        if (
            find(GAME_MANAGER.gameState.myName ?? "").test(message.text) ||
            (
                GAME_MANAGER.gameState.myIndex !== null &&
                find("" + (GAME_MANAGER.gameState.myIndex + 1)).test(message.text)
            )
            
        ) {
            style += " mention";
        } 
    } else if (message.type === "retributionistBug") {
        style += " result"
        return <>
            <StyledText className={"chat-message " + style}>{text}</StyledText>
            <ChatElement message={message.message}/>
        </>
    } else if (message.type === "playerDied") {
        // Who knew rendering an entire grave in the chat would be difficult?
        return <>
            <StyledText className={"chat-message " + style}>{text}</StyledText>
            {message.grave.will.length !== 0 
                && <StyledText className={"chat-message will"}>{message.grave.will}</StyledText>}
            {message.grave.deathNotes.length !== 0 && message.grave.deathNotes.map(note => <>
                <StyledText className={"chat-message " + style}>{translate("chatmessage.deathNote")}</StyledText>
                <StyledText className={"chat-message deathNote"}>{note}</StyledText>
            </>)}
        </>
    }

    return <StyledText className={"chat-message " + style}>{text}</StyledText>;
}

export function translateChatMessage(message: ChatMessage): string {
    switch (message.type) {
        case "normal":
            if(message.messageSender.type === "player"){
                return translate("chatmessage.normal",
                    GAME_MANAGER.gameState.players[message.messageSender.player].toString(), 
                    message.text
                );
            } else {
                // TODO: This only works because Jailor and Medium are the only other message sender types.
                return translate("chatmessage.normal",
                    translate("role."+message.messageSender.type+".name"),
                    message.text
                );
            }
        case "whisper":
            return translate("chatmessage.whisper", 
                GAME_MANAGER.gameState.players[message.fromPlayerIndex].toString(),
                GAME_MANAGER.gameState.players[message.toPlayerIndex].toString(),
                message.text
            );
        case "broadcastWhisper":
            return translate("chatmessage.broadcastWhisper",
                GAME_MANAGER.gameState.players[message.whisperer].toString(),
                GAME_MANAGER.gameState.players[message.whisperee].toString(),
            );
        case "roleAssignment":
            return translate("chatmessage.roleAssignment", 
                translate("role." + message.role + ".name")
            );
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

            return translate(message.grave.will.length === 0 ? "chatmessage.playerDied.noWill": "chatmessage.playerDied",
                GAME_MANAGER.gameState.players[message.grave.playerIndex].toString(),
                graveRoleString,
                deathCause
            );
        case "youDied":
            return translate("chatmessage.youDied");
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
            if (message.votee !== null) {
                return translate("chatmessage.voted",
                    GAME_MANAGER.gameState.players[message.voter].toString(),
                    GAME_MANAGER.gameState.players[message.votee].toString(),
                );
            } else {
                return translate("chatmessage.voted.cleared",
                    GAME_MANAGER.gameState.players[message.voter].toString(),
                );
            }
        case "playerOnTrial":
            return translate("chatmessage.playerOnTrial",
                GAME_MANAGER.gameState.players[message.playerIndex].toString(),
            );
        case "judgementVote":
            return translate("chatmessage.judgementVote",
                GAME_MANAGER.gameState.players[message.voterPlayerIndex].toString(),
            );
        case "judgementVerdict":
            return translate("chatmessage.judgementVerdict",
                GAME_MANAGER.gameState.players[message.voterPlayerIndex].toString(),
                translate("verdict."+message.verdict.toLowerCase())
            );
        case "trialVerdict":
            return translate("chatmessage.trialVerdict",
                GAME_MANAGER.gameState.players[message.playerOnTrial].toString(),
                message.innocent>=message.guilty?translate("verdict.innocent"):translate("verdict.guilty"),
                message.innocent,
                message.guilty
            );
        case "targeted":
            if (message.targets.length > 0) {
                return translate("chatmessage.targeted",
                    GAME_MANAGER.gameState.players[message.targeter].toString(),
                    message.targets.map((target) => GAME_MANAGER.gameState.players[target].toString()).join(", ")
                );
            } else {
                return translate("chatmessage.targeted.cleared",
                    GAME_MANAGER.gameState.players[message.targeter].toString(),
                );
            }
        case "mayorRevealed":
            return translate("chatmessage.mayorRevealed",
                GAME_MANAGER.gameState.players[message.playerIndex].toString(),
            );
        case "jailedTarget":
            return translate("chatmessage.jailedTarget",
                GAME_MANAGER.gameState.players[message.playerIndex].toString(),
            );
        case "jailedSomeone":
            return translate("chatmessage.jailedSomeone",
                GAME_MANAGER.gameState.players[message.playerIndex].toString()
            );
        case "deputyKilled":
            return translate("chatmessage.deputyKilled",
                GAME_MANAGER.gameState.players[message.shotIndex].toString()
            );
        case "jailorDecideExecute":
            if (message.targets.length > 0) {
                return translate("chatmessage.jailorDecideExecute",
                    message.targets.map((target) => GAME_MANAGER.gameState.players[target].toString()).join(", ")
                );
            } else {
                return translate("chatmessage.jailorDecideExecute.nobody");
            }
        /* NIGHT */
        case "roleBlocked":
            return translate("chatmessage.roleBlocked" + (message.immune ? ".immune" : ""));
        case "sheriffResult":
            return translate("chatmessage.sheriffResult." + (message.suspicious ? "suspicious" : "innocent"));
        case "lookoutResult":
            if (message.players.length === 0) {
                return translate("chatmessage.lookoutResult.nobody");
            } else {
                return translate("chatmessage.lookoutResult", 
                    message.players.map(playerIndex => 
                        GAME_MANAGER.gameState.players[playerIndex].toString()
                    ).join(", ")
                );
            }
        case "spyMafiaVisit":
            if (message.players.length === 0) {
                return translate("chatmessage.spyMafiaVisit.nobody");
            } else {
                return translate("chatmessage.spyMafiaVisit", 
                    message.players.map(playerIndex => 
                        GAME_MANAGER.gameState.players[playerIndex].toString()
                    ).join(", ")
                );
            }
        case "spyCovenVisit":
            if (message.players.length === 0) {
                return translate("chatmessage.spyCovenVisit.nobody");
            } else {
                return translate("chatmessage.spyCovenVisit", 
                    message.players.map(playerIndex => 
                        GAME_MANAGER.gameState.players[playerIndex].toString()
                    ).join(", ")
                );
            }
        case "spyBug":
            return translate("chatmessage.spyBug."+message.bug);
        case "trackerResult":
            if (message.players.length === 0) {
                return translate("chatmessage.trackerResult.nobody");
            } else {
                return translate("chatmessage.trackerResult", 
                    message.players.map(playerIndex => 
                        GAME_MANAGER.gameState.players[playerIndex].toString()
                    ).join(", ")
                );
            }
        case "seerResult":
            return translate("chatmessage.seerResult." + (message.enemies ? "enemies" : "friends"));
        case "retributionistBug":
            return translate("chatmessage.retributionistBug");
        case "playerRoleAndWill":
            return translate("chatmessage.playersRoleAndWill", 
                translate("role."+message.role+".name"), 
                message.will
            );
        case "consigliereResult":
            const visitedNobody = message.visited.length === 0;
            const visitedByNobody = message.visitedBy.length === 0;

            return translate("chatmessage.consigliereResult", 
                translate("chatmessage.consigliereResult.role", translate("role."+message.role+".name")),
                visitedNobody 
                    ? translate("chatmessage.consigliereResult.visited.nobody") 
                    : translate("chatmessage.consigliereResult.visited",
                        message.visited.map((playerIndex) => GAME_MANAGER.gameState.players[playerIndex].toString()).join(", "), 
                    ),
                visitedByNobody 
                    ? translate("chatmessage.consigliereResult.visitedBy.nobody") 
                    : translate("chatmessage.consigliereResult.visitedBy",
                        message.visitedBy.map((playerIndex) => GAME_MANAGER.gameState.players[playerIndex].toString()).join(", "), 
                    )
            );
        /* Messages that players must not miss */
        case "silenced":
            return translate("chatmessage.silenced");
        case "mediumSeance":
            return translate("chatmessage.mediumSeance", GAME_MANAGER.gameState.players[message.player].toString());
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
        case "doomsayerFailed":
        case "doomsayerWon":
            return translate("chatmessage."+message.type);
        default:
            console.error("Unknown message type " + (message as any).type + ":");
            console.error(message);
            return "FIXME: " + translate("chatmessage." + message);
    }
}

export type ChatMessage = {
    type: "normal", 
    messageSender: MessageSender, 
    text: string, 
    chatGroup: "all" | "mafia" | "dead" | "vampire" | "coven"
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
    type: "youDied"
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
    type: "jailedSomeone",
    playerIndex: PlayerIndex
} | {
    type: "jailedTarget"
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
    type: "spyCovenVisit", 
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
    type: "retributionistBug", 
    message: ChatMessage
} | {
    type: "veteranAttackedYou"
} | {
    type: "veteranAttackedVisitor"
} | {
    type: "vigilanteSuicide"
} | {
    type: "doctorHealed"
} | {
    type: "bodyguardProtected"
} | {
    type: "protectedYou"
} | {
    type: "transported"
} | {
    type: "godfatherForcedMafioso"
} | {
    type: "godfatherForcedYou"
} | {
    type: "silenced"
} | {
    type: "framerFramedPlayers", 
    players: PlayerIndex[]
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
    type: "witchBug", 
    message: ChatMessage
} | {
    type: "arsonistCleanedSelf"
} | {
    type: "arsonistDousedPlayers", 
    players: PlayerIndex[]
} | {
    type: "jesterWon"
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
