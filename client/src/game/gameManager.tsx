import { create_gameState } from "./gameState";
import Anchor from "../menu/Anchor";
import StartMenu from "../menu/main/StartMenu";
import GAME_MANAGER from "../index";
import messageListener from "./messageListener";
import CONFIG from "../resources/config.json"
import React from "react";
import { Phase, Player } from "./gameState.d";
import { GameManager, Server, StateListener } from "./gameManager.d";

export function create_gameManager(): GameManager {

    console.log("Game manager created.");
    
    let gameManager: GameManager = {
        roomCode: null,

        name: undefined,

        Server : create_server(),

        gameState : create_gameState(),

        listeners : [],
        addStateListener(listener) {
            gameManager.listeners.push(listener);
        },
        addAndCallStateListener(listener): void {
            gameManager.listeners.push(listener);
            listener(null);
        },
        removeStateListener(listener) {
            gameManager.listeners.splice(gameManager.listeners.indexOf(listener));
        },
        invokeStateListeners(type=null) {
            for(let i = 0; i < gameManager.listeners.length; i++){
                if(typeof(gameManager.listeners[i])==="function"){
                    gameManager.listeners[i](type);
                }
            }
        },

        host_button() {
            gameManager.Server.send(`"Host"`);
        },
        join_button() {
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

            gameManager.Server.send(JSON.stringify({
                "Join":{
                    "room_code": actual_code == null ? 0 : actual_code
                }
            }));

            return promise;
        },

        setName_button(name) {
            // if(name)
                gameManager.Server.send(JSON.stringify({
                    "SetName":{
                        "name":name
                    }
                }));
        },
        startGame_button() {
            gameManager.Server.send(`"StartGame"`);
        },
        phaseTimeButton(phase: Phase, time: number) {
            if (isValidPhaseTime(time)) {
                gameManager.Server.send(JSON.stringify({
                    "SetPhaseTime":{
                        "phase": phase,
                        "time": time
                    }
                }))
            }
        },
        roleList_button(roleListEntries) {
            gameManager.Server.send(JSON.stringify({
                "SetRoleList":{
                    "role_list": {
                        "role_list": roleListEntries
                    }
                }
            }));
        },

        judgement_button(judgement) {
            if(judgement===1) judgement="Innocent";
            if(judgement===-1) judgement="Guilty";
            if(judgement===0) judgement="Abstain";
            gameManager.Server.send(JSON.stringify({
                "Judgement":{
                    "verdict":judgement
                }
            }));
        },
        vote_button(votee_index) {
            gameManager.Server.send(JSON.stringify({
                "Vote":{
                    "player_index":votee_index
                }
            }));
        },
        target_button(target_index_list) {
            gameManager.Server.send(JSON.stringify({
                "Target":{
                    "player_index_list":target_index_list
                }
            }));
        },
        dayTarget_button(target_index) {
            gameManager.Server.send(JSON.stringify({
                "DayTarget":{
                    "player_index":target_index
                }
            }));
        },

        saveWill_button(will) {
            gameManager.Server.send(JSON.stringify({
                "SaveWill":{
                    "will":will
                }
            }));
        },
        sendMessage_button(text) {
            gameManager.Server.send(JSON.stringify({
                "SendMessage":{
                    "text":text
                }
            }));
        },
        sendWhisper_button(playerIndex, text) {
            gameManager.Server.send(JSON.stringify({
                "SendWhisper":{
                    "player_index":playerIndex,
                    "text":text
                }
            }));
        },
        
        messageListener(serverMessage) {
            messageListener(serverMessage);
        },
    
        tick(timePassedms) {
            //console.log("tick");
            console.log(Anchor.instance?.state.content);
            gameManager.gameState.secondsLeft = Math.round(gameManager.gameState.secondsLeft - timePassedms/1000)
            if(gameManager.gameState.secondsLeft < 0)
                gameManager.gameState.secondsLeft = 0;
            gameManager.invokeStateListeners("tick");
        },

        getPlayer(playerIndex: number): Player {
            return this.gameState.players[playerIndex];
        }
    }
    return gameManager;
}
function create_server(){
 

    let Server: Server = {
        ws: null,

        openListener : (event: any)=>{
            //Server.ws.send("Hello to Server");
        },
        closeListener : (event: any)=>{
            Anchor.setContent(<StartMenu/>);
        },
        messageListener: (event: any)=>{
            GAME_MANAGER.messageListener(
                JSON.parse(event.data)
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
        send : (packets)=>{
            if (Server.ws === null) {
                console.log("Attempted to send packet to null websocket!");
            } else {
                Server.ws.send(packets);
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