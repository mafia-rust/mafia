import { DoomsayerGuess } from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeDoomsayerMenu";
import GameState, { Phase, PhaseTimes, PlayerIndex, Verdict } from "./gameState.d";
import { ToClientPacket, ToServerPacket } from "./packet";
import { RoleListEntry } from "./roleListState.d";

export interface Server {
    ws: WebSocket | null,

    open(): Promise<void>;
    sendPacket(packets: ToServerPacket): void;
    close(): void;
}

export type StateEventType = ToClientPacket["type"] | "tick";
export type StateListener = (type?: StateEventType) => void;

export interface GameManager {
    roomCode: string | null,
    
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
    sendSetRoleListPacket(roleListEntries: RoleListEntry[]): void;
    sendSetRoleListEntryPacket(index: number, roleListEntry: RoleListEntry): void;
    
    sendJudgementPacket(judgement: Verdict): void;
    sendVotePacket(voteeIndex: PlayerIndex| null): void;
    sendTargetPacket(targetIndexList: number[]): void;
    sendDayTargetPacket(targetIndex: number): void;
    sendSaveWillPacket(will: string): void;
    sendSaveNotesPacket(notes: string): void;
    sendSaveDeathNotePacket(notes: string): void;
    sendSendMessagePacket(text: string): void;
    sendSendWhisperPacket(playerIndex: number, text: string): void;
    sendExcludedRolesPacket(roles: RoleListEntry[]): void;

    sendSetDoomsayerGuess(guesses: [
        [number, DoomsayerGuess],
        [number, DoomsayerGuess],
        [number, DoomsayerGuess]
    ]): void;
    
    messageListener(serverMessage: ToClientPacket): void;

    tick(timePassedms: number): void;

}

export declare function createGameManager(): GameManager;