import { StateContext, useStateContext } from "./StateContext";
import { useWebsocketMessageListener, WebsocketContext } from "../menu/WebsocketContext";
import { AppContext, AppContextType } from "../menu/AppContext";
import GameScreen from "./../menu/game/GameScreen";
import { computePlayerKeywordData, computePlayerKeywordDataForLobby } from "../components/StyledText";
import { WikiArticleLink } from "../wiki/WikiArticleLink";
import React, { ReactElement, useContext } from "react";
import WikiArticle from "../wiki/WikiArticle";
import LobbyMenu from "../menu/lobby/LobbyMenu";
import LoadingScreen from "../menu/LoadingScreen";
import AudioController from "../menu/AudioController";
import NightMessagePopup from "../components/NightMessagePopup";
import PlayMenu from "../menu/main/PlayMenu";
import { defaultAlibi } from "../menu/game/gameScreenContent/WillMenu";
import ListMap from "../ListMap";
import { WebSocketContextType } from "../menu/WebsocketContext";
import { State } from "../stateContext/state";
import { ToClientPacket } from "../packet";
import translate from "../game/lang";
import { GameClient } from "./stateType/otherState";

function StateContextProvider(props: Readonly<{
    children: React.ReactNode
}>): ReactElement{
    const appCtx = useContext(AppContext)!;
    const websocketContext = useContext(WebsocketContext)!;
    const stateCtx = useStateContext();

    const onMessage = (packet: ToClientPacket)=>{
        messageListener(packet, appCtx, stateCtx, websocketContext);
    };

    useWebsocketMessageListener(websocketContext, onMessage);

    return <StateContext.Provider value={stateCtx}>
        {props.children}
    </StateContext.Provider>
}


