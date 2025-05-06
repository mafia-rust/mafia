import { createContext, ReactElement, useContext, useState } from "react";
import { ToClientPacket, ToServerPacket } from "../game/packet";
import { DoomsayerGuess } from "./game/gameScreenContent/AbilityMenu/RoleSpecificMenus/LargeDoomsayerMenu";
import { AbilityInput } from "../game/abilityInput";
import { ModifierType, PhaseTimes, PhaseType, Verdict } from "../game/gameState.d";
import { Role } from "../game/roleState.d";
import { RoleList, RoleOutline } from "../game/roleListState.d";
import { AnchorContext } from "./AnchorContext";
import StartMenu from "./main/StartMenu";
import React from "react";
import translate from "../game/lang";
import { isValidPhaseTime } from "../game/gameManager";

export const WebsocketContext = createContext<WebSocketContext | undefined>(undefined);

type WebSocketContext = {
    webSocket: WebSocket | null,

    open(): Promise<boolean>;
    sendPacket(packets: ToServerPacket): void;
    close(): void;

    sendLobbyListRequest(): void;
    /**
     * @returns A promise that will be fulfilled as true if the join was 
     *          successful and false if the join was unsuccessful
     */
    sendHostPacket(): Promise<boolean>;
    /**
     * @returns A promise that will be fulfilled as true if the join was 
     *          successful and false if the join was unsuccessful
     */
    sendRejoinPacket(roomCode: number, playerId: number): Promise<boolean>;
    /**
     * @returns A promise that will be fulfilled as true if the join was 
     *          successful and false if the join was unsuccessful
     */
    sendJoinPacket(roomCode: number): Promise<boolean>;
    sendKickPlayerPacket(playerId: number): void;
    sendSetPlayerHostPacket(playerId: number): void;
    sendRelinquishHostPacket(): void;
    sendSetSpectatorPacket(spectator: boolean): void;
    sendSetNamePacket(name: string): void;
    sendReadyUpPacket(ready: boolean): void;
    sendSendLobbyMessagePacket(text: string): void;
    sendSetLobbyNamePacket(name: string): void;
    sendStartGamePacket(): Promise<boolean>;
    sendBackToLobbyPacket(): void;
    sendSetPhaseTimePacket(phase: PhaseType, time: number): void;
    sendSetPhaseTimesPacket(phaseTimeSettings: PhaseTimes): void;
    sendSetRoleListPacket(roleListEntries: RoleList): void;
    sendSetRoleOutlinePacket(index: number, roleOutline: RoleOutline): void;
    sendSimplifyRoleListPacket(): void;
    
    sendJudgementPacket(judgement: Verdict): void;
    sendSaveWillPacket(will: string): void;
    sendSaveNotesPacket(notes: string[]): void;
    sendSaveCrossedOutOutlinesPacket(crossedOutOutlines: number[]): void;
    sendSaveDeathNotePacket(notes: string): void;
    sendSendChatMessagePacket(text: string, block: boolean): void;
    sendSendWhisperPacket(playerIndex: number, text: string): void;
    sendEnabledRolesPacket(roles: Role[]): void;
    sendEnabledModifiersPacket(modifiers: ModifierType[]): void;

    sendAbilityInput(input: AbilityInput): void;
    sendSetDoomsayerGuess(guesses: [
        [number, DoomsayerGuess],
        [number, DoomsayerGuess],
        [number, DoomsayerGuess]
    ]): void;
    sendSetConsortOptions(
        roleblock: boolean, 
        youWereRoleblockedMessage: boolean, 
        youSurvivedAttackMessage: boolean, 
        youWereGuardedMessage: boolean, 
        youWereTransportedMessage: boolean, 
        youWerePossessedMessage: boolean, 
        yourTargetWasJailedMessage: boolean
    ): void

    sendVoteFastForwardPhase(fastForward: boolean): void;
    sendHostDataRequest(): void;
    sendHostEndGamePacket(): void;
    sendHostSkipPhase(): void;
    sendHostSetPlayerNamePacket(player_id: number, name: string): void;
}
export function useWebSocketContext(){
    const anchorContext = useContext(AnchorContext)!;

    const defaultContext: WebSocketContext = {
        webSocket: null,

        open : () => {
            let address = process.env.REACT_APP_WS_ADDRESS;
            if(!address){
                throw new Error("Missing env var REACT_APP_WS_ADDRES, make sure you defined it in .env");
            }
            try {
                defaultContext.webSocket = new WebSocket(address);
            } catch {
                return Promise.resolve(false);
            }

            let completePromise: (value: boolean) => void;
            const promise = Promise.race([
                new Promise<boolean>((resolver) => {
                    completePromise = resolver;
                }),
                new Promise<boolean>((resolver) => {
                    setTimeout(() => {
                        resolver(false)
                    }, 3000)
                })
            ]);

            defaultContext.webSocket.onopen = (event: Event)=>{
                completePromise(true);
                console.log("Connected to server.");
            };
            defaultContext.webSocket.onclose = (event: CloseEvent)=>{
                console.log("Disconnected from server.");
                completePromise(false);
                GAME_MANAGER.invokeStateListeners("connectionClosed");
                if (defaultContext.webSocket === null) return; // We closed it ourselves
                defaultContext.webSocket = null;

                anchorContext?.pushErrorCard({
                    title: translate("notification.connectionFailed"), 
                    body: ""
                });
                anchorContext.setContent(<StartMenu/>);
            };
            defaultContext.webSocket.onmessage = (event: MessageEvent<string>)=>{
                GAME_MANAGER.messageListener(
                    JSON.parse(event.data) as ToClientPacket
                );
            };
            defaultContext.webSocket.onerror = (event: Event) => {
                defaultContext.close();
                completePromise(false);
                anchorContext.pushErrorCard({
                    title: translate("notification.connectionFailed"), 
                    body: translate("notification.serverNotFound")
                });
            };
            
            return promise;
        },

        sendPacket : (packet: ToServerPacket)=>{
            if (defaultContext.webSocket === null) {
                console.error("Attempted to send packet to null websocket!");
            } else {
                defaultContext.webSocket.send(JSON.stringify(packet));
            }
        },

        close : ()=>{
            if(defaultContext.webSocket === null) return;
            
            defaultContext.webSocket.close();
            defaultContext.webSocket = null;
        }


        sendLobbyListRequest() {
            defaultContext.sendPacket({ type: "lobbyListRequest" });
        },
        sendHostPacket() {
            let completePromise: (success: boolean) => void;
            const promise = new Promise<boolean>((resolver) => {
                completePromise = resolver;
            });
            let onJoined: StateListener = (type) => {
                if (type === "acceptJoin") {
                    completePromise(true);
                    GAME_MANAGER.removeStateListener(onJoined);
                } else if (type === "rejectJoin") {
                    completePromise(false);
                    GAME_MANAGER.removeStateListener(onJoined);
                }
            };
            GAME_MANAGER.addStateListener(onJoined);
            defaultContext.sendPacket({ type: "host" });
    
            return promise;
        },
        sendRejoinPacket(roomCode: number, playerId: number) {
            let completePromise: (success: boolean) => void;
            const promise = new Promise<boolean>((resolver) => {
                completePromise = resolver;
            });
            const onJoined: StateListener = (type) => {
                if (type === "acceptJoin") {
                    completePromise(true);
                    GAME_MANAGER.removeStateListener(onJoined);
                } else if (type === "rejectJoin") {
                    completePromise(false);
                    GAME_MANAGER.removeStateListener(onJoined);
                } else if (type === "connectionClosed") {
                    completePromise(false);
                    GAME_MANAGER.removeStateListener(onJoined);
                }
            };
            GAME_MANAGER.addStateListener(onJoined);
    
            defaultContext.sendPacket({
                type: "reJoin",
                roomCode,
                playerId
            });
    
    
            return promise;
        },
        sendJoinPacket(roomCode: number) {
            let completePromise: (success: boolean) => void;
            const promise = new Promise<boolean>((resolver) => {
                completePromise = resolver;
            });
            const onJoined: StateListener = (type) => {
                if (type === "acceptJoin") {
                    completePromise(true);
                    GAME_MANAGER.removeStateListener(onJoined);
                } else if (type === "rejectJoin") {
                    completePromise(false);
                    GAME_MANAGER.removeStateListener(onJoined);
                } else if (type === "connectionClosed") {
                    completePromise(false);
                    GAME_MANAGER.removeStateListener(onJoined);
                }
            };
            GAME_MANAGER.addStateListener(onJoined);
    
            defaultContext.sendPacket({
                type: "join",
                roomCode
            });
    
            return promise;
        },
        sendKickPlayerPacket(playerId: number) {
            defaultContext.sendPacket({
                type: "kick",
                playerId: playerId
            });
        },
        sendSetPlayerHostPacket(playerId: number) {
            defaultContext.sendPacket({
                type: "setPlayerHost",
                playerId: playerId
            });
        },
        sendRelinquishHostPacket() {
            defaultContext.sendPacket({
                type: "relinquishHost",
            });
        },
    
        sendSetSpectatorPacket(spectator) {
            defaultContext.sendPacket({
                type: "setSpectator",
                spectator: spectator
            });
        },
    
        sendSetNamePacket(name) {
            defaultContext.sendPacket({
                type: "setName",
                name: name
            });
        },
    
        sendReadyUpPacket(ready) {
            defaultContext.sendPacket({
                type: "readyUp",
                ready: ready
            });
        },
        sendSendLobbyMessagePacket(text) {
            defaultContext.sendPacket({
                type: "sendLobbyMessage",
                text: text
            });
        },
    
        sendSetLobbyNamePacket(name) {
            defaultContext.sendPacket({
                type: "setLobbyName",
                name: name
            });
        },
        sendStartGamePacket() {
            let completePromise: (success: boolean) => void;
            let promise = new Promise<boolean>((resolver) => {
                completePromise = resolver;
            });
            let onJoined: StateListener = (type) => {
                if (type === "startGame") {
                    completePromise(true);
                    GAME_MANAGER.removeStateListener(onJoined);
                } else if (type === "rejectStart") {
                    completePromise(false);
                    GAME_MANAGER.removeStateListener(onJoined);
                }
            };
            GAME_MANAGER.addStateListener(onJoined);
    
            defaultContext.sendPacket({
                type: "startGame"
            });
    
            return promise;
        },
        sendBackToLobbyPacket() {
            defaultContext.sendPacket({
                type: "hostForceBackToLobby"
            });
        },
        sendSetPhaseTimePacket(phase: PhaseType, time: number) {
            if (isValidPhaseTime(time)) {
                defaultContext.sendPacket({
                    type: "setPhaseTime",
                    phase: phase,
                    time: time
                });
            }
        },
        sendSetPhaseTimesPacket(phaseTimeSettings: PhaseTimes) {
            defaultContext.sendPacket({
                type: "setPhaseTimes",
                phaseTimeSettings
            });
        },
        sendSetRoleListPacket(roleListEntries: RoleOutline[]) {
            defaultContext.sendPacket({
                type: "setRoleList",
                roleList: roleListEntries
            });
        },
        sendSetRoleOutlinePacket(index: number, roleOutline: RoleOutline) {
            defaultContext.sendPacket({
                type: "setRoleOutline",
                index,
                roleOutline
            });
        },
        sendSimplifyRoleListPacket() {
            defaultContext.sendPacket({
                type: "simplifyRoleList"
            });
        },
    
        sendJudgementPacket(judgement: Verdict) {
            defaultContext.sendPacket({
                type: "judgement",
                verdict: judgement
            });
        },
    
        sendSaveWillPacket(will) {
            defaultContext.sendPacket({
                type: "saveWill",
                will: will
            });
        },
        sendSaveNotesPacket(notes) {
            defaultContext.sendPacket({
                type: "saveNotes",
                notes: notes
            });
        },
        sendSaveCrossedOutOutlinesPacket(crossedOutOutlines) {
            defaultContext.sendPacket({
                type: "saveCrossedOutOutlines",
                crossedOutOutlines: crossedOutOutlines
            });
        },
        sendSaveDeathNotePacket(notes) {
            defaultContext.sendPacket({
                type: "saveDeathNote",
                deathNote: notes.trim().length === 0 ? null : notes
            });
        },
        sendSendChatMessagePacket(text, block) {
            defaultContext.sendPacket({
                type: "sendChatMessage",
                text: text,
                block: block
            });
        },
        sendSendWhisperPacket(playerIndex, text) {
            defaultContext.sendPacket({
                type: "sendWhisper",
                playerIndex: playerIndex,
                text: text
            });
        },
        sendEnabledRolesPacket(roles) {
            defaultContext.sendPacket({
                type: "setEnabledRoles",
                roles: roles
            });
        },
        sendEnabledModifiersPacket(modifiers) {
            defaultContext.sendPacket({
                type: "setEnabledModifiers",
                modifiers: modifiers
            });
        },
    
        sendAbilityInput(input) {
            defaultContext.sendPacket({
                type: "abilityInput",
                abilityInput: input
            });
        },
        sendSetDoomsayerGuess(guesses) {
            defaultContext.sendPacket({
                type: "setDoomsayerGuess",
                guesses: guesses
            });
        },
        sendSetConsortOptions(
            roleblock: boolean,
            youWereRoleblockedMessage: boolean,
            youSurvivedAttackMessage: boolean,
            youWereGuardedMessage: boolean,
            youWereTransportedMessage: boolean,
            youWerePossessedMessage: boolean,
            youWereWardblockedMessage: boolean
        ): void {
            defaultContext.sendPacket({
                type: "setConsortOptions",
                roleblock: roleblock,
    
                youWereRoleblockedMessage: youWereRoleblockedMessage ?? false,
                youSurvivedAttackMessage: youSurvivedAttackMessage ?? false,
                youWereGuardedMessage: youWereGuardedMessage ?? false,
                youWereTransportedMessage: youWereTransportedMessage ?? false,
                youWerePossessedMessage: youWerePossessedMessage ?? false,
                youWereWardblockedMessage: youWereWardblockedMessage ?? false
            });
        },
    
        sendVoteFastForwardPhase(fastForward: boolean) {
            defaultContext.sendPacket({
                type: "voteFastForwardPhase",
                fastForward: fastForward
            });
        },
    
        sendHostDataRequest() {
            defaultContext.sendPacket({
                type: "hostDataRequest"
            })
        },
        sendHostEndGamePacket() {
            defaultContext.sendPacket({
                type: "hostForceEndGame"
            })
        },
        sendHostSkipPhase() {
            defaultContext.sendPacket({
                type: "hostForceSkipPhase"
            })
        },
        sendHostSetPlayerNamePacket(playerId, name) {
            defaultContext.sendPacket({
                type: "hostForceSetPlayerName",
                id: playerId,
                name
            })
        }
    }

    const [wsCtx, setWsCtx] = useState<WebSocketContext>(defaultContext);

    return wsCtx;
}