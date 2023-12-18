
import { createGameState, createLobbyState, createPlayer } from "./gameState";
import Anchor from "./../menu/Anchor";
import LobbyMenu from "./../menu/lobby/LobbyMenu";
import StartMenu from "./../menu/main/StartMenu";
import GAME_MANAGER from "./../index";
import GameScreen, { ContentMenus } from "./../menu/game/GameScreen";
import React from "react";
import { ToClientPacket } from "./packet";
import { Tag } from "./gameState.d";
import { Role } from "./roleState.d";

export default function messageListener(packet: ToClientPacket){

    // console.log(JSON.stringify(packet, null, 2));
    console.log(packet.type);


    switch(packet.type) {
        case "acceptJoin":
            if(packet.inGame){
                GAME_MANAGER.state = createGameState();
                Anchor.setContent(GameScreen.createDefault());
            }else{
                GAME_MANAGER.state = createLobbyState();
                Anchor.setContent(<LobbyMenu/>);
            }
            GAME_MANAGER.roomCode = packet.roomCode.toString(18);
            if(GAME_MANAGER.state.stateType === "lobby")
                GAME_MANAGER.state.myId = packet.playerId;
        break;
        case "rejectJoin":
            switch(packet.reason) {
                case "INVALID_ROOM_CODE":
                    Anchor.pushInfo("Couldn't join", "No lobby has that room code!");
                break;
                case "GAME_ALREADY_STARTED":
                    Anchor.pushInfo("Couldn't join", "That game has already begun!");
                break;
                case "ROOM_FULL":
                    Anchor.pushInfo("Couldn't join", "That lobby is full!");
                break;
                case "SERVER_BUSY":
                    Anchor.pushInfo("Couldn't join", "The server is busy. Try again later!");
                break;
                default:
                    Anchor.pushInfo("Couldn't join", "Failed to join the lobby. Try again later!");
                    console.log("incoming message response not implemented " + packet.type + ": " + packet.reason);
                    console.log(packet);
                break;
            }
            Anchor.setContent(<StartMenu/>);
        break;
        case "rejectStart":
            switch(packet.reason) {
                case "GameEndsInstantly":
                    Anchor.pushInfo("Couldn't start", "Game would end instantly! Make sure your role list is valid.");
                break;
                case "ZeroTimeGame":
                    Anchor.pushInfo("Couldn't start", "Make sure your phase time settings are valid!");
                break;
                default:
                    Anchor.pushInfo("Couldn't start", "Failed to start lobby. Try again later!");
                    console.log("incoming message response not implemented " + packet.type + ": " + packet.reason);
                    console.log(packet);
                break;
            }
        break;
        case "playersHost":
            if(GAME_MANAGER.state.stateType === "lobby"){
                for(let [playerId, player] of GAME_MANAGER.state.players){
                    player.host = packet.hosts.includes(playerId);
                }
            }
        break;
        /*
        In Lobby/Game 
        */
        case "yourId":
            if(GAME_MANAGER.state.stateType === "lobby")
                GAME_MANAGER.state.myId = packet.playerId;
        break;
        case "yourPlayerIndex":
            if(GAME_MANAGER.state.stateType === "game")
                GAME_MANAGER.state.myIndex = packet.playerIndex;
        break;
        case "lobbyPlayers":
            if(GAME_MANAGER.state.stateType === "lobby"){
                GAME_MANAGER.state.players = new Map();
                for(let [playerId, name] of Object.entries(packet.players)){
                    GAME_MANAGER.state.players.set(Number.parseInt(playerId), {name: name, host: false});
                }
                
            }
        break;
        case "kickPlayer":
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            //COMPLETLY BROKEN
            if(GAME_MANAGER.state.stateType === "lobby" && packet.playerId === GAME_MANAGER.state.myId){
                GAME_MANAGER.leaveGame();
            }
            // GAME_MANAGER.gameState = createGameState();
            // Anchor.setContent(<StartMenu/>)
        break;
        case "startGame":
            GAME_MANAGER.state = createGameState();
            Anchor.setContent(GameScreen.createDefault());
        break;
        case "gamePlayers":
            if(GAME_MANAGER.state.stateType === "game"){
                // GAME_MANAGER.state.players = [];
                // for(let i = 0; i < packet.players.length; i++){
                //     GAME_MANAGER.state.players.push(createPlayer(packet.players[i], i));
                // }

                //only update the playerlist with the new one if there are any differences
                let playersChanged = false;
                if(GAME_MANAGER.state.players.length !== packet.players.length)
                    playersChanged = true;
                else{
                    for(let i = 0; i < packet.players.length; i++){
                        if(GAME_MANAGER.state.players[i].name !== packet.players[i]){
                            playersChanged = true;
                            break;
                        }
                    }
                }
                if(playersChanged){
                    GAME_MANAGER.state.players = [];
                    for(let i = 0; i < packet.players.length; i++){
                        GAME_MANAGER.state.players.push(createPlayer(packet.players[i], i));
                    }
                }

            }
        break;
        case "roleList":
            //list of role list entriy
            if(GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game")
                GAME_MANAGER.state.roleList = packet.roleList;
        break;
        case "roleOutline":
            //role list entriy
            if(GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game")
                GAME_MANAGER.state.roleList[packet.index] = packet.roleOutline;
        break;
        case "phaseTime":
            if(GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game")
                GAME_MANAGER.state.phaseTimes[packet.phase as keyof typeof GAME_MANAGER.state.phaseTimes] = packet.time;
        break;
        case "phaseTimes":
            if(GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game")
                GAME_MANAGER.state.phaseTimes = packet.phaseTimeSettings;
        break;
        case "excludedRoles":
            if(GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game")
                GAME_MANAGER.state.excludedRoles = packet.roles;
        break;
        case "phase":
            if(GAME_MANAGER.state.stateType === "game"){
                GAME_MANAGER.state.phase = packet.phase;
                GAME_MANAGER.state.dayNumber = packet.dayNumber;
                GAME_MANAGER.state.timeLeftMs = packet.secondsLeft * 1000;
            }
        break;
        case "playerOnTrial":
            if(GAME_MANAGER.state.stateType === "game")
                GAME_MANAGER.state.playerOnTrial = packet.playerIndex;
        break;
        case "playerAlive":
            if(GAME_MANAGER.state.stateType === "game"){
                for(let i = 0; i < GAME_MANAGER.state.players.length && i < packet.alive.length; i++){
                    GAME_MANAGER.state.players[i].alive = packet.alive[i];
                }
            }
        break;
        case "playerVotes":
            if(GAME_MANAGER.state.stateType === "game"){
                for(let i = 0; i < GAME_MANAGER.state.players.length; i++){
                    GAME_MANAGER.state.players[i].numVoted = 0;
                }
                for(let [playerIndex, numVoted] of Object.entries(packet.votesForPlayer)){
                    GAME_MANAGER.state.players[Number.parseInt(playerIndex)].numVoted = numVoted;
                }
            }
        break;
        case "yourButtons":
            if(GAME_MANAGER.state.stateType === "game"){
                for(let i = 0; i < GAME_MANAGER.state.players.length && i < packet.buttons.length; i++){
                    GAME_MANAGER.state.players[i].buttons = packet.buttons[i];
                }
            }
        break;
        case "yourRoleLabels":
            if(GAME_MANAGER.state.stateType === "game"){
                for (const [key, value] of Object.entries(packet.roleLabels)) { 
                    if(
                        GAME_MANAGER.state.players !== undefined && 
                        GAME_MANAGER.state.players[Number.parseInt(key)] !== undefined
                    )
                        GAME_MANAGER.state.players[Number.parseInt(key)].roleLabel = value as Role;
                }
            }
        break;
        case "yourPlayerTags":
            if(GAME_MANAGER.state.stateType === "game"){
                for (const [key, value] of Object.entries(packet.playerTags)) { 
                    GAME_MANAGER.state.players[Number.parseInt(key)].playerTags = value as Tag[];
                }
            }
        break;
        case "yourWill":
            if(GAME_MANAGER.state.stateType === "game")
                GAME_MANAGER.state.will = packet.will;
        break;
        case "yourNotes":
            if(GAME_MANAGER.state.stateType === "game")
            GAME_MANAGER.state.notes = packet.notes;
        break;
        case "yourDeathNote":
            if(GAME_MANAGER.state.stateType === "game")
                GAME_MANAGER.state.deathNote = packet.deathNote ?? "";
        break;
        case "yourRoleState":
            if(GAME_MANAGER.state.stateType === "game"){
                if(GAME_MANAGER.state.roleState?.role!== packet.roleState.role){
                    GameScreen.instance?.closeMenu(ContentMenus.RoleSpecificMenu);
                }
                GAME_MANAGER.state.roleState = packet.roleState;
            }
        break;
        case "yourTarget":
            if(GAME_MANAGER.state.stateType === "game")
                GAME_MANAGER.state.targets = packet.playerIndices;
        break;
        case "yourVoting":
            if(GAME_MANAGER.state.stateType === "game")
                GAME_MANAGER.state.voted = packet.playerIndex;
        break;
        case "yourJudgement":
            if(GAME_MANAGER.state.stateType === "game")
                GAME_MANAGER.state.judgement = packet.verdict;
        break;
        case "addChatMessages":
            if(GAME_MANAGER.state.stateType === "game")
                GAME_MANAGER.state.chatMessages = GAME_MANAGER.state.chatMessages.concat(packet.chatMessages);
        break;
        case "addGrave":
            if(GAME_MANAGER.state.stateType === "game")
                GAME_MANAGER.state.graves.push(packet.grave);
        break;
        case "gameOver":
            if(GAME_MANAGER.state.stateType === "game"){
                GAME_MANAGER.state.ticking = false;
                switch(packet.reason) {
                    case "ReachedMaxDay":
                        // alert("Game Over: Reached the maximum day!");
                        console.log("incoming message response not implemented " + packet.type + ": " + packet.reason);
                        console.log(packet);
                    break;
                    default:
                        // alert("Game ended for an unknown reason!");
                        console.log("incoming message response not implemented " + packet.type + ": " + packet.reason);
                        console.log(packet);
                    break;
                }
            }
        break;
        default:
            console.log("incoming message response not implemented " + packet);
            console.log(packet);
        break;
    }

    GAME_MANAGER.invokeStateListeners(packet.type);
}


