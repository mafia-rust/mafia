import { DoomsayerGuess } from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeDoomsayerMenu";
import { Phase, PhaseTimes, PlayerIndex, State, Verdict } from "./gameState.d";
import { ToClientPacket, ToServerPacket } from "./packet";
import { RoleList, RoleOutline } from "./roleListState.d";
import { Role } from "./roleState.d";

export type Server = {
    ws: WebSocket | null,

    open(): Promise<void>;
    sendPacket(packets: ToServerPacket): void;
    close(): void;
}

export type StateEventType = ToClientPacket["type"] | "tick" | "filterUpdate";
export type StateListener = (type?: StateEventType) => void;

export type GameManager = {

    setDisconnectedState(): void;
    setLobbyState(): void;
    setGameState(): void;
    setOutsideLobbyState(): void;

    saveReconnectData(roomCode: number, playerId: number): void;
    deleteReconnectData(): void;
    loadReconnectData(): {
        roomCode: number,
        playerId: number,
        lastSaveTime: number,
    } | null;
    
    saveSettings(volume: number): void;
    loadSettings(): {
        volume: number
    } | null;
    

    state: State,
    getMyName(): string | undefined,
    getMyHost(): boolean | undefined,
    getPlayerNames(): string[],

    server: Server,
    listeners: StateListener[],

    addStateListener(listener: StateListener): void;
    removeStateListener(listener: StateListener): void;
    invokeStateListeners(type?: StateEventType): void;

    setPrependWhisperFunction: (f: any) => void;
    prependWhisper: (index: PlayerIndex) => void;

    leaveGame(): void;

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
    sendSetNamePacket(name: string): void;
    sendSetLobbyNamePacket(name: string): void;
    sendStartGamePacket(): Promise<boolean>;
    sendSetPhaseTimePacket(phase: Phase, time: number): void;
    sendSetPhaseTimesPacket(phaseTimeSettings: PhaseTimes): void;
    sendSetRoleListPacket(roleListEntries: RoleList): void;
    sendSetRoleOutlinePacket(index: number, roleOutline: RoleOutline): void;
    sendSimplifyRoleListPacket(): void;
    
    sendJudgementPacket(judgement: Verdict): void;
    sendVotePacket(voteeIndex: PlayerIndex| null): void;
    sendTargetPacket(targetIndexList: number[]): void;
    sendDayTargetPacket(targetIndex: number): void;
    sendSaveWillPacket(will: string): void;
    sendSaveNotesPacket(notes: string): void;
    sendSaveCrossedOutOutlinesPacket(crossedOutOutlines: number[]): void;
    sendSaveDeathNotePacket(notes: string): void;
    sendSendMessagePacket(text: string): void;
    sendSendWhisperPacket(playerIndex: number, text: string): void;
    sendExcludedRolesPacket(roles: Role[]): void;

    sendSetDoomsayerGuess(guesses: [
        [number, DoomsayerGuess],
        [number, DoomsayerGuess],
        [number, DoomsayerGuess]
    ]): void;
    sendSetAmnesiacRoleOutline(roleOutline: RoleOutline): void;
    sendSetJournalistJournal(journal: string): void;
    sendSetJournalistJournalPublic(isPublic: boolean): void;
    sendSetConsortOptions(
        roleblock: boolean, 
        youWereRoleblockedMessage: boolean, 
        youSurvivedAttackMessage: boolean, 
        youWereProtectedMessage: boolean, 
        youWereTransportedMessage: boolean, 
        youWerePossessedMessage: boolean, 
        yourTargetWasJailedMessage: boolean
    ): void
    sendSetForgerWill(role: Role | null, will: string): void;
    sendVoteFastForwardPhase(fastForward: boolean): void;

    messageListener(serverMessage: ToClientPacket): void;

    lastPingTime: number,
    pingCalculation: number,
    tick(timePassedMs: number): void;

}

export declare function createGameManager(): GameManager;