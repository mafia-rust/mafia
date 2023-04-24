import GameState, { Phase, PhaseTimes, Player, PlayerIndex, RoleListEntry } from "./gameState.d";

export type ServerMessage = any;

export interface Server {
    ws: WebSocket | null,

    openListener(event: Event): void;
    closeListener(event: Event): void;
    messageListener(event: Event): void;

    open(): Promise<void>;
    send(packets: string | ArrayBufferLike | Blob | ArrayBufferView): void;
    close(): void;
}

// TODO make this better
type Judgement = "Innocent" | "Guilty" | "Abstain" | -1 | 0 | 1;

export type StateListener = (type: any) => void;

export interface GameManager {
    roomCode: string | null,
    name: string | undefined,
    Server: Server,
    gameState: GameState,
    listeners: StateListener[],

    addStateListener(listener: StateListener): void;
    addAndCallStateListener(listener: StateListener): void;
    removeStateListener(listener: StateListener): void;
    invokeStateListeners(type: any): void;

    sendHostPacket(): void;
    sendJoinPacket(): Promise<void>;
    sendSetNamePacket(name: string): void;
    sendStartGamePacket(): void;
    phaseTimeButton(phase: Phase, time: number): void;
    sendSetRoleListPacket(roleListEntries: RoleListEntry[]): void;
    
    sendJudgementPacket(judgement: Judgement): void;
    sendVotePacket(votee_index: PlayerIndex| null): void;
    sendTargetPacket(target_index_list: number[]): void;
    sendDayTargetPacket(target_index: number): void;
    sendSaveWillPacket(will: string): void;
    sendSendMessagePacket(text: string): void;
    sendSendWhisperPacket(playerIndex: number, text: string): void;
    
    messageListener(serverMessage: ServerMessage): void;

    tick(timePassedms: number): void;

}

export declare function create_gameManager(): GameManager;