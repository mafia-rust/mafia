
import { createPlayer } from "./gameState";
import Anchor, { chatMessageToAudio } from "./../menu/Anchor";
import GAME_MANAGER from "./../index";
import GameScreen, { ContentMenu } from "./../menu/game/GameScreen";
import { ToClientPacket } from "./packet";
import { Tag } from "./gameState.d";
import { Role } from "./roleState.d";
import translate from "./lang";
import { computeKeywordDataWithPlayers } from "../components/StyledText";
import { deleteReconnectData, saveReconnectData } from "./localStorage";
import { WikiArticleLink } from "../components/WikiArticleLink";
import React from "react";
import WikiArticle from "../components/WikiArticle";
import SpectatorGameScreen from "../menu/spectator/SpectatorGameScreen";
import LobbyMenu from "../menu/lobby/LobbyMenu";

export default function messageListener(packet: ToClientPacket){

    console.log(JSON.stringify(packet, null, 2));


    switch(packet.type) {
        case "pong":
            GAME_MANAGER.pingCalculation = Date.now() - GAME_MANAGER.lastPingTime;
            console.log("Ping: "+GAME_MANAGER.pingCalculation);
        break;
        case "rateLimitExceeded":
            Anchor.pushError(translate("notification.rateLimitExceeded"), "");
        break;
        case "lobbyList":
            if(GAME_MANAGER.state.stateType === "outsideLobby"){
                GAME_MANAGER.state.lobbies = new Map();

                for(let [lobbyId, lobbyData] of Object.entries(packet.lobbies))
                    GAME_MANAGER.state.lobbies.set(Number.parseInt(lobbyId), lobbyData);
            }
        break;
        case "acceptJoin":
            if(packet.inGame && packet.spectator){
                GAME_MANAGER.setSpectatorGameState();
                Anchor.setContent(<SpectatorGameScreen/>);
            }else if(packet.inGame && !packet.spectator){
                GAME_MANAGER.setGameState();
                Anchor.setContent(GameScreen.createDefault());
            }else{
                GAME_MANAGER.setLobbyState();
                Anchor.setContent(<LobbyMenu/>);
            }
            

            if(GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game"){
                GAME_MANAGER.state.roomCode = packet.roomCode;
            }
            if(GAME_MANAGER.state.stateType === "lobby")
                GAME_MANAGER.state.myId = packet.playerId;
        

            saveReconnectData(packet.roomCode, packet.playerId);
            Anchor.clearCoverCard();
        break;
        case "rejectJoin":
            switch(packet.reason) {
                case "roomDoesntExist":
                    Anchor.pushError(translate("notification.rejectJoin"), translate("notification.rejectJoin.roomDoesntExist"));
                    // If the room doesn't exist, don't suggest the user to reconnect to it.
                    deleteReconnectData();
                    Anchor.clearCoverCard();
                break;
                case "gameAlreadyStarted":
                    Anchor.pushError(translate("notification.rejectJoin"), translate("notification.rejectJoin.gameAlreadyStarted"));
                break;
                case "roomFull":
                    Anchor.pushError(translate("notification.rejectJoin"), translate("notification.rejectJoin.roomFull"));
                break;
                case "serverBusy":
                    Anchor.pushError(translate("notification.rejectJoin"), translate("notification.rejectJoin.serverBusy"));
                break;
                case "playerTaken":
                    Anchor.pushError(translate("notification.rejectJoin"), translate("notification.rejectJoin.playerTaken"));
                break;
                case "playerDoesntExist":
                    Anchor.pushError(translate("notification.rejectJoin"), translate("notification.rejectJoin.playerDoesntExist"));
                break;
                default:
                    Anchor.pushError(translate("notification.rejectJoin"), `${packet.type} message response not implemented: ${packet.reason}`);
                    console.error(`${packet.type} message response not implemented: ${packet.reason}`);
                    console.error(packet);
                break;
            }
            deleteReconnectData();
            
        break;
        case "rejectStart":
            switch(packet.reason) {
                case "gameEndsInstantly":
                    Anchor.pushError(translate("notification.rejectStart"), translate("notification.rejectStart.gameEndsInstantly"));
                break;
                case "roleListTooSmall":
                    Anchor.pushError(translate("notification.rejectStart"), translate("notification.rejectStart.roleListTooSmall"));
                break;
                case "roleListCannotCreateRoles":
                    Anchor.pushError(translate("notification.rejectStart"), translate("notification.rejectStart.roleListCannotCreateRoles"));
                break;
                case "zeroTimeGame":
                    Anchor.pushError(translate("notification.rejectStart"), translate("notification.rejectStart.zeroTimeGame"));
                break;
                default:
                    Anchor.pushError(translate("notification.rejectStart"), "");
                    console.error(`${packet.type} message response not implemented: ${packet.reason}`);
                    console.error(packet);
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
        case "playersLostConnection":
            if(GAME_MANAGER.state.stateType === "lobby"){
                for(let [playerId, player] of GAME_MANAGER.state.players){
                    if(packet.lostConnection.includes(playerId))
                        player.connection = "couldReconnect";
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
            if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player")
                GAME_MANAGER.state.clientState.myIndex = packet.playerIndex;

            //TODO jack Im sorry
            Anchor.clearAudioQueue();
        break;
        case "lobbyClients":
            if(GAME_MANAGER.state.stateType === "lobby"){
                GAME_MANAGER.state.players = new Map();
                for(let [clientId, lobbyClient] of Object.entries(packet.clients)){
                    GAME_MANAGER.state.players.set(Number.parseInt(clientId), lobbyClient);
                }
            }
        break;
        case "lobbyName":
            if(GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game"){
                GAME_MANAGER.state.lobbyName = packet.name;
            }
        break;
        case "startGame":
            const isSpectator = GAME_MANAGER.getMySpectator();
            if(isSpectator){
                GAME_MANAGER.setSpectatorGameState();
                Anchor.setContent(<SpectatorGameScreen/>);
            }else{
                GAME_MANAGER.setGameState();
                Anchor.setContent(GameScreen.createDefault());
            }
        break;
        case "gamePlayers":
            if(GAME_MANAGER.state.stateType === "game"){
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

                // Recompute keyword data, since player names are keywords.
                computeKeywordDataWithPlayers();
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
                GAME_MANAGER.state.phaseTimes[packet.phase.type as keyof typeof GAME_MANAGER.state.phaseTimes] = packet.time;
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
                GAME_MANAGER.state.phaseState = packet.phase;
                GAME_MANAGER.state.dayNumber = packet.dayNumber;
        
                if(packet.phase.type === "briefing" && GAME_MANAGER.state.clientState.type === "player"){
                    const role = GAME_MANAGER.state.clientState.roleState?.type;
                    if(role !== undefined){
                        Anchor.setCoverCard(<WikiArticle article={"role/"+role as WikiArticleLink}/>, "wiki-menu-colors");
                    }
                }
            }
        break;
        case "phaseTimeLeft":
            if(GAME_MANAGER.state.stateType === "game")
                GAME_MANAGER.state.timeLeftMs = packet.secondsLeft * 1000;
        break;
        case "playerOnTrial":
            if(GAME_MANAGER.state.stateType === "game" && (
                GAME_MANAGER.state.phaseState.type === "testimony" || 
                GAME_MANAGER.state.phaseState.type === "judgement" || 
                GAME_MANAGER.state.phaseState.type === "finalWords"
            ))
                GAME_MANAGER.state.phaseState.playerOnTrial = packet.playerIndex;
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

                    let numVoted = packet.votesForPlayer[i];
                    if(numVoted !== undefined){
                        GAME_MANAGER.state.players[i].numVoted = numVoted;
                    }
                }
            }
        break;
        case "yourSendChatGroups":
            if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player"){
                GAME_MANAGER.state.clientState.sendChatGroups = [...packet.sendChatGroups];
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
                for (const player of GAME_MANAGER.state.players) {
                    player.roleLabel = null;
                }
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
                for(let i = 0; i < GAME_MANAGER.state.players.length; i++){
                    GAME_MANAGER.state.players[i].playerTags = [];
                }

                for(const [key, value] of Object.entries(packet.playerTags)){
                    if(
                        GAME_MANAGER.state.players !== undefined && 
                        GAME_MANAGER.state.players[Number.parseInt(key)] !== undefined
                    )
                        GAME_MANAGER.state.players[Number.parseInt(key)].playerTags = value as Tag[];
                }
            }
        break;
        case "yourWill":
            if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player"){
                GAME_MANAGER.state.clientState.will = packet.will;

                if(GAME_MANAGER.state.clientState.will === ""){
                    GAME_MANAGER.sendSaveWillPacket("ROLE\nNight 1: \nNight 2:");
                }
            }
        break;
        case "yourNotes":
            if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player"){
                GAME_MANAGER.state.clientState.notes = packet.notes;
                
                if(GAME_MANAGER.state.clientState.notes === ""){
                    GAME_MANAGER.sendSaveNotesPacket(GAME_MANAGER.state.players.map((player) => {
                        return player.toString();
                    }).join(" - \n") + " - \n");
                }
            }
        break;
        case "yourCrossedOutOutlines":
            if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player")
                GAME_MANAGER.state.clientState.crossedOutOutlines = packet.crossedOutOutlines;
            break;
        case "yourDeathNote":
            if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player")
                GAME_MANAGER.state.clientState.deathNote = packet.deathNote ?? "";
        break;
        case "yourRoleState":
            if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player"){
                if(GAME_MANAGER.state.clientState.roleState?.type!== packet.roleState.type){
                    GameScreen.instance?.closeMenu(ContentMenu.RoleSpecificMenu);
                }
                GAME_MANAGER.state.clientState.roleState = packet.roleState;
            }
        break;
        case "yourTarget":
            if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player")
                GAME_MANAGER.state.clientState.targets = packet.playerIndices;
        break;
        case "yourVoting":
            if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player")
                GAME_MANAGER.state.clientState.voted = packet.playerIndex;
        break;
        case "yourJudgement":
            if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player")
                GAME_MANAGER.state.clientState.judgement = packet.verdict;
        break;
        case "yourVoteFastForwardPhase":
            if(GAME_MANAGER.state.stateType === "game")
                GAME_MANAGER.state.fastForward = packet.fastForward;
        break;
        case "addChatMessages":
            if(GAME_MANAGER.state.stateType === "game" || GAME_MANAGER.state.stateType === "lobby"){
                GAME_MANAGER.state.chatMessages = GAME_MANAGER.state.chatMessages.concat(packet.chatMessages);

                for(let chatMessage of packet.chatMessages){
                    let audioSrc = chatMessageToAudio(chatMessage);
                    if(audioSrc)
                        Anchor.queueAudioFile(audioSrc);
                }
            }
        break;
        case "addGrave":
            if(GAME_MANAGER.state.stateType === "game")
                GAME_MANAGER.state.graves.push(packet.grave);
        break;
        case "gameOver":
            if(GAME_MANAGER.state.stateType === "game"){
                GAME_MANAGER.state.ticking = false;
                switch(packet.reason) {
                    case "reachedMaxDay":
                    case "draw":
                        console.log("Game ended! (naturally)");
                    break;
                    default:
                        // alert("Game ended for an unknown reason!");
                        console.error(`${packet.type} message response not implemented: ${packet.reason}`);
                        console.error(packet);
                    break;
                }
            }
        break;
        default:
            console.error(`incoming message response not implemented: ${(packet as any)?.type}`);
            console.error(packet);
        break;
    }

    GAME_MANAGER.invokeStateListeners(packet.type);
}


