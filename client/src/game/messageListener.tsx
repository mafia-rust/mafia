
import { createGameState, createPlayer } from "./gameState";
import Anchor from "./../menu/Anchor";
import LobbyMenu from "./../menu/lobby/LobbyMenu";
import StartMenu from "./../menu/main/StartMenu";
import GAME_MANAGER from "./../index";
import GameScreen from "./../menu/game/GameScreen";
import React from "react";
import { ToClientPacket } from "./packet";
import { Role } from "./gameState.d";

export default function messageListener(packet: ToClientPacket){

    console.log(packet);
    switch(packet.type) {
        case "acceptJoin":
            if(packet.inGame){
                Anchor.setContent(<GameScreen/>);
            }else{
                Anchor.setContent(<LobbyMenu/>);
            }
        break;
        case "rejectJoin":
            switch(packet.reason) {
                case "INVALID_ROOM_CODE":
                    alert("Couldn't join: No lobby has that room code!");
                break;
                case "GAME_ALREADY_STARTED":
                    alert("Couldn't join: That game has already begun!");
                break;
                case "ROOM_FULL":
                    alert("Couldn't join: That lobby is full!");
                break;
                case "SERVER_BUSY":
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
            Anchor.setContent(<LobbyMenu/>);
        break;

        //InLobby/Game

        
        case "yourName":
            GAME_MANAGER.gameState.myName = packet.name;
        break;
        case "yourPlayerIndex":
            GAME_MANAGER.gameState.myIndex = packet.playerIndex;
        break;
        case "players":
            GAME_MANAGER.gameState.players = [];
            for(let i = 0; i < packet.names.length; i++){
                if(GAME_MANAGER.gameState.players.length > i){
                    GAME_MANAGER.gameState.players[i].name = packet.names[i];
                }else{
                    //if this player index isnt in the already made list, create a new player and then sync
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
            GAME_MANAGER.gameState.phaseTimes[packet.phase as keyof typeof GAME_MANAGER.gameState.phaseTimes] = packet.time;
        break;
        case "investigatorResults":
            GAME_MANAGER.gameState.investigatorResults = packet.investigatorResults.results;
        break;
        case "phase":
            GAME_MANAGER.gameState.phase = packet.phase;
            GAME_MANAGER.gameState.dayNumber = packet.dayNumber;
            GAME_MANAGER.gameState.secondsLeft = packet.secondsLeft;
        break;
        case "playerOnTrial":
            GAME_MANAGER.gameState.playerOnTrial = packet.playerIndex;
        break;
        case "playerAlive":
            for(let i = 0; i < GAME_MANAGER.gameState.players.length && i < packet.alive.length; i++){
                GAME_MANAGER.gameState.players[i].alive = packet.alive[i];
            }
        break;
        case "playerVotes":
            for(let i = 0; i < GAME_MANAGER.gameState.players.length; i++){
                GAME_MANAGER.gameState.players[i].numVoted = 0;
            }
            for(let [playerIndex, numVoted] of Object.entries(packet.votedForPlayer)){
                GAME_MANAGER.gameState.players[Number.parseInt(playerIndex)].numVoted = numVoted;
            }
        break;
        case "yourButtons":
            for(let i = 0; i < GAME_MANAGER.gameState.players.length && i < packet.buttons.length; i++){
                GAME_MANAGER.gameState.players[i].buttons = packet.buttons[i];
            }
        break;
        case "yourRoleLabels":
            for (const [key, value] of Object.entries(packet.roleLabels)) { 
                GAME_MANAGER.gameState.players[Number.parseInt(key)].roleLabel = value as Role;
            }
        break;
        case "yourWill":
            GAME_MANAGER.gameState.will = packet.will;
        break;
        case "yourNotes":
            GAME_MANAGER.gameState.notes = packet.notes;
        break;
        case "yourRole":
            if(typeof(packet.role)==="string"){
                GAME_MANAGER.gameState.role = packet.role;
            }else{
                GAME_MANAGER.gameState.role = Object.keys(packet.role)[0];
            }
        break;
        case "yourRoleData":
            GAME_MANAGER.gameState.role = packet.roleData.role;
            GAME_MANAGER.gameState.roleData = packet.roleData;
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
            GAME_MANAGER.gameState.chatMessages = GAME_MANAGER.gameState.chatMessages.concat(packet.chatMessages);
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


