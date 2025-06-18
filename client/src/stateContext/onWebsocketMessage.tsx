import AudioController from "../menu/AudioController";
import { computePlayerKeywordData, computePlayerKeywordDataForLobby } from "../components/StyledText";
import { WikiArticleLink } from "../wiki/WikiArticleLink";
import { defaultAlibi } from "../menu/game/gameScreenContent/WillMenu";
import ListMap from "../ListMap";
import { WebSocketContextType } from "../menu/WebsocketContext";
import { ToClientPacket } from "../packet";
import translate from "../game/lang";
import { AppContextType } from "../menu/AppContext";
import { Tag } from "./stateType/tagState";
import { PlayerIndex } from "./stateType/otherState";
import { Role } from "./stateType/roleState";
import { chatMessageToAudio, sendDefaultName } from "../menu/App";
import { StateContext } from "./StateContext";
import { deleteReconnectData, saveReconnectData } from "../game/localStorage";
import { GameClient } from "./stateType/gameState";
import { sortControllerIdCompare } from "../game/abilityInput";
import NightMessagePopup from "../components/NightMessagePopup"
import WikiArticle from "../wiki/WikiArticle";
import React from "react";


export default function onWebsocketMessage(
    packet: ToClientPacket,
    appCtx: AppContextType,
    stateCtx: StateContext,
    websocketCtx: WebSocketContextType,
){
    console.log(JSON.stringify(packet, null, 2));

    switch(packet.type) {
        case "pong":
            if (stateCtx.state.type !== "disconnected") {
                websocketCtx.sendPacket({
                    type: "ping"
                });
            }
        break;
        case "rateLimitExceeded":
            appCtx.pushErrorCard({ title: translate("notification.rateLimitExceeded"), body: "" });
        break;
        case "forcedOutsideLobby":
            stateCtx.setGameBrowser();
            appCtx.setContent({type:"gameBrowser"});
        break;
        case "forcedDisconnect":
            stateCtx.setDisconnected();
            appCtx.setContent({type:"main"});
        break;
        case "lobbyList":
            if(stateCtx.state.type === "gameBrowser"){
                stateCtx.state.lobbies = new Map();

                for(let [lobbyId, lobbyData] of Object.entries(packet.lobbies))
                    stateCtx.state.lobbies.set(Number.parseInt(lobbyId), lobbyData);
            }
        break;
        case "acceptJoin":
            if(packet.inGame){
                stateCtx.setGame(packet.spectator);
                setTimeout(()=>{appCtx.setContent({type:"gameScreen", spectator: packet.spectator});}, 500);
            }else{
                stateCtx.setLobby(packet.roomCode, packet.playerId);
                setTimeout(()=>{appCtx.setContent({type:"lobbyScreen"});}, 500);
            }
            

            if(stateCtx.state.type === "lobby" || stateCtx.state.type === "game"){
                stateCtx.state.roomCode = packet.roomCode;
                stateCtx.state.myId = packet.playerId;
            }

            saveReconnectData(packet.roomCode, packet.playerId);
            sendDefaultName(websocketCtx);
            appCtx.clearCoverCard();
        break;
        case "rejectJoin":
            switch(packet.reason) {
                case "roomDoesntExist":
                    appCtx.pushErrorCard({ title: translate("notification.rejectJoin"), body: translate("notification.rejectJoin.roomDoesntExist") });
                    // If the room doesn't exist, don't suggest the user to reconnect to it.
                    deleteReconnectData();
                    appCtx.clearCoverCard();
                break;
                case "gameAlreadyStarted":
                    appCtx.pushErrorCard({ title: translate("notification.rejectJoin"), body: translate("notification.rejectJoin.gameAlreadyStarted") });
                break;
                case "roomFull":
                    appCtx.pushErrorCard({ title: translate("notification.rejectJoin"), body: translate("notification.rejectJoin.roomFull") });
                break;
                case "serverBusy":
                    appCtx.pushErrorCard({ title: translate("notification.rejectJoin"), body: translate("notification.rejectJoin.serverBusy") });
                break;
                case "playerTaken":
                    appCtx.pushErrorCard({ title: translate("notification.rejectJoin"), body: translate("notification.rejectJoin.playerTaken") });
                break;
                case "playerDoesntExist":
                    appCtx.pushErrorCard({ title: translate("notification.rejectJoin"), body: translate("notification.rejectJoin.playerDoesntExist") });
                break;
                default:
                    appCtx.pushErrorCard({ title: translate("notification.rejectJoin"), body: `${packet.type} message response not implemented: ${packet.reason}` });
                    console.error(`${packet.type} message response not implemented: ${packet.reason}`);
                    console.error(packet);
                break;
            }
            deleteReconnectData();
            
        break;
        case "rejectStart":
            switch(packet.reason) {
                case "gameEndsInstantly":
                    appCtx.pushErrorCard({ title: translate("notification.rejectStart"), body: translate("notification.rejectStart.gameEndsInstantly") });
                break;
                case "roleListTooSmall":
                    appCtx.pushErrorCard({ title: translate("notification.rejectStart"), body: translate("notification.rejectStart.roleListTooSmall") });
                break;
                case "roleListCannotCreateRoles":
                    appCtx.pushErrorCard({ title: translate("notification.rejectStart"), body: translate("notification.rejectStart.roleListCannotCreateRoles") });
                break;
                case "zeroTimeGame":
                    appCtx.pushErrorCard({ title: translate("notification.rejectStart"), body: translate("notification.rejectStart.zeroTimeGame") });
                break;
                case "tooManyCLients":
                    appCtx.pushErrorCard({ title: translate("notification.rejectStart"), body: translate("notification.rejectStart.tooManyClients") });
                break;
                default:
                    appCtx.pushErrorCard({ title: translate("notification.rejectStart"), body: "" });
                    console.error(`${packet.type} message response not implemented: ${packet.reason}`);
                    console.error(packet);
                break;
            }
        break;
        case "playersHost":
            if(stateCtx.state.type === "lobby"){
                for(let [playerId, player] of stateCtx.state.players.entries()){
                    if (packet.hosts.includes(playerId)) {
                        player.ready = "host";
                    } else {
                        player.ready = player.ready === "host" ? "ready" : player.ready
                    }
                }
                stateCtx.state.players = new ListMap(stateCtx.state.players.entries());
            }else if(stateCtx.state.type === "game"){
                if (packet.hosts.includes(stateCtx.state.myId ?? -1)) {
                    if (stateCtx.state.host === null) {
                        stateCtx.state.host = {
                            clients: new ListMap()
                        }
                    }

                    for (const [id, client] of stateCtx.state.host.clients.entries()) {
                        client.host = packet.hosts.includes(id);
                    }
                } else {
                    stateCtx.state.host = null
                }
            }
        break;
        case "playersReady":
            if(stateCtx.state.type === "lobby"){
                for(let [playerId, player] of stateCtx.state.players.entries()){
                    if (packet.ready.includes(playerId)) {
                        player.ready = "ready";
                    } else {
                        player.ready = player.ready === "host" ? "host" : "notReady"
                    }
                }
                stateCtx.state.players = new ListMap(stateCtx.state.players.entries());
            }
        break;
        case "playersLostConnection":
            if(stateCtx.state.type === "lobby"){
                for(let [playerId, player] of stateCtx.state.players.entries()){
                    if(packet.lostConnection.includes(playerId))
                        player.connection = "couldReconnect";
                }
                stateCtx.state.players = new ListMap(stateCtx.state.players.entries());
            }
        break;
        /*
        In Lobby/Game 
        */
        case "yourId":
            if(stateCtx.state.type === "lobby")
                stateCtx.state.myId = packet.playerId;
        break;
        case "yourPlayerIndex":
            if(stateCtx.state.type === "game" && stateCtx.state.clientState.type === "player")
                stateCtx.state.clientState.myIndex = packet.playerIndex;

            //TODO jack Im sorry
            AudioController.clearQueue();
        break;
        case "yourFellowInsiders":
            if(stateCtx.state.type === "game" && stateCtx.state.clientState.type === "player")
                stateCtx.state.clientState.fellowInsiders = packet.fellowInsiders;
        break;
        case "lobbyClients":
            if(stateCtx.state.type === "lobby"){
                const oldMySpectator = stateCtx.state.players.get(stateCtx.state.myId!)?.clientType.type === "spectator";

                stateCtx.state.players = new ListMap();
                for(const [clientId, lobbyClient] of packet.clients){
                    stateCtx.state.players.insert(clientId, lobbyClient);
                }
                const newMySpectator = stateCtx.state.players.get(stateCtx.state.myId!)?.clientType.type === "spectator";

                
                if (oldMySpectator && !newMySpectator){
                    sendDefaultName(websocketCtx);
                }

                // Recompute keyword data, since player names are keywords.
                computePlayerKeywordDataForLobby(
                    Array.from(stateCtx.state.players.values())
                        .filter(client => client.clientType.type === "player")
                        .map(client => (client.clientType as { type: "player", name: string }).name)
                );
            }
        break;
        case "hostData":
            if (stateCtx.state.type === "game") {
                stateCtx.state.host = {
                    clients: new ListMap<number, GameClient>(packet.clients)
                }
            } 
        break;
        case "lobbyName":
            if(stateCtx.state.type === "lobby" || stateCtx.state.type === "game"){
                stateCtx.state.lobbyName = packet.name;
            }
        break;
        case "startGame": 
            if (stateCtx.state.type === "lobby") {
                const isSpectator = stateCtx.state.players.get(stateCtx.state.myId!)?.clientType.type === "spectator";
                stateCtx.setGame(isSpectator);
    
                AudioController.queueFile("audio/start_game.mp3");
            }
            break;
        case "gameInitializationComplete":
            if (stateCtx.state.type === "game") {
                const isSpectator = stateCtx.state.clientState.type === "spectator";
                stateCtx.state.initialized = true;
                appCtx.setContent({type:"gameScreen", spectator: isSpectator});
            }
            break;
        case "backToLobby":
            if(stateCtx.state.type==="game"){
                stateCtx.setLobby(
                    stateCtx.state.roomCode,
                    stateCtx.state.myId
                );
                appCtx.setContent({type:"lobbyScreen"});
            }
        break;
        case "gamePlayers":
            if(stateCtx.state.type === "game"){
                //only update the playerlist with the new one if there are any differences
                let playersChanged = false;
                if(stateCtx.state.players.length !== packet.players.length)
                    playersChanged = true;
                else{
                    for(let i = 0; i < packet.players.length; i++){
                        if(stateCtx.state.players[i].name !== packet.players[i]){
                            playersChanged = true;
                            break;
                        }
                    }
                }
                if(playersChanged){
                    stateCtx.state.players = [];
                    for(let i = 0; i < packet.players.length; i++){
                        stateCtx.state.players.push(createPlayer(packet.players[i], i));
                    }
                }

                // Recompute keyword data, since player names are keywords.
                computePlayerKeywordData(stateCtx.state.players);
            }
        break;
        case "roleList":
            //list of role list entriy
            if(stateCtx.state.type === "lobby" || stateCtx.state.type === "game")
                stateCtx.state.roleList = packet.roleList;
        break;
        case "roleOutline":
            //role list entriy
            if(stateCtx.state.type === "lobby" || stateCtx.state.type === "game") {
                stateCtx.state.roleList = structuredClone(stateCtx.state.roleList);
                stateCtx.state.roleList[packet.index] = packet.roleOutline;
                stateCtx.state.roleList = [...stateCtx.state.roleList];
            }
        break;
        case "phaseTime":
            if(stateCtx.state.type === "lobby" || stateCtx.state.type === "game") {
                stateCtx.state.phaseTimes[packet.phase.type] = packet.time;
                stateCtx.state.phaseTimes = {...stateCtx.state.phaseTimes};
            }
        break;
        case "phaseTimes":
            if(stateCtx.state.type === "lobby" || stateCtx.state.type === "game")
                stateCtx.state.phaseTimes = packet.phaseTimeSettings;
        break;
        case "enabledRoles":
            if(stateCtx.state.type === "lobby" || stateCtx.state.type === "game")
                stateCtx.state.enabledRoles = packet.roles;
        break;
        case "enabledModifiers":
            if(stateCtx.state.type === "lobby" || stateCtx.state.type === "game")
                stateCtx.state.enabledModifiers = packet.modifiers;
        break;
        case "phase":
            if(stateCtx.state.type === "game"){
                stateCtx.state.phaseState = packet.phase;
                stateCtx.state.dayNumber = packet.dayNumber;
        
                if(packet.phase.type === "briefing" && stateCtx.state.clientState.type === "player"){
                    const role = stateCtx.state.clientState.roleState?.type;
                    if(role !== undefined){
                        appCtx.setCoverCard(<WikiArticle article={"role/"+role as WikiArticleLink}/>);
                    }
                }
            }
        break;
        case "phaseTimeLeft":
            if(stateCtx.state.type === "game")
                stateCtx.state.timeLeftMs = packet.secondsLeft!==null?(packet.secondsLeft * 1000):null;
        break;
        case "playerAlive":
            if(stateCtx.state.type === "game"){
                for(let i = 0; i < stateCtx.state.players.length && i < packet.alive.length; i++){
                    stateCtx.state.players[i].alive = packet.alive[i];
                }
                stateCtx.state.players = [...stateCtx.state.players];
            }
        break;
        case "playerVotes":
            if(stateCtx.state.type === "game"){

                let listMapVotes = new ListMap<PlayerIndex, number>(packet.votesForPlayer);

                for(let i = 0; i < stateCtx.state.players.length; i++){
                    stateCtx.state.players[i].numVoted = 0;
                    
                    let numVoted = listMapVotes.get(i);
                    if(numVoted !== null){
                        stateCtx.state.players[i].numVoted = numVoted;
                    }
                }
                stateCtx.state.players = [...stateCtx.state.players];
            }
        break;
        case "yourSendChatGroups":
            if(stateCtx.state.type === "game" && stateCtx.state.clientState.type === "player"){
                stateCtx.state.clientState.sendChatGroups = [...packet.sendChatGroups];
            }
        break;
        case "yourInsiderGroups":
            if(stateCtx.state.type === "game" && stateCtx.state.clientState.type === "player"){
                stateCtx.state.clientState.insiderGroups = [...packet.insiderGroups];
            }
        break;
        case "yourAllowedControllers":
            if(stateCtx.state.type === "game" && stateCtx.state.clientState.type === "player"){
                stateCtx.state.clientState.savedControllers = 
                    packet.save.sort((a, b) => sortControllerIdCompare(a[0],b[0]));
            }
        break;
        case "yourRoleLabels":
            if(stateCtx.state.type === "game"){
                for (const player of stateCtx.state.players) {
                    player.roleLabel = null;
                }
                for (const [key, value] of packet.roleLabels) { 
                    if(
                        stateCtx.state.players !== undefined && 
                        stateCtx.state.players[key] !== undefined
                    )
                        stateCtx.state.players[key].roleLabel = value as Role;
                }
                stateCtx.state.players = [...stateCtx.state.players];
            }
        break;
        case "yourPlayerTags":
            if(stateCtx.state.type === "game"){
                for(let i = 0; i < stateCtx.state.players.length; i++){
                    stateCtx.state.players[i].playerTags = [];
                }

                for(const [key, value] of packet.playerTags){
                    if(
                        stateCtx.state.players !== undefined && 
                        stateCtx.state.players[key] !== undefined
                    )
                        stateCtx.state.players[key].playerTags = value as Tag[];
                }
                stateCtx.state.players = [...stateCtx.state.players];
            }
        break;
        case "yourWill":
            if(stateCtx.state.type === "game" && stateCtx.state.clientState.type === "player"){
                stateCtx.state.clientState.will = packet.will;

                if(stateCtx.state.clientState.will === ""){
                    websocketCtx.sendSaveWillPacket(defaultAlibi());
                }
            }
        break;
        case "yourNotes":
            if(stateCtx.state.type === "game" && stateCtx.state.clientState.type === "player"){
                stateCtx.state.clientState.notes = packet.notes;
                
                // old default notes
                // if(stateCtx.state.clientState.notes.length === 0){
                //     const myIndex = stateCtx.state.clientState.myIndex;
                //     const myRoleKey = `role.${stateCtx.state.clientState.roleState.type}.name`;

                //     stateCtx.state.sendSaveNotesPacket([
                //         "Claims\n" + 
                //         stateCtx.state.players
                //             .map(player => 
                //                 `@${player.index + 1} - ${player.index === myIndex ? translate(myRoleKey) : ''}\n`
                //             )
                //             .join('')
                //     ]);
                // }
            }
        break;
        case "yourCrossedOutOutlines":
            if(stateCtx.state.type === "game" && stateCtx.state.clientState.type === "player")
                stateCtx.state.clientState.crossedOutOutlines = packet.crossedOutOutlines;
        break;
        case "yourDeathNote":
            if(stateCtx.state.type === "game" && stateCtx.state.clientState.type === "player")
                stateCtx.state.clientState.deathNote = packet.deathNote ?? "";
        break;
        case "yourRoleState":
            if(stateCtx.state.type === "game" && stateCtx.state.clientState.type === "player"){
                stateCtx.state.clientState.roleState = packet.roleState;
            }
        break;
        case "yourJudgement":
            if(stateCtx.state.type === "game" && stateCtx.state.clientState.type === "player")
                stateCtx.state.clientState.judgement = packet.verdict;
        break;
        case "yourVoteFastForwardPhase":
            if(stateCtx.state.type === "game")
                stateCtx.state.fastForward = packet.fastForward;
        break;
        case "addChatMessages":
            if(stateCtx.state.type === "game" || stateCtx.state.type === "lobby"){
                stateCtx.state.chatMessages = stateCtx.state.chatMessages.concat(packet.chatMessages);

                // Chat notification icon state
                if(stateCtx.state.type === "game" && packet.chatMessages.length !== 0){
                    stateCtx.state.missedChatMessages = true;
                    
                    for(let chatMessage of packet.chatMessages){
                        if(
                            chatMessage.variant.type === "whisper" &&
                            stateCtx.state.clientState.type === "player" &&
                            chatMessage.variant.toPlayerIndex === stateCtx.state.clientState.myIndex
                        ){
                            stateCtx.state.clientState.missedWhispers.push(chatMessage.variant.fromPlayerIndex);
                        }
                    }
                }

                if (stateCtx.state.type !== "game" || stateCtx.state.initialized === true) {
                    for(let chatMessage of packet.chatMessages){
                        let audioSrc = chatMessageToAudio(chatMessage);
                        if(audioSrc)
                            AudioController.queueFile(audioSrc);
                    }
                }
            }
        break;
        case "nightMessages":
            if(stateCtx.state.type === "game" || stateCtx.state.type === "lobby"){

                if(appCtx.getCoverCard()===null && packet.chatMessages.length!==0){
                    appCtx.setCoverCard(<NightMessagePopup messages={packet.chatMessages}/>)
                }
            }
        break;
        case "addGrave":
            if(stateCtx.state.type === "game")
                stateCtx.state.graves = [...stateCtx.state.graves, packet.grave];
        break;
        case "gameOver":
            if(stateCtx.state.type === "game"){
                stateCtx.state.ticking = false;
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

    /*BEFORE YOU DELETE THIS LINE, REMEMBER THAT STATECTX STUFF NEEDS SET STATE HERE?!?!??*/
    // stateCtx.state.invokeStateListeners(packet.type);
}

function createPlayer(arg0: string, i: number): import("./stateType/gameState").Player {
    throw new Error("Function not implemented.");
}

