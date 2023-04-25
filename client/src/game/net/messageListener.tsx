
import { createGameState, createPlayer } from "../gameState";
import Anchor from "../../menu/Anchor";
import * as LobbyMenu from "../../menu/lobby/LobbyMenu";
import StartMenu from "../../menu/main/StartMenu";
import GAME_MANAGER from "../../index";
import GameScreen from "../../menu/game/GameScreen";
import React from "react";
import { ToClientPacket } from "./packet";

export default function messageListener(packet: ToClientPacket){
    switch(packet.type) {
        case "acceptJoin":
            Anchor.setContent(LobbyMenu.create());
        break;
        case "rejectJoin":
            switch(packet.reason) {
                case "InvalidRoomCode":
                    alert("Couldn't join: No lobby has that room code!");
                break;
                case "GameAlreadyStarted":
                    alert("Couldn't join: That game has already begun!");
                break;
                case "RoomFull":
                    alert("Couldn't join: That lobby is full!");
                break;
                case "ServerBusy":
                    alert("Couldn't join: The server is busy. Try again later!");
                break;
                default:
                    alert("Couldn't join lobby for an unknown reason!");
                    console.log("incoming message response not implemented " + packet.type + ": " + packet.reason);
                    console.log(packet);
                break;
            }
            Anchor.setContent(<StartMenu/>);
        break;
        case "rejectStart":
            switch(packet.reason) {
                case "GameEndsInstantly":
                    alert("Couldn't start: Game would end instantly!");
                break;
                case "ZeroTimeGame":
                    alert("Couldn't start: There must be at least one phase!");
                break;
                default:
                    alert("Couldn't start lobby for an unknown reason!");
                    console.log("incoming message response not implemented " + packet.type + ": " + packet.reason);
                    console.log(packet);
                break;
            }
        break;
        case "acceptHost":
            GAME_MANAGER.roomCode = packet.roomCode.toString(18);
            Anchor.setContent(LobbyMenu.create());
        break;

        //InLobby/Game

        
        case "yourName":
            GAME_MANAGER.gameState.myName = packet.name;
        break;
        case "yourPlayerIndex":
            GAME_MANAGER.gameState.myIndex = packet.playerIndex;
        break;
        case "players":
            for(let i = 0; i < packet.names.length; i++){
                if(GAME_MANAGER.gameState.players.length > i){
                    GAME_MANAGER.gameState.players[i].name = packet.names[i];
                }else{
                    //if this player index isnt in the list, create a new player and then sync
                    GAME_MANAGER.gameState.players.push(createPlayer(packet.names[i], i));
                }
            }
        break;
        case "kicked":
            GAME_MANAGER.gameState = createGameState();
            Anchor.setContent(<StartMenu/>)
        break;
        case "startGame":
            Anchor.setContent(<GameScreen/>);
        break;
        case "roleList":
            //list of role list entriy
            GAME_MANAGER.gameState.roleList = packet.roleList;
        break;
        case "phaseTime":
            GAME_MANAGER.gameState.phaseTimes[packet.phase] = packet.time;
        break;
        case "investigatorResults":
            GAME_MANAGER.gameState.investigatorResults = packet.investigatorResults;
        break;
        case "phase":
            GAME_MANAGER.gameState.phase = packet.phase;
            GAME_MANAGER.gameState.dayNumber = packet.dayNumber;
            GAME_MANAGER.gameState.secondsLeft = packet.secondsLeft;
        break;
        case "playerOnTrial":
            GAME_MANAGER.gameState.playerOnTrial = packet.playerIndex;
        break;
        case "playerButtons":
            for(let i = 0; i < GAME_MANAGER.gameState.players.length && i < packet.buttons.length; i++){
                GAME_MANAGER.gameState.players[i].buttons = packet.buttons[i];
            }
        break;
        case "playerAlive":
            for(let i = 0; i < GAME_MANAGER.gameState.players.length && i < packet.alive.length; i++){
                GAME_MANAGER.gameState.players[i].alive = packet.alive[i];
            }
        break;
        case "playerVotes":
            for(let i = 0; i < GAME_MANAGER.gameState.players.length && i < packet.votedForPlayer.length; i++){
                GAME_MANAGER.gameState.players[i].numVoted = packet.votedForPlayer[i];
            }
        break;
        case "yourWill":
            GAME_MANAGER.gameState.will = packet.will;
        break;
        case "yourRole":
            if(typeof(packet.role)==="string"){
                GAME_MANAGER.gameState.role = packet.role;
            }else{
                GAME_MANAGER.gameState.role = Object.keys(packet.role)[0];
            }
        break;
        case "yourTarget":
            GAME_MANAGER.gameState.targets = packet.playerIndices;
        break;
        case "yourVoting":
            GAME_MANAGER.gameState.voted = packet.playerIndex;
        break;
        case "yourJudgement":
            GAME_MANAGER.gameState.judgement = packet.verdict;
        break;
        case "addChatMessages":
            for(let i = 0; i < packet.chatMessages.length; i++){
                GAME_MANAGER.gameState.chatMessages.push(packet.chatMessages[i]);
            }
        break;
        case "addGrave":
            GAME_MANAGER.gameState.graves.push(packet.grave);
        break;
        case "gameOver":
            switch(packet.reason) {
                case "ReachedMaxDay":
                    alert("Game Over: Reached the maximum day!");
                break;
                default:
                    alert("Game ended for an unknown reason!");
                    console.log("incoming message response not implemented " + packet.type + ": " + packet.reason);
                    console.log(packet);
                break;
            }
        break;
        default:
            console.log("incoming message response not implemented " + packet);
            console.log(packet);
        break;
    }

    GAME_MANAGER.invokeStateListeners(packet.type);
}


