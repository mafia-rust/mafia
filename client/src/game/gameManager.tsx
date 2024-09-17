import { ANCHOR_CONTROLLER } from "./../menu/Anchor";
import StartMenu from "./../menu/main/StartMenu";
import GAME_MANAGER from "./../index";
import messageListener from "./messageListener";
import CONFIG from "./../resources/config.json"
import React from "react";
import { PhaseType, PhaseTimes, Verdict, Player, PlayerIndex } from "./gameState.d";
import { GameManager, Server, StateListener } from "./gameManager.d";
import { LobbyPreviewData, ToClientPacket, ToServerPacket } from "./packet";
import { RoleOutline } from "./roleListState.d";
import translate from "./lang";
import PlayMenu from "../menu/main/PlayMenu";
import { createGameState, createLobbyState } from "./gameState";
import { Role } from "./roleState.d";
import DUMMY_NAMES from "../resources/dummyNames.json";
import { deleteReconnectData } from "./localStorage";
import { KiraGuess } from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeKiraMenu";
import AudioController from "../menu/AudioController";
export function createGameManager(): GameManager {

    console.log("Game manager created.");
    
    let gameManager: GameManager = {
        async setDisconnectedState(): Promise<void> {
            AudioController.clearQueue();
            AudioController.pauseQueue();

            if (GAME_MANAGER.server.ws) {
                let completePromise: () => void;
                const promise = new Promise<void>((resolver) => {
                    completePromise = resolver;
                });

                GAME_MANAGER.server.ws?.addEventListener("close", () => completePromise());
                GAME_MANAGER.server.close();

                GAME_MANAGER.state = {
                    stateType: "disconnected"
                };
                return promise;
            } else {
                GAME_MANAGER.state = {
                    stateType: "disconnected"
                };
                return Promise.resolve();
            }
        },
        setLobbyState() {
            
            let gameState = null
            if (GAME_MANAGER.state.stateType === "game") {
                gameState = {...GAME_MANAGER.state};
            }

            GAME_MANAGER.state = createLobbyState();

            if(gameState!=null){
                GAME_MANAGER.state.roomCode = gameState.roomCode;
                GAME_MANAGER.state.lobbyName = gameState.lobbyName;
                GAME_MANAGER.state.roleList = gameState.roleList;
                GAME_MANAGER.state.phaseTimes = gameState.phaseTimes;
                GAME_MANAGER.state.enabledRoles = gameState.enabledRoles;
            }
        },
        setGameState() {

            let lobbyState = null;
            if (GAME_MANAGER.state.stateType === "lobby") {
                lobbyState = {...GAME_MANAGER.state};
            }


            AudioController.clearQueue();
            AudioController.unpauseQueue();
            GAME_MANAGER.state = createGameState();
            if (lobbyState !== null && GAME_MANAGER.state.stateType === "game") {
                GAME_MANAGER.state.roomCode = lobbyState.roomCode;
                GAME_MANAGER.state.lobbyName = lobbyState.lobbyName;
                GAME_MANAGER.state.roleList = lobbyState.roleList;
                GAME_MANAGER.state.phaseTimes = lobbyState.phaseTimes;
                GAME_MANAGER.state.enabledRoles = lobbyState.enabledRoles;
                GAME_MANAGER.state.host = lobbyState.players.get(lobbyState.myId!)?.host ?? false;
            }
        },
        setSpectatorGameState() {
            this.setGameState();
            if(GAME_MANAGER.state.stateType === "game")
                GAME_MANAGER.state.clientState = {
                    type: "spectator"
                };
        },
        async setOutsideLobbyState() {
            AudioController.clearQueue();
            AudioController.pauseQueue();
            
            if (!GAME_MANAGER.server.ws?.OPEN) {
                await GAME_MANAGER.server.open();
            }

            GAME_MANAGER.state = {
                stateType: "outsideLobby",
                selectedRoomCode: null,
                lobbies: new Map<number, LobbyPreviewData>()
            };
        },

        state: {
            stateType: "disconnected"
        },

        getMyName() {
            if (gameManager.state.stateType === "lobby"){
                let client = gameManager.state.players.get(gameManager.state.myId!);
                if(client === undefined) return undefined;
                if(client.clientType.type === "spectator") return undefined;
                return client.clientType.name;
            }
            if (gameManager.state.stateType === "game" && gameManager.state.clientState.type === "player")
                return gameManager.state.players[gameManager.state.clientState.myIndex!]?.name;
            return undefined;
        },
        getMyHost() {
            if (gameManager.state.stateType === "lobby")
                return gameManager.state.players.get(gameManager.state.myId!)?.host;
            if (gameManager.state.stateType === "game")
                return gameManager.state.host;
            return undefined;
        },
        getMySpectator() {
            if (gameManager.state.stateType === "lobby")
                return gameManager.state.players.get(gameManager.state.myId!)?.clientType.type === "spectator";
            if (gameManager.state.stateType === "game")
                return gameManager.state.clientState.type === "spectator";
            return false;
        },
        getPlayerNames(): string[] {
            switch (GAME_MANAGER.state.stateType) {
                case "game":
                    return GAME_MANAGER.state.players.map((player) => player.toString());
                case "lobby":
                    return [];
                default:
                    return DUMMY_NAMES;
            }
        },
        getLivingPlayers(): Player[] | null{
            if(GAME_MANAGER.state.stateType !== "game") return null;
            return GAME_MANAGER.state.players.filter(player => player.alive)
        },
        getVotesRequired(): number | null{            
            let count = 1;
            let livingPlayers = GAME_MANAGER.getLivingPlayers();
            if(livingPlayers === null) return null;
            for (let player of livingPlayers) {
                if (player.alive && !player.playerTags.includes("forfeitVote")) {
                    count += 1;
                }
            }


            return Math.ceil(count / 2);
        },
        updateChatFilter(filter: PlayerIndex | null) {
            if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player"){
                GAME_MANAGER.state.clientState.chatFilter = filter;
                GAME_MANAGER.invokeStateListeners("filterUpdate");
            }
        },


        server: createServer(),

        listeners: [],

        addStateListener(listener) {
            gameManager.listeners.push(listener);
        },
        removeStateListener(listener) {
            let index = gameManager.listeners.indexOf(listener);
            if (index !== -1)
                gameManager.listeners.splice(index, 1);
        },
        invokeStateListeners(type) {
            for (let i = 0; i < gameManager.listeners.length; i++) {
                if (typeof (gameManager.listeners[i]) === "function") {
                    gameManager.listeners[i](type);
                }
            }
        },

        setPrependWhisperFunction: (f) => {
            gameManager.prependWhisper = f;
        },
        prependWhisper: (index) => {},
        
        wikiArticleCallbacks: [],
        addSetWikiArticleCallback: (callback) => {
            gameManager.wikiArticleCallbacks.push(callback);
        },
        removeSetWikiArticleCallback: (callback) => {
            gameManager.wikiArticleCallbacks.splice(gameManager.wikiArticleCallbacks.indexOf(callback), 1)
        },
        setWikiArticle: (article) => {
            for (const callback of gameManager.wikiArticleCallbacks) {
                callback(article);
            }
        },


        leaveGame() {
            if (this.state.stateType !== "disconnected") {
                this.server.sendPacket({ type: "leave" });
            }
            deleteReconnectData();
            this.setOutsideLobbyState();
            ANCHOR_CONTROLLER?.setContent(<PlayMenu/>);
        },

        sendLobbyListRequest() {
            this.server.sendPacket({ type: "lobbyListRequest" });
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
            this.server.sendPacket({ type: "host" });

            return promise;
        },
        sendRejoinPacket(roomCode: number, playerId: number) {
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

            this.server.sendPacket({
                type: "reJoin",
                roomCode,
                playerId
            });


            return promise;
        },
        sendJoinPacket(roomCode: number) {
            let completePromise: (success: boolean) => void;
            let promise = new Promise<boolean>((resolver) => {
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

            this.server.sendPacket({
                type: "join",
                roomCode
            });

            return promise;
        },
        sendKickPlayerPacket(playerId: number) {
            this.server.sendPacket({
                type: "kick",
                playerId: playerId
            });
        },

        sendSetSpectatorPacket(spectator) {
            this.server.sendPacket({
                type: "setSpectator",
                spectator: spectator
            });
        },

        sendSetNamePacket(name) {
            this.server.sendPacket({
                type: "setName",
                name: name
            });
        },
        sendSendLobbyMessagePacket(text) {
            this.server.sendPacket({
                type: "sendLobbyMessage",
                text: text
            });
        },

        sendSetLobbyNamePacket(name) {
            this.server.sendPacket({
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

            this.server.sendPacket({
                type: "startGame"
            });

            return promise;
        },
        sendBackToLobbyPacket() {
            this.server.sendPacket({
                type: "backToLobby"
            });
        },
        sendSetPhaseTimePacket(phase: PhaseType, time: number) {
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
        sendSimplifyRoleListPacket() {
            this.server.sendPacket({
                type: "simplifyRoleList"
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
        sendSaveCrossedOutOutlinesPacket(crossedOutOutlines) {
            this.server.sendPacket({
                type: "saveCrossedOutOutlines",
                crossedOutOutlines: crossedOutOutlines
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
        sendEnabledRolesPacket(roles) {
            this.server.sendPacket({
                type: "setEnabledRoles",
                roles: roles
            });
        },

        sendSetDoomsayerGuess(guesses) {
            this.server.sendPacket({
                type: "setDoomsayerGuess",
                guesses: guesses
            });
        },
        sendSetKiraGuess(guessesRecord) {

            let guesses: [PlayerIndex, KiraGuess][] = [];
            for(let [player, guess] of Object.entries(guessesRecord)){
                guesses.push([Number.parseInt(player), guess]);
            }

            this.server.sendPacket({
                type: "setKiraGuess",
                guesses: guesses
            });
        },
        sendSetWildcardRoleOutline(role) {
            this.server.sendPacket({
                type: "setWildcardRole",
                role: role
            });
        },
        sendSetJournalistJournal(journal: string) {
            this.server.sendPacket({
                type: "setJournalistJournal",
                journal: journal,
            });
        },
        sendSetJournalistJournalPublic(isPublic: boolean) {
            this.server.sendPacket({
                type: "setJournalistJournalPublic",
                public: isPublic,
            });
        },
        sendSetConsortOptions(
            roleblock: boolean,
            youWereRoleblockedMessage: boolean,
            youSurvivedAttackMessage: boolean,
            youWereProtectedMessage: boolean,
            youWereTransportedMessage: boolean,
            youWerePossessedMessage: boolean,
            yourTargetWasJailedMessage: boolean
        ): void {
            this.server.sendPacket({
                type: "setConsortOptions",
                roleblock: roleblock,

                youWereRoleblockedMessage: youWereRoleblockedMessage ?? false,
                youSurvivedAttackMessage: youSurvivedAttackMessage ?? false,
                youWereProtectedMessage: youWereProtectedMessage ?? false,
                youWereTransportedMessage: youWereTransportedMessage ?? false,
                youWerePossessedMessage: youWerePossessedMessage ?? false,
                yourTargetWasJailedMessage: yourTargetWasJailedMessage ?? false
            });
        },
        sendSetForgerWill(role: Role | null, will: string) {
            this.server.sendPacket({
                type: "setForgerWill",
                role: role,
                will: will
            });
        },
        sendSetCounterfeiterAction(action: "forge" | "noForge") {
            this.server.sendPacket({
                type: "setCounterfeiterAction",
                action: action
            });
        },
        sendSetAuditorChosenOutline(index) {
            this.server.sendPacket({
                type: "setAuditorChosenOutline",
                index: index
            });
        },
        sendSetOjoAction(action) {
            this.server.sendPacket({
                type: "setOjoAction",
                action: action
            });
        },
        sendSetPuppeteerAction(action) {
            this.server.sendPacket({
                type: "setPuppeteerAction",
                action: action
            });
        },
        sendSetErosAction(action) {
            this.server.sendPacket({
                type: "setErosAction",
                action: action
            });
        },
        sendRetrainerRetrain(role) {
            this.server.sendPacket({
                type: "retrainerRetrain",
                role: role
            });
        },

        sendVoteFastForwardPhase(fastForward: boolean) {
            this.server.sendPacket({
                type: "voteFastForwardPhase",
                fastForward: fastForward
            });
        },
        sendForfeitVotePacket(forfeit: boolean) {
            this.server.sendPacket({
                type: "forfeitVote",
                forfeit
            });
        },

        messageListener(serverMessage) {
            messageListener(serverMessage);
        },

        lastPingTime: 0,
        pingCalculation: 0,
        tick(timePassedMs) {
            if (gameManager.state.stateType !== "disconnected") {
                if(gameManager.lastPingTime + (30 * 1000) < Date.now()){
                    gameManager.lastPingTime = Date.now();
                    this.server.sendPacket({
                        type: "ping"
                    });
                }
            }
            if (gameManager.state.stateType === "game") {
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
            };
            Server.ws.onclose = (event: CloseEvent)=>{
                console.log("Disconnected from server.");
                if (Server.ws === null) return; // We closed it ourselves

                ANCHOR_CONTROLLER?.pushErrorCard({
                    title: translate("notification.connectionFailed"), 
                    body: ""
                });
                ANCHOR_CONTROLLER?.setContent(<StartMenu/>);
            };
            Server.ws.onmessage = (event: MessageEvent<string>)=>{
                GAME_MANAGER.messageListener(
                    JSON.parse(event.data) as ToClientPacket
                );
            };
            Server.ws.onerror = (event: Event) => {
                Server.close();
                ANCHOR_CONTROLLER?.pushErrorCard({
                    title: translate("notification.connectionFailed"), 
                    body: translate("notification.serverNotFound")
                });
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
