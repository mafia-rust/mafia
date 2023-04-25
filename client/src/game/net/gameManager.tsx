import { create_gameState as createGameState } from "../gameState";
import Anchor from "../../menu/Anchor";
import StartMenu from "../../menu/main/StartMenu";
import GAME_MANAGER from "../../index";
import messageListener from "../net/messageListener";
import CONFIG from "../../resources/config.json"
import React from "react";
import { Phase, Player, RoleListEntry, Verdict } from "../gameState.d";
import { GameManager, Server, StateListener } from "./gameManager.d";
import { ToClientPacket, ToServerPacket } from "./packet";

export function create_gameManager(): GameManager {

    console.log("Game manager created.");
    
    let gameManager: GameManager = {
        roomCode: null,

        name: undefined,

        server : createServer(),

        gameState : createGameState(),

        listeners : [],
        addStateListener(listener) {
            gameManager.listeners.push(listener);
        },
        addAndCallStateListener(listener): void {
            gameManager.listeners.push(listener);
            listener();
        },
        removeStateListener(listener) {
            gameManager.listeners.splice(gameManager.listeners.indexOf(listener));
        },
        invokeStateListeners(type) {
            for(let i = 0; i < gameManager.listeners.length; i++){
                if(typeof(gameManager.listeners[i])==="function"){
                    gameManager.listeners[i](type);
                }
            }
        },

        sendHostPacket() {
            this.server.sendPacket({type: "host"});
        },
        sendJoinPacket() {
            let completePromise: () => void;
            let promise = new Promise<void>((resolver) => {
                completePromise = resolver;
            });
            
            let setName: StateListener = (type: any) => {
                if (type==="AcceptJoin") {
                    completePromise();
                    // This listener shouldn't stick around
                    GAME_MANAGER.removeStateListener(setName);
                }
            };
            GAME_MANAGER.addStateListener(setName);

            let actual_code: number | null = parseInt(gameManager.roomCode!, 18);

            this.server.sendPacket({
                type: "join",
                roomCode: actual_code == null ? 0 : actual_code
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
        phaseTimeButton(phase: Phase, time: number) {
            if (isValidPhaseTime(time)) {
                this.server.sendPacket({
                    type: "setPhaseTime",
                    phase: phase,
                    time: time
                });
            }
        },
        sendSetRoleListPacket(roleListEntries: RoleListEntry[]) {
            this.server.sendPacket({
                type: "setRoleList",
                roleList: roleListEntries
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
        
        messageListener(serverMessage) {
            messageListener(serverMessage);
        },
    
        tick(timePassedms) {
            //console.log("tick");
            gameManager.gameState.secondsLeft = Math.round(gameManager.gameState.secondsLeft - timePassedms/1000)
            if(gameManager.gameState.secondsLeft < 0)
                gameManager.gameState.secondsLeft = 0;
            gameManager.invokeStateListeners("tick");
        },
    }
    return gameManager;
}
function createServer(){

    let Server: Server = {
        ws: null,

        openListener : ()=>{
            //Server.ws.send("Hello to Server");
        },
        closeListener : ()=>{
            Anchor.setContent(<StartMenu/>);
        },
        messageListener: (event: any)=>{
            GAME_MANAGER.messageListener(
                JSON.parse(event.data) as ToClientPacket
            );
        },

        open : ()=>{
            let address = CONFIG.server_ip + ":" + CONFIG.port;
            Server.ws = new WebSocket("ws://"+address);   //TODO

            let completePromise: () => void;
            let promise = new Promise<void>((resolver) => {
                completePromise = resolver;
            });

            Server.ws.addEventListener("open", (event: Event)=>{
                completePromise();
                Server.openListener(event);
            });
            Server.ws.addEventListener("close", (event: Event)=>{
                Server.closeListener(event);
            });
            Server.ws.addEventListener("message", (event: Event)=>{
                Server.messageListener(event);
            });
            
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
            if(Server.ws==null) return;
            
            Server.ws.close();
            Server.ws.removeEventListener("close", Server.closeListener);
            Server.ws.removeEventListener("message", Server.messageListener);
            Server.ws.removeEventListener("open", Server.openListener);
            Server.ws = null;
        }
        
    }
    return Server;
}

export function isValidPhaseTime(time: number) {
    return Number.isSafeInteger(time) && time <= 1000 && 0 <= time;
}

export type { GameManager, Server } from "./gameManager.d";
// export default gameManager;


/*
rust side code of packets i need to make
pub enum ToServerPacket{
    
    Join
    Host

    //
    StartGame,
    Kick,
    SetRoleList,
    SetPhaseTimes,
    SetInvestigatorResults,

    //
    Vote,   //Accusation
    Target,
    DayTarget,
    Judgement,  //Vote
    Whisper,
    SendMessage,
    SaveWill,
}
*/