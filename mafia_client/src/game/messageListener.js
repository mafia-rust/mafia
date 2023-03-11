
import { create_gameState, create_grave, create_player } from "./gameState";
import { Main } from "../Main";
import { LobbyMenu } from "../openMenus/lobby/LobbyMenu";
import { StartMenu } from "../openMenus/StartMenu";
import gameManager from "../index.js";
import { GameScreen } from "../gameMenus/GameScreen";

export function messageListener(serverMessage){

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
            Main.instance.setContent(<LobbyMenu/>);
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
            Main.instance.setContent(<StartMenu/>);
        break;
        case "AcceptHost":
            gameManager.roomCode = serverMessage.room_code;
            Main.instance.setContent(<LobbyMenu/>);
        break;

        //InLobby/Game

        
        case"YourName":
            gameManager.gameState.myName = serverMessage.name;
        break;
        case"YourPlayerIndex":
            gameManager.gameState.myIndex = serverMessage.player_index;
        break;
        case"Players":
            for(let i = 0; i < serverMessage.names.length; i++){
                if(gameManager.gameState.players.length > i){
                    gameManager.gameState.players[i].name = serverMessage.names[i];
                }else{
                    //if this player index isnt in the list, create a new player and then sync
                    gameManager.gameState.players.push(create_player());
                    gameManager.gameState.players[i].name = serverMessage.names[i];
                }
            }
        break;
        case"Kicked":
            gameManager.gameState = create_gameState();
            Main.instance.setContent(<StartMenu/>)
        break;
        case "OpenGameMenu":
            Main.instance.setContent(<GameScreen/>);
        break;
        case"PhaseTimes":
            gameManager.gameState.phaseTimes.morning    = serverMessage.phase_times.morning.secs;
            gameManager.gameState.phaseTimes.discussion = serverMessage.phase_times.discussion.secs;
            gameManager.gameState.phaseTimes.voting     = serverMessage.phase_times.voting.secs;
            gameManager.gameState.phaseTimes.testimony  = serverMessage.phase_times.testimony.secs;
            gameManager.gameState.phaseTimes.judgement  = serverMessage.phase_times.judgement.secs;
            gameManager.gameState.phaseTimes.evening    = serverMessage.phase_times.evening.secs;
            gameManager.gameState.phaseTimes.night      = serverMessage.phase_times.night.secs;
        break;
        case "RoleList":
            gameManager.gameState.roleList = serverMessage.role_list.role_list;
        break;
        case"Phase":
            gameManager.gameState.phase = serverMessage.phase;
            gameManager.gameState.dayNumber = serverMessage.day_number;
            gameManager.gameState.secondsLeft = serverMessage.seconds_left;
        break;
        case"PlayerOnTrial":
            gameManager.gameState.playerOnTrial = serverMessage.player_index;
        break;
        case"PlayerButtons":
            for(let i = 0; i < gameManager.gameState.players.length && i < serverMessage.buttons.length; i++){
                gameManager.gameState.players[i].buttons.vote = serverMessage.buttons[i].vote;
                gameManager.gameState.players[i].buttons.target = serverMessage.buttons[i].target;
                gameManager.gameState.players[i].buttons.dayTarget = serverMessage.buttons[i].day_target;
            }
        break;
        case"PlayerAlive":
            for(let i = 0; i < gameManager.gameState.players.length && i < serverMessage.alive.length; i++){
                gameManager.gameState.players[i].alive = serverMessage.alive[i];
            }
        break;
        case"PlayerVotes":
            for(let i = 0; i < gameManager.gameState.players.length && i < serverMessage.voted_for_player.length; i++){
                gameManager.gameState.players[i].numVoted = serverMessage.voted_for_player[i];
            }
        break;
        case"YourWill":
            gameManager.gameState.will = serverMessage.will;
        break;
        case"YourRole":
            gameManager.gameState.role = serverMessage.role;
        break;
        case"YourTarget":
            gameManager.gameState.targets = serverMessage.player_indices;
        break;
        case"YourVoting":
            gameManager.gameState.voted = serverMessage.player_index;
        break;
        case"YourJudgement":
            gameManager.gameState.judgement = serverMessage.verdict;
        break;
        case"AddChatMessages":
            for(let i = 0; i < serverMessage.chat_messages.length; i++){
                gameManager.gameState.chatMessages.push(serverMessage.chat_messages[i]);
            }
        break;
        case"AddGrave":
            let grave = create_grave();
            grave.playerIndex = serverMessage.grave.player_index;
            grave.role =        serverMessage.grave.role;
            grave.killer =      serverMessage.grave.killer;
            grave.will =        serverMessage.grave.will;
            grave.diedPhase =   serverMessage.grave.died_phase;
            grave.dayNumber =   serverMessage.grave.day_number;

            gameManager.gameState.graves.push(grave);
        break;
        default:
            console.log("incoming_message response not implemented "+type);
            console.log(serverMessage);
        break;
    }


    
    gameManager.invokeStateListeners(type);
}


