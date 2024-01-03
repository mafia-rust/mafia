
import { createPlayer } from "./gameState";
import Anchor from "./../menu/Anchor";
import GAME_MANAGER from "./../index";
import GameScreen, { ContentMenus } from "./../menu/game/GameScreen";
import { ToClientPacket } from "./packet";
import { Tag } from "./gameState.d";
import { Role } from "./roleState.d";
import translate from "./lang";

export default function messageListener(packet: ToClientPacket){

    console.log(JSON.stringify(packet, null, 2));


    switch(packet.type) {
        case "rateLimitExceeded":
            Anchor.pushError(translate("notification.rateLimitExceeded"), "");
        break;
        case "lobbyList":
            if(GAME_MANAGER.state.stateType === "outsideLobby")
                GAME_MANAGER.state.roomCodes = packet.roomCodes.map((roomCode) => roomCode.toString(18));
        break;
        case "acceptJoin":
            if(packet.inGame){
                GAME_MANAGER.setGameState();
            }else{
                GAME_MANAGER.setLobbyState();
            }
            if(GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game"){
                GAME_MANAGER.state.roomCode = packet.roomCode.toString(18);
            }
            if(GAME_MANAGER.state.stateType === "lobby")
                GAME_MANAGER.state.myId = packet.playerId;

            GAME_MANAGER.saveReconnectData(packet.roomCode.toString(18), packet.playerId);
        break;
        case "rejectJoin":
            switch(packet.reason) {
                case "roomDoesntExist":
                    Anchor.pushError(translate("notification.rejectJoin"), translate("notification.rejectJoin.roomDoesntExist"));
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

            GAME_MANAGER.setDisconnectedState();

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
                    player.lostConnection = packet.lostConnection.includes(playerId);
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
                    GAME_MANAGER.state.players.set(Number.parseInt(playerId), {name: name, host: false, lostConnection: false});
                }
            }
        break;
        case "startGame":
            GAME_MANAGER.setGameState();
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
                for(let i = 0; i < GAME_MANAGER.state.players.length; i++){
                    GAME_MANAGER.state.players[i].playerTags = [];
                }

                for(const [key, value] of Object.entries(packet.playerTags)){
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


