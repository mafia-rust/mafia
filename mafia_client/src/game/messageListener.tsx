
import { create_gameState, create_grave, create_player } from "./gameState";
import Anchor from "../menu/Anchor";
import LobbyMenu from "../menu/lobby/LobbyMenu";
import StartMenu from "../menu/main/StartMenu";
import GAME_MANAGER from "../index";
import GameScreen from "../menu/game/GameScreen";
import React from "react";
import { Phase } from "./gameState.d";

export default function messageListener(serverMessage: any){

    let type;
    if(typeof(serverMessage)==="string"){
        type = serverMessage;
    }else{
        //object, THIS ASSUMES THAT SERVER MESSAGE IS AN OBJECT WITH AT LEAST 1 KEY
        type = Object.keys(serverMessage)[0];
        serverMessage = serverMessage[type];
    }


    //In each of the cases, ensure that your not interpreting anything as an object when it isnt.
    //on the rust side, this is an enum called ToClientPacket
    switch(type) {
        case "AcceptJoin":
            Anchor.setContent(<LobbyMenu/>);
        break;
        case "RejectJoin":
            switch(serverMessage.reason) {
                case "InvalidRoomCode":
                    alert("Couldn't join: No lobby has that room code!");
                break;
                case "GameAlreadyStarted":
                    alert("Couldn't join: That game has already begun!");
                break;
                case "RoomFull":
                    alert("Couldn't join: That lobby is full!");
                break;
                default:
                    alert("Couldn't join lobby for an unknown reason!");
                    console.log("incoming_message response not implemented "+type+": "+serverMessage.reason);
                    console.log(serverMessage);
                break;
            }
            Anchor.setContent(<StartMenu/>);
        break;
        case "AcceptHost":
            GAME_MANAGER.roomCode = serverMessage.room_code;
            Anchor.setContent(<LobbyMenu/>);
        break;

        //InLobby/Game

        
        case"YourName":
            GAME_MANAGER.gameState.myName = serverMessage.name;
        break;
        case"YourPlayerIndex":
            GAME_MANAGER.gameState.myIndex = serverMessage.player_index;
        break;
        case"Players":
            for(let i = 0; i < serverMessage.names.length; i++){
                if(GAME_MANAGER.gameState.players.length > i){
                    GAME_MANAGER.gameState.players[i].name = serverMessage.names[i];
                }else{
                    //if this player index isnt in the list, create a new player and then sync
                    GAME_MANAGER.gameState.players.push(create_player(serverMessage.names[i], i));
                }
            }
        break;
        case"Kicked":
            GAME_MANAGER.gameState = create_gameState();
            Anchor.setContent(<StartMenu/>)
        break;
        case "OpenGameMenu":
            Anchor.setContent(<GameScreen/>);
        break;
        case "RoleList":
            //list of role list entriy
            GAME_MANAGER.gameState.roleList = serverMessage.role_list.role_list;
        break;
        case"PhaseTime":
            GAME_MANAGER.gameState.phaseTimes[serverMessage.phase as Phase] = serverMessage.time;
        break;
        case"InvestigatorResults":
            GAME_MANAGER.gameState.investigatorResults = serverMessage.investigator_results.results;
        break;
        case"Phase":
            GAME_MANAGER.gameState.phase = serverMessage.phase;
            GAME_MANAGER.gameState.dayNumber = serverMessage.day_number;
            GAME_MANAGER.gameState.secondsLeft = serverMessage.seconds_left;
        break;
        case"PlayerOnTrial":
            GAME_MANAGER.gameState.playerOnTrial = serverMessage.player_index;
        break;
        case"PlayerButtons":
            for(let i = 0; i < GAME_MANAGER.gameState.players.length && i < serverMessage.buttons.length; i++){
                GAME_MANAGER.gameState.players[i].buttons.vote = serverMessage.buttons[i].vote;
                GAME_MANAGER.gameState.players[i].buttons.target = serverMessage.buttons[i].target;
                GAME_MANAGER.gameState.players[i].buttons.dayTarget = serverMessage.buttons[i].day_target;
            }
        break;
        case"PlayerAlive":
            for(let i = 0; i < GAME_MANAGER.gameState.players.length && i < serverMessage.alive.length; i++){
                GAME_MANAGER.gameState.players[i].alive = serverMessage.alive[i];
            }
        break;
        case"PlayerVotes":
            for(let i = 0; i < GAME_MANAGER.gameState.players.length && i < serverMessage.voted_for_player.length; i++){
                GAME_MANAGER.gameState.players[i].numVoted = serverMessage.voted_for_player[i];
            }
        break;
        case"YourWill":
            GAME_MANAGER.gameState.will = serverMessage.will;
        break;
        case"YourRole":
            if(typeof(serverMessage.role)==="string"){
                GAME_MANAGER.gameState.role = serverMessage.role;
            }else{
                GAME_MANAGER.gameState.role = Object.keys(serverMessage.role)[0];
            }
        break;
        case"YourTarget":
            GAME_MANAGER.gameState.targets = serverMessage.player_indices;
        break;
        case"YourVoting":
            GAME_MANAGER.gameState.voted = serverMessage.player_index;
        break;
        case"YourJudgement":
            GAME_MANAGER.gameState.judgement = serverMessage.verdict;
        break;
        case"AddChatMessages":
            for(let i = 0; i < serverMessage.chat_messages.length; i++){
                GAME_MANAGER.gameState.chatMessages.push(serverMessage.chat_messages[i]);
            }
        break;
        case"AddGrave":
            let grave = create_grave();
            grave.playerIndex = serverMessage.grave.player_index;
            grave.role =        serverMessage.grave.role;
            grave.death_cause = serverMessage.grave.death_cause;
            grave.will =        serverMessage.grave.will;
            grave.diedPhase =   serverMessage.grave.died_phase;
            grave.dayNumber =   serverMessage.grave.day_number;

            GAME_MANAGER.gameState.graves.push(grave);
        break;
        default:
            console.log("incoming_message response not implemented "+type);
            console.log(serverMessage);
        break;
    }


    
    GAME_MANAGER.invokeStateListeners(type);
}


