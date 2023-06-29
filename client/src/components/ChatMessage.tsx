import { ReactElement } from "react";
import { ChatMessage } from "../game/chatMessage";
import translate from "../game/lang";
import React from "react";
import GAME_MANAGER from "..";
import StyledText from "./StyledText";
import "./chatMessage.css"

export default function ChatElement(props: {message: ChatMessage}): ReactElement {
    const message = props.message;
    const chatMessageStyles = require("../resources/styling/chatMessage.json");
    let style = typeof chatMessageStyles[message.type] === "string" ? chatMessageStyles[message.type] : "";
    
    // Special chat messages that don't play by the rules
    if (message.type === "normal") {
        if(message.messageSender.type !== "player"){
            style = "discreet";
        } else if (message.chatGroup === "dead") {
            style = "dead player";
        } else {
            style = "player"
        }
    } else if (message.type === "retributionistBug") {
        return <>
            <StyledText className="chat-message result">{translate("chatmessage.retributionistBug")}</StyledText>
            <ChatElement message={message.message}/>
        </>
    }

    return <StyledText className={"chat-message " + style}>{
        translateChatMessage(message)
    }</StyledText>;
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

            return translate("chatmessage.playerDied",
                GAME_MANAGER.gameState.players[message.grave.playerIndex].toString(),
                graveRoleString,
                deathCause,
                message.grave.will
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
            return "";
        case "playerRoleAndWill":
            return translate("chatmessage.playersRoleAndWill", 
                translate("role."+message.role+".name"), 
                message.will
            );
        case "consigliereResult":
            return translate("chatmessage.consigliereResult", 
                translate("role."+message.role+".name"),
                (message.visited.map((playerIndex) => GAME_MANAGER.gameState.players[playerIndex].toString()).join(", ")), 
                (message.visitedBy.map((playerIndex) => GAME_MANAGER.gameState.players[playerIndex].toString()).join(", "))
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
        case "doomsayerFailed":
        case "doomsayerWon":
            return translate("chatmessage."+message.type);
        default:
            console.error("Unknown message type " + (message as any).type + ":");
            console.error(message);
            return "FIXME: " + translate("chatmessage." + message);
    }
}