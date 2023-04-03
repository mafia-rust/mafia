import GameState, { Phase, PhaseTimes, Player } from "./gameState.d";

export interface GameManager {
    roomCode: string | null,
    name: string | undefined,
    Server: any,
    gameState: GameState,
    listeners: any[],

    addStateListener(listener: any): void;
    removeStateListener(listener: any): void;
    invokeStateListeners(type: any): void;

    host_button(): void;
    join_button(): void;
    setName_button(name: string): void;
    startGame_button(): void;
    phaseTimeButton(phase: Phase, time: number): void;
    roleList_button(roleListEntries: any): void;
    
    judgement_button(judgement: any): void;
    vote_button(votee_index: any): void;
    target_button(target_index_list: number[]): void;
    dayTarget_button(target_index: number): void;
    saveWill_button(will: string): void;
    sendMessage_button(text: string): void;
    sendWhisper_button(playerIndex: number, text: string): void;

    messageListener(serverMessage: any): void;

    tick(timePassedms: number): void;

    getPlayer(playerIndex: number): Player | null;
}

export declare function create_gameManager(): GameManager;

export interface Server {
    ws: any,

    openListener(event: any): void;
    closeListener(event: any): void;
    messageListener(event: any): void;

    open(): void;
    send(packets: any): void;
    close(): void;
}