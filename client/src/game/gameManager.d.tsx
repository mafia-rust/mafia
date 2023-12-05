import { DoomsayerGuess } from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeDoomsayerMenu";
import GameState, { Phase, PhaseTimes, PlayerID, PlayerIndex, State, Verdict } from "./gameState.d";
import { ToClientPacket, ToServerPacket } from "./packet";
import { RoleOutline } from "./roleListState.d";

export type Server = {
    ws: WebSocket | null,

    open(): Promise<void>;
    sendPacket(packets: ToServerPacket): void;
    close(): void;
}

export type StateEventType = ToClientPacket["type"] | "tick";
export type StateListener = (type?: StateEventType) => void;

export type GameManager = {
    roomCode: string | null,
    playerId: number | null,
    

    // state: State,
    gameState: GameState,

    server: Server,
    listeners: StateListener[],

    addStateListener(listener: StateListener): void;
    removeStateListener(listener: StateListener): void;
    invokeStateListeners(type?: StateEventType): void;

    tryJoinGame(roomCode: string): Promise<void>;
    leaveGame(): void;

    sendHostPacket(): void;
    sendJoinPacket(): Promise<void>;
    sendSetNamePacket(name: string): void;
    sendStartGamePacket(): void;
    sendSetPhaseTimePacket(phase: Phase, time: number): void;
    sendSetPhaseTimesPacket(phaseTimeSettings: PhaseTimes): void;
    sendSetRoleListPacket(roleListEntries: RoleOutline[]): void;
    sendSetRoleOutlinePacket(index: number, roleOutline: RoleOutline): void;
    
    sendJudgementPacket(judgement: Verdict): void;
    sendVotePacket(voteeIndex: PlayerIndex| null): void;
    sendTargetPacket(targetIndexList: number[]): void;
    sendDayTargetPacket(targetIndex: number): void;
    sendSaveWillPacket(will: string): void;
    sendSaveNotesPacket(notes: string): void;
    sendSaveDeathNotePacket(notes: string): void;
    sendSendMessagePacket(text: string): void;
    sendSendWhisperPacket(playerIndex: number, text: string): void;
    sendExcludedRolesPacket(roles: RoleOutline[]): void;
    sendKickPlayerPacket(playerId: PlayerID): void;

    sendSetDoomsayerGuess(guesses: [
        [number, DoomsayerGuess],
        [number, DoomsayerGuess],
        [number, DoomsayerGuess]
    ]): void;
    sendSetAmnesiacRoleOutline(roleOutline: RoleOutline): void;

    messageListener(serverMessage: ToClientPacket): void;

    tick(timePassedMs: number): void;

}

export declare function createGameManager(): GameManager;