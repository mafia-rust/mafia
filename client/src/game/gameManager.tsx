import Anchor from "./../menu/Anchor";
import StartMenu from "./../menu/main/StartMenu";
import GAME_MANAGER from "./../index";
import messageListener from "./messageListener";
import CONFIG from "./../resources/config.json"
import React from "react";
import { Phase, PhaseTimes, Verdict } from "./gameState.d";
import { GameManager, Server, StateListener } from "./gameManager.d";
import { ToClientPacket, ToServerPacket } from "./packet";
import { RoleOutline } from "./roleListState.d";
import translate from "./lang";
import PlayMenu from "../menu/main/PlayMenu";
import { createGameState, createLobbyState } from "./gameState";
import LobbyMenu from "../menu/lobby/LobbyMenu";
import GameScreen from "../menu/game/GameScreen";
import LoadingScreen from "../menu/LoadingScreen";
export function createGameManager(): GameManager {

    console.log("Game manager created.");
    
    let gameManager: GameManager = {

        setDisconnectedState() {
            GAME_MANAGER.server.close();
            
            GAME_MANAGER.state = {
                stateType: "disconnected"
            };
            Anchor.setContent(<StartMenu/>);
        },
        setLobbyState() {
            GAME_MANAGER.state = createLobbyState();
            Anchor.setContent(<LobbyMenu/>);
        },
        setGameState() {
            GAME_MANAGER.state = createGameState();
            Anchor.setContent(GameScreen.createDefault());
        },
        async setOutsideLobbyState() {

            Anchor.setContent(<LoadingScreen type="default"/>)
            // GAME_MANAGER.server.close();
            if (!GAME_MANAGER.server.ws?.OPEN){
                await GAME_MANAGER.server.open();
            }

            GAME_MANAGER.state = {
                stateType: "outsideLobby",
                selectedRoomCode: null,
                roomCodes: []
            }

            Anchor.setContent(<PlayMenu/>);
        },

        saveReconnectData(roomCode, playerId) {
            localStorage.setItem(
                "reconnectData", 
                JSON.stringify({
                    "roomCode": roomCode,
                    "playerId": playerId,
                    "lastSaveTime": Date.now()
                })
            );
        },
        deleteReconnectData() {
            localStorage.removeItem("reconnectData");
        },
        loadReconnectData() {
            let data = localStorage.getItem("reconnectData");
            // localStorage.removeItem("reconnectData");
            if (data) {
                return JSON.parse(data);
            }
            return null;
        },

        state: {
            stateType: "disconnected"
        },
        
        getMyName() {
            if (gameManager.state.stateType === "lobby") 
                return gameManager.state.players.get(gameManager.state.myId!)?.name;
            if (gameManager.state.stateType === "game")
                return gameManager.state.players[gameManager.state.myIndex!]?.name;
            return undefined;
        },
        getMyHost() {
            if (gameManager.state.stateType === "lobby") 
                return gameManager.state.players.get(gameManager.state.myId!)?.host;
            if (gameManager.state.stateType === "game")
                return gameManager.state.players[gameManager.state.myIndex!]?.host;            
            return undefined;
        },


        server : createServer(),

        listeners : [],

        addStateListener(listener) {
            gameManager.listeners.push(listener);
        },
        removeStateListener(listener) {
            let index = gameManager.listeners.indexOf(listener);
            if(index !== -1)
                gameManager.listeners.splice(index, 1);
        },
        invokeStateListeners(type) {
            for(let i = 0; i < gameManager.listeners.length; i++){
                if(typeof(gameManager.listeners[i])==="function"){
                    gameManager.listeners[i](type);
                }
            }
        },


        leaveGame() {
            if (this.state.stateType === "game") {
                this.server.sendPacket({type: "leave"});
            }
            this.deleteReconnectData();
            this.setOutsideLobbyState();
            // Set URL to main menu and refresh
            // window.history.replaceState({}, document.title, window.location.pathname);
            // window.location.reload();
        },

        sendLobbyListRequest() {
            this.server.sendPacket({type: "lobbyListRequest"});
        },
        sendHostPacket() {
            this.server.sendPacket({type: "host"});
        },
        sendRejoinPacket(roomCode: string, playerId: number) {
            let completePromise: () => void;
            let promise = new Promise<void>((resolver) => {
                completePromise = resolver;
            });
            let onJoined: StateListener = (type) => {
                if (type==="acceptJoin") {
                    completePromise();
                    GAME_MANAGER.removeStateListener(onJoined);
                }
            };
            GAME_MANAGER.addStateListener(onJoined);

            this.server.sendPacket({
                type: "reJoin",
                roomCode: parseInt(roomCode, 18),
                playerId: playerId
            });
            
            
            return promise;
        },
        sendJoinPacket(roomCode: string) {
            let completePromise: () => void;
            let promise = new Promise<void>((resolver) => {
                completePromise = resolver;
            });
            
            let onJoined: StateListener = (type) => {
                if (type==="acceptJoin") {
                    completePromise();
                    GAME_MANAGER.removeStateListener(onJoined);
                }
            };
            GAME_MANAGER.addStateListener(onJoined);

            let actualCode: number = parseInt(roomCode, 18);

            this.server.sendPacket({
                type: "join",
                roomCode: isNaN(actualCode) ? 0 : actualCode
            });

            return promise;
        },

        sendSetNamePacket(name) {
            this.server.sendPacket({
                type: "setName",
                name: name
            });
        },
        sendStartGamePacket() {
            this.server.sendPacket({
                type: "startGame"
            });
        },
        sendSetPhaseTimePacket(phase: Phase, time: number) {
            if (isValidPhaseTime(time)) {
                this.server.sendPacket({
                    type: "setPhaseTime",
                    phase: phase,
                    time: time
                });
            }
        },
        sendSetPhaseTimesPacket(phaseTimeSettings: PhaseTimes) {
            this.server.sendPacket({
                type: "setPhaseTimes",
                phaseTimeSettings
            });
        },
        sendSetRoleListPacket(roleListEntries: RoleOutline[]) {
            this.server.sendPacket({
                type: "setRoleList",
                roleList: roleListEntries
            });
        },
        sendSetRoleOutlinePacket(index: number, roleOutline: RoleOutline) {
            this.server.sendPacket({
                type: "setRoleOutline",
                index,
                roleOutline
            });
        },

        sendJudgementPacket(judgement: Verdict) {
            this.server.sendPacket({
                type: "judgement",
                verdict: judgement
            });
        },
        sendVotePacket(voteeIndex) {
            this.server.sendPacket({
                type: "vote",
                playerIndex: voteeIndex
            });
        },
        sendTargetPacket(targetIndexList) {
            this.server.sendPacket({
                type: "target",
                playerIndexList: targetIndexList
            });
        },
        sendDayTargetPacket(targetIndex) {
            this.server.sendPacket({
                type: "dayTarget",
                playerIndex: targetIndex
            });
        },

        sendSaveWillPacket(will) {
            this.server.sendPacket({
                type: "saveWill",
                will: will
            });
        },
        sendSaveNotesPacket(notes) {
            this.server.sendPacket({
                type: "saveNotes",
                notes: notes
            });
        },
        sendSaveDeathNotePacket(notes) {
            this.server.sendPacket({
                type: "saveDeathNote",
                deathNote: notes.trim().length === 0 ? null : notes
            });
        },
        sendSendMessagePacket(text) {
            this.server.sendPacket({
                type: "sendMessage",
                text: text
            });
        },
        sendSendWhisperPacket(playerIndex, text) {
            this.server.sendPacket({
                type: "sendWhisper",
                playerIndex: playerIndex,
                text: text
            });
        },
        sendExcludedRolesPacket(roles){
            this.server.sendPacket({
                type:"setExcludedRoles",
                roles:roles
            })
        },

        sendSetDoomsayerGuess(guesses) {
            this.server.sendPacket({
                type: "setDoomsayerGuess",
                guesses: guesses
            });
        },
        sendSetAmnesiacRoleOutline(roleOutline) {
            this.server.sendPacket({
                type: "setAmnesiacRoleOutline",
                roleOutline: roleOutline
            });
        },
        
        messageListener(serverMessage) {
            messageListener(serverMessage);
        },
    
        tick(timePassedMs) {
            if (gameManager.state.stateType === "game"){
                if (!gameManager.state.ticking) return;

                const newTimeLeft = gameManager.state.timeLeftMs - timePassedMs;
                if (Math.floor(newTimeLeft / 1000) < Math.floor(gameManager.state.timeLeftMs / 1000)) {
                    gameManager.invokeStateListeners("tick");
                }
                gameManager.state.timeLeftMs = newTimeLeft;
                if (gameManager.state.timeLeftMs < 0) {
                    gameManager.state.timeLeftMs = 0;
                }
            }
            
            
            
        },
    }
    return gameManager;
}
function createServer(){

    let Server: Server = {
        ws: null,

        open : () => {
            let address = CONFIG.address;
            Server.ws = new WebSocket(address);

            let completePromise: () => void;
            let promise = new Promise<void>((resolver) => {
                completePromise = resolver;
            });

            Server.ws.onopen = (event: Event)=>{
                completePromise();
                console.log("Connected to server.");
                
                Anchor.setContent(<PlayMenu/>);
            };
            Server.ws.onclose = (event: CloseEvent)=>{
                console.log("Disconnected from server.");
                if (Server.ws === null) return; // We closed it ourselves

                Anchor.pushError(translate("notification.connectionFailed"), "");
                Anchor.setContent(<StartMenu/>);
            };
            Server.ws.onmessage = (event: MessageEvent<string>)=>{
                GAME_MANAGER.messageListener(
                    JSON.parse(event.data) as ToClientPacket
                );
            };
            Server.ws.onerror = (event: Event) => {
                Server.close();
                Anchor.pushError(translate("notification.connectionFailed"), translate("notification.serverNotFound"));
                Anchor.setContent(<StartMenu/>);
            };
            
            return promise;
        },

        sendPacket : (packet: ToServerPacket)=>{
            if (Server.ws === null) {
                console.error("Attempted to send packet to null websocket!");
            } else {
                Server.ws.send(JSON.stringify(packet));
            }
        },

        close : ()=>{
            if(Server.ws === null) return;
            
            Server.ws.close();
            Server.ws = null;
        }
        
    }
    return Server;
}

export function isValidPhaseTime(time: number) {
    return Number.isSafeInteger(time) && time <= 1000 && 0 <= time;
}

export type { GameManager, Server } from "./gameManager.d";
