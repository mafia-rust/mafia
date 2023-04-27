import GameState, { Phase, PhaseTimes, Player, PlayerIndex, RoleListEntry, Verdict } from "../gameState.d";
import { ToClientPacket, ToServerPacket } from "./packet";

export type ServerMessage = any;

export interface Server {
    ws: WebSocket | null,

    openListener(event: Event): void;
    closeListener(event: Event): void;
    messageListener(event: Event): void;

    open(): Promise<void>;
    sendPacket(packets: ToServerPacket): void;
    close(): void;
}

export type StateEventType = ToClientPacket["type"] | undefined | "tick";
export type StateListener = (type?: StateEventType) => void;

export interface GameManager {
    roomCode: string | null,
    name: string | undefined,

    willMenuOpen: boolean,
    wikiMenuOpen: boolean,
    graveyardMenuOpen: boolean,
    playerListMenuOpen: boolean,
    
    gameState: GameState,

    server: Server,
    listeners: StateListener[],

    addStateListener(listener: StateListener): void;
    addAndCallStateListener(listener: StateListener): void;
    removeStateListener(listener: StateListener): void;
    invokeStateListeners(type?: StateEventType): void;

    sendHostPacket(): void;
    sendJoinPacket(): Promise<void>;
    sendSetNamePacket(name: string): void;
    sendStartGamePacket(): void;
    phaseTimeButton(phase: Phase, time: number): void;
    sendSetRoleListPacket(roleListEntries: RoleListEntry[]): void;
    
    sendJudgementPacket(judgement: Verdict): void;
    sendVotePacket(voteeIndex: PlayerIndex| null): void;
    sendTargetPacket(targetIndexList: number[]): void;
    sendDayTargetPacket(targetIndex: number): void;
    sendSaveWillPacket(will: string): void;
    sendSendMessagePacket(text: string): void;
    sendSendWhisperPacket(playerIndex: number, text: string): void;
    
    messageListener(serverMessage: ToClientPacket): void;

    tick(timePassedms: number): void;

}

export declare function createGameManager(): GameManager;