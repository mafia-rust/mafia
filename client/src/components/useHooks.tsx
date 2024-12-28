import { useEffect, useState } from "react";
import GAME_MANAGER from "..";
import { StateEventType } from "../game/gameManager.d";
import GameState, { LobbyState, PlayerGameState } from "../game/gameState.d";

function usePacketListener(listener: (type?: StateEventType) => void) {
    // Catch all the packets we miss between setState and useEffect
    const packets: StateEventType[] = [];
    const packetQueueListener = (type?: StateEventType) => {
        if (type) packets.push(type);
    };
    GAME_MANAGER.addStateListener(packetQueueListener)

    useEffect(() => {
        GAME_MANAGER.removeStateListener(packetQueueListener);

        for (const packet of packets) {
            listener(packet);
        }

        GAME_MANAGER.addStateListener(listener);
        return () => GAME_MANAGER.removeStateListener(listener);
    });
}

// https://stackoverflow.com/a/77278013/9157590
function deepEqual<T>(a: T, b: T): boolean {
    if (a === b) {
        return true;
    }

    const bothAreObjects =
        a && b && typeof a === "object" && typeof b === "object";

    return Boolean(
        bothAreObjects &&
            Object.keys(a).length === Object.keys(b).length &&
            Object.entries(a).every(([k, v]) => deepEqual(v, b[k as keyof T]))
    );
};

export function useGameState<T>(
    getValue: (gameState: GameState) => T, 
    events?: StateEventType[],
    fallback?: T
): T | undefined {
    const [state, setState] = useState<T | undefined>(() => {
        if (GAME_MANAGER.state.stateType === "game") {
            return getValue(GAME_MANAGER.state);
        } else {
            return fallback;
        }
    });

    usePacketListener((type?: StateEventType) => {
        if (GAME_MANAGER.state.stateType === "game" && (events ?? []).includes(type as StateEventType)) {
            const value = getValue(GAME_MANAGER.state);
            if (!deepEqual(value, state)) {
                setState(value);
            }
        }
    })

    return state;
}

export function useLobbyState<T>(
    getValue: (gameState: LobbyState) => T, 
    events?: StateEventType[],
    fallback?: T
): T | undefined {
    const [state, setState] = useState<T | undefined>(() => {
        if (GAME_MANAGER.state.stateType === "lobby") {
            return getValue(GAME_MANAGER.state);
        } else {
            return fallback;
        }
    });

    usePacketListener((type?: StateEventType) => {
        if (GAME_MANAGER.state.stateType === "lobby" && (events ?? []).includes(type as StateEventType)) {
            const value = getValue(GAME_MANAGER.state);
            if (!deepEqual(value, state)) {
                setState(value);
            }
        }
    });

    return state;
}

export function useLobbyOrGameState<T>(
    getValue: (gameState: LobbyState | GameState) => T, 
    events?: StateEventType[],
    fallback?: T
): T | undefined {
    const [state, setState] = useState<T | undefined>(() => {
        if (GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game") {
            return getValue(GAME_MANAGER.state);
        } else {
            return fallback;
        }
    });

    usePacketListener((type?: StateEventType) => {
        if (
            (GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game") 
            && (events ?? []).includes(type as StateEventType)
        ) {
            const value = getValue(GAME_MANAGER.state);
            if (!deepEqual(value, state)) {
                setState(value);
            }
        }
    });

    return state;
}

export function usePlayerState<T>(
    getValue: (playerState: PlayerGameState, gameState: GameState) => T,
    events?: StateEventType[],
    fallback?: T
): T | undefined {
    return useGameState(
        gameState => {
            if (gameState.clientState.type === "player") {
                return getValue(gameState.clientState, gameState);
            } else {
                return fallback;
            }
        }, 
        events, fallback
    );
}