import GameState, { Phase, PhaseTimes, Player } from "./gameState.d";

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
    removeStateListener(listener: StateListener): void;
    invokeStateListeners(type: any): void;

    host_button(): void;
    join_button(): Promise<void>;
    setName_button(name: string): void;
    startGame_button(): void;
    phaseTimeButton(phase: Phase, time: number): void;
    roleList_button(roleListEntries: any): void;
    
    judgement_button(judgement: Judgement): void;
    vote_button(votee_index: number): void;
    target_button(target_index_list: number[]): void;
    dayTarget_button(target_index: number): void;
    saveWill_button(will: string): void;
    sendMessage_button(text: string): void;
    sendWhisper_button(playerIndex: number, text: string): void;

    messageListener(serverMessage: ServerMessage): void;

    tick(timePassedms: number): void;

    getPlayer(playerIndex: number): Player | null;
}

export declare function create_gameManager(): GameManager;