export default function messageListener(
    packet: ToClientPacket,
    appCtx: AppContextType,
    stateCtx: State,
    websocketCtx: WebSocketContextType,
){
    console.log(JSON.stringify(packet, null, 2));

    switch(packet.type) {
        case "pong":
            if (stateCtx.type !== "disconnected") {
                websocketCtx.sendPacket({
                    type: "ping"
                });
            }
        break;
        case "rateLimitExceeded":
            appCtx.pushErrorCard({ title: translate("notification.rateLimitExceeded"), body: "" });
        break;
        case "forcedOutsideLobby":
            stateCtx.setOutsideLobbyState();
            appCtx.setContent(<PlayMenu/>);
        break;
        case "forcedDisconnect":
            stateCtx.setDisconnectedState();
            appCtx.setContent({type:"main"});
        break
        case "lobbyList":
            if(stateCtx.type === "gameBrowser"){
                stateCtx.lobbies = new Map();

                for(let [lobbyId, lobbyData] of Object.entries(packet.lobbies))
                    stateCtx.lobbies.set(Number.parseInt(lobbyId), lobbyData);
            }
        break;
        case "acceptJoin":
            if(packet.inGame && packet.spectator){
                stateCtx.setSpectatorGameState();
                appCtx.setContent(<LoadingScreen type="join" />)
            }else if(packet.inGame && !packet.spectator){
                stateCtx.setGameState();
                appCtx.setContent(<LoadingScreen type="join" />)
            }else{
                stateCtx.setLobbyState();
                appCtx.setContent(<LobbyMenu/>);
            }
            

            if(stateCtx.type === "lobby" || stateCtx.type === "game"){
                stateCtx.roomCode = packet.roomCode;
                stateCtx.myId = packet.playerId;
            }

            saveReconnectData(packet.roomCode, packet.playerId);
            sendDefaultName();
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
            if(stateCtx.type === "lobby"){
                for(let [playerId, player] of stateCtx.players.entries()){
                    if (packet.hosts.includes(playerId)) {
                        player.ready = "host";
                    } else {
                        player.ready = player.ready === "host" ? "ready" : player.ready
                    }
                }
                stateCtx.players = new ListMap(stateCtx.players.entries());
            }else if(stateCtx.type === "game"){
                if (packet.hosts.includes(stateCtx.myId ?? -1)) {
                    if (stateCtx.host === null) {
                        stateCtx.host = {
                            clients: new ListMap()
                        }
                    }

                    for (const [id, client] of stateCtx.host.clients.entries()) {
                        client.host = packet.hosts.includes(id);
                    }
                } else {
                    stateCtx.host = null
                }
            }
        break;
        case "playersReady":
            if(stateCtx.type === "lobby"){
                for(let [playerId, player] of stateCtx.players.entries()){
                    if (packet.ready.includes(playerId)) {
                        player.ready = "ready";
                    } else {
                        player.ready = player.ready === "host" ? "host" : "notReady"
                    }
                }
                stateCtx.players = new ListMap(stateCtx.players.entries());
            }
        break;
        case "playersLostConnection":
            if(stateCtx.type === "lobby"){
                for(let [playerId, player] of stateCtx.players.entries()){
                    if(packet.lostConnection.includes(playerId))
                        player.connection = "couldReconnect";
                }
                stateCtx.players = new ListMap(stateCtx.players.entries());
            }
        break;
        /*
        In Lobby/Game 
        */
        case "yourId":
            if(stateCtx.type === "lobby")
                stateCtx.myId = packet.playerId;
        break;
        case "yourPlayerIndex":
            if(stateCtx.type === "game" && stateCtx.clientState.type === "player")
                stateCtx.clientState.myIndex = packet.playerIndex;

            //TODO jack Im sorry
            AudioController.clearQueue();
        break;
        case "yourFellowInsiders":
            if(stateCtx.type === "game" && stateCtx.clientState.type === "player")
                stateCtx.clientState.fellowInsiders = packet.fellowInsiders;
        break;
        case "lobbyClients":
            if(stateCtx.type === "lobby"){
                const oldMySpectator = stateCtx.players.get(stateCtx.myId!)?.clientType.type === "spectator";

                stateCtx.players = new ListMap();
                for(const [clientId, lobbyClient] of packet.clients){
                    stateCtx.players.insert(clientId, lobbyClient);
                }
                const newMySpectator = stateCtx.players.get(stateCtx.myId!)?.clientType.type === "spectator";

                
                if (oldMySpectator && !newMySpectator){
                    sendDefaultName();
                }

                // Recompute keyword data, since player names are keywords.
                computePlayerKeywordDataForLobby(
                    Array.from(stateCtx.players.values())
                        .filter(client => client.clientType.type === "player")
                        .map(client => (client.clientType as { type: "player", name: string }).name)
                );
            }
        break;
        case "hostData":
            if (stateCtx.type === "game") {
                stateCtx.host = {
                    clients: new ListMap<number, GameClient>(packet.clients)
                }
            } 
        break;
        case "lobbyName":
            if(stateCtx.type === "lobby" || stateCtx.type === "game"){
                stateCtx.lobbyName = packet.name;
            }
        break;
        case "startGame": 
            if (stateCtx.type === "lobby") {
                const isSpectator = stateCtx.players.get(stateCtx.myId!)?.clientType.type === "spectator";
                if(isSpectator){
                    stateCtx.setSpectatorGameState();
                    appCtx.setContent(<LoadingScreen type="join" />)
                }else{
                    stateCtx.setGameState();
                    appCtx.setContent(<LoadingScreen type="join" />)
                }
    
                AudioController.queueFile("audio/start_game.mp3");
            }
            break;
        case "gameInitializationComplete":
            if (stateCtx.type === "game") {
                const isSpectator = stateCtx.clientState.type === "spectator";
                stateCtx.initialized = true;
                appCtx.setContent(<GameScreen isSpectator={isSpectator}/>);
            }
            break;
        case "backToLobby":
            stateCtx.setLobbyState();
            appCtx.setContent({type:"lobbyScreen"});
        break;
        case "gamePlayers":
            if(stateCtx.type === "game"){
                //only update the playerlist with the new one if there are any differences
                let playersChanged = false;
                if(stateCtx.players.length !== packet.players.length)
                    playersChanged = true;
                else{
                    for(let i = 0; i < packet.players.length; i++){
                        if(stateCtx.players[i].name !== packet.players[i]){
                            playersChanged = true;
                            break;
                        }
                    }
                }
                if(playersChanged){
                    stateCtx.players = [];
                    for(let i = 0; i < packet.players.length; i++){
                        stateCtx.players.push(createPlayer(packet.players[i], i));
                    }
                }

                // Recompute keyword data, since player names are keywords.
                computePlayerKeywordData(stateCtx.players);
            }
        break;
        case "roleList":
            //list of role list entriy
            if(stateCtx.type === "lobby" || stateCtx.type === "game")
                stateCtx.roleList = packet.roleList;
        break;
        case "roleOutline":
            //role list entriy
            if(stateCtx.type === "lobby" || stateCtx.type === "game") {
                stateCtx.roleList = structuredClone(stateCtx.roleList);
                stateCtx.roleList[packet.index] = packet.roleOutline;
                stateCtx.roleList = [...stateCtx.roleList];
            }
        break;
        case "phaseTime":
            if(stateCtx.type === "lobby" || stateCtx.type === "game") {
                stateCtx.phaseTimes[packet.phase.type] = packet.time;
                stateCtx.phaseTimes = {...stateCtx.phaseTimes};
            }
        break;
        case "phaseTimes":
            if(stateCtx.type === "lobby" || stateCtx.type === "game")
                stateCtx.phaseTimes = packet.phaseTimeSettings;
        break;
        case "enabledRoles":
            if(stateCtx.type === "lobby" || stateCtx.type === "game")
                stateCtx.enabledRoles = packet.roles;
        break;
        case "enabledModifiers":
            if(stateCtx.type === "lobby" || stateCtx.type === "game")
                stateCtx.enabledModifiers = packet.modifiers;
        break;
        case "phase":
            if(stateCtx.type === "game"){
                stateCtx.phaseState = packet.phase;
                stateCtx.dayNumber = packet.dayNumber;
        
                if(packet.phase.type === "briefing" && stateCtx.clientState.type === "player"){
                    const role = stateCtx.clientState.roleState?.type;
                    if(role !== undefined){
                        appCtx.setCoverCard(<WikiArticle article={"role/"+role as WikiArticleLink}/>);
                    }
                }
            }
        break;
        case "phaseTimeLeft":
            if(stateCtx.type === "game")
                stateCtx.timeLeftMs = packet.secondsLeft!==null?(packet.secondsLeft * 1000):null;
        break;
        case "playerAlive":
            if(stateCtx.type === "game"){
                for(let i = 0; i < stateCtx.players.length && i < packet.alive.length; i++){
                    stateCtx.players[i].alive = packet.alive[i];
                }
                stateCtx.players = [...stateCtx.players];
            }
        break;
        case "playerVotes":
            if(stateCtx.type === "game"){

                let listMapVotes = new ListMap<PlayerIndex, number>(packet.votesForPlayer);

                for(let i = 0; i < stateCtx.players.length; i++){
                    stateCtx.players[i].numVoted = 0;
                    
                    let numVoted = listMapVotes.get(i);
                    if(numVoted !== null){
                        stateCtx.players[i].numVoted = numVoted;
                    }
                }
                stateCtx.players = [...stateCtx.players];
            }
        break;
        case "yourSendChatGroups":
            if(stateCtx.type === "game" && stateCtx.clientState.type === "player"){
                stateCtx.clientState.sendChatGroups = [...packet.sendChatGroups];
            }
        break;
        case "yourInsiderGroups":
            if(stateCtx.type === "game" && stateCtx.clientState.type === "player"){
                stateCtx.clientState.insiderGroups = [...packet.insiderGroups];
            }
        break;
        case "yourAllowedControllers":
            if(stateCtx.type === "game" && stateCtx.clientState.type === "player"){
                stateCtx.clientState.savedControllers = 
                    packet.save.sort((a, b) => sortControllerIdCompare(a[0],b[0]));
            }
        break;
        case "yourRoleLabels":
            if(stateCtx.type === "game"){
                for (const player of stateCtx.players) {
                    player.roleLabel = null;
                }
                for (const [key, value] of packet.roleLabels) { 
                    if(
                        stateCtx.players !== undefined && 
                        stateCtx.players[key] !== undefined
                    )
                        stateCtx.players[key].roleLabel = value as Role;
                }
                stateCtx.players = [...stateCtx.players];
            }
        break;
        case "yourPlayerTags":
            if(stateCtx.type === "game"){
                for(let i = 0; i < stateCtx.players.length; i++){
                    stateCtx.players[i].playerTags = [];
                }

                for(const [key, value] of packet.playerTags){
                    if(
                        stateCtx.players !== undefined && 
                        stateCtx.players[key] !== undefined
                    )
                        stateCtx.players[key].playerTags = value as Tag[];
                }
                stateCtx.players = [...stateCtx.players];
            }
        break;
        case "yourWill":
            if(stateCtx.type === "game" && stateCtx.clientState.type === "player"){
                stateCtx.clientState.will = packet.will;

                if(stateCtx.clientState.will === ""){
                    websocketCtx.sendSaveWillPacket(defaultAlibi());
                }
            }
        break;
        case "yourNotes":
            if(stateCtx.type === "game" && stateCtx.clientState.type === "player"){
                stateCtx.clientState.notes = packet.notes;
                
                // old default notes
                // if(stateCtx.clientState.notes.length === 0){
                //     const myIndex = stateCtx.clientState.myIndex;
                //     const myRoleKey = `role.${stateCtx.clientState.roleState.type}.name`;

                //     stateCtx.sendSaveNotesPacket([
                //         "Claims\n" + 
                //         stateCtx.players
                //             .map(player => 
                //                 `@${player.index + 1} - ${player.index === myIndex ? translate(myRoleKey) : ''}\n`
                //             )
                //             .join('')
                //     ]);
                // }
            }
        break;
        case "yourCrossedOutOutlines":
            if(stateCtx.type === "game" && stateCtx.clientState.type === "player")
                stateCtx.clientState.crossedOutOutlines = packet.crossedOutOutlines;
        break;
        case "yourDeathNote":
            if(stateCtx.type === "game" && stateCtx.clientState.type === "player")
                stateCtx.clientState.deathNote = packet.deathNote ?? "";
        break;
        case "yourRoleState":
            if(stateCtx.type === "game" && stateCtx.clientState.type === "player"){
                stateCtx.clientState.roleState = packet.roleState;
            }
        break;
        case "yourJudgement":
            if(stateCtx.type === "game" && stateCtx.clientState.type === "player")
                stateCtx.clientState.judgement = packet.verdict;
        break;
        case "yourVoteFastForwardPhase":
            if(stateCtx.type === "game")
                stateCtx.fastForward = packet.fastForward;
        break;
        case "addChatMessages":
            if(stateCtx.type === "game" || stateCtx.type === "lobby"){
                stateCtx.chatMessages = stateCtx.chatMessages.concat(packet.chatMessages);

                // Chat notification icon state
                if(stateCtx.type === "game" && packet.chatMessages.length !== 0){
                    stateCtx.missedChatMessages = true;
                    
                    for(let chatMessage of packet.chatMessages){
                        if(
                            chatMessage.variant.type === "whisper" &&
                            stateCtx.clientState.type === "player" &&
                            chatMessage.variant.toPlayerIndex === stateCtx.clientState.myIndex
                        ){
                            stateCtx.clientState.missedWhispers.push(chatMessage.variant.fromPlayerIndex);
                        }
                    }
                }

                if (stateCtx.type !== "game" || stateCtx.initialized === true) {
                    for(let chatMessage of packet.chatMessages){
                        let audioSrc = chatMessageToAudio(chatMessage);
                        if(audioSrc)
                            AudioController.queueFile(audioSrc);
                    }
                }
            }
        break;
        case "nightMessages":
            if(stateCtx.type === "game" || stateCtx.type === "lobby"){

                if(appCtx.getCoverCard()===null && packet.chatMessages.length!==0){
                    appCtx.setCoverCard(<NightMessagePopup messages={packet.chatMessages}/>)
                }
            }
        break;
        case "addGrave":
            if(stateCtx.type === "game")
                stateCtx.graves = [...stateCtx.graves, packet.grave];
        break;
        case "gameOver":
            if(stateCtx.type === "game"){
                stateCtx.ticking = false;
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

    /*BEFORE YOU DELETE THIS LINE, REMEMBER THAT STATECTX STUFF NEEDS SET STATE HERE?!?!??*/stateCtx.invokeStateListeners(packet.type);
}


