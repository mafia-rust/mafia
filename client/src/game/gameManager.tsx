import { createGameState } from "./gameState";
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

export function createGameManager(): GameManager {

    console.log("Game manager created.");
    
    let gameManager: GameManager = {
        roomCode: null,
        playerId: null,

        gameState : createGameState(),

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

        async tryJoinGame(roomCode: string) {
            GAME_MANAGER.roomCode = roomCode;
            
            GAME_MANAGER.server.close();
            await GAME_MANAGER.server.open();
            
            await GAME_MANAGER.sendJoinPacket();
        },

        leaveGame() {
            if (this.gameState.inGame) {
                this.server.sendPacket({type: "leave"});
            }
            // Set URL to main menu and refresh
            window.history.replaceState({}, document.title, window.location.pathname);
            window.location.reload();
        },

        sendHostPacket() {
            this.server.sendPacket({type: "host"});
        },
        sendJoinPacket() {
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

            let actualCode: number = parseInt(gameManager.roomCode!, 18);

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
        sendKickPlayerPacket(playerId){
            this.server.sendPacket({
                type:"kickPlayer",
                playerId:playerId
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
            const newTimeLeft = gameManager.gameState.timeLeftMs - timePassedMs;
            if (Math.floor(newTimeLeft / 1000) < Math.floor(gameManager.gameState.timeLeftMs / 1000)) {
                gameManager.invokeStateListeners("tick");
            }
            gameManager.gameState.timeLeftMs = newTimeLeft;
            if (gameManager.gameState.timeLeftMs < 0) {
                gameManager.gameState.timeLeftMs = 0;
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
            };
            Server.ws.onclose = (event: CloseEvent)=>{
                if (Server.ws === null) return; // We closed it ourselves

                Anchor.pushInfo("Connection closed", "The connection to the server was closed.")
                Anchor.setContent(<StartMenu/>);
            };
            Server.ws.onmessage = (event: MessageEvent<string>)=>{
                GAME_MANAGER.messageListener(
                    JSON.parse(event.data) as ToClientPacket
                );
            };
            Server.ws.onerror = (event: Event) => {
                Server.ws = null;
                Anchor.pushInfo("Failed to connect", "Contact an admin to see if the server is online.");
                Anchor.setContent(<StartMenu/>);
            };
            
            return promise;
        },

        sendPacket : (packet: ToServerPacket)=>{
            if (Server.ws === null) {
                console.log("Attempted to send packet to null websocket!");
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
