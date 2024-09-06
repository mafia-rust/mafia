import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import Anchor, { ANCHOR_CONTROLLER } from './menu/Anchor';
import { GameManager, createGameManager } from './game/gameManager';
import StartMenu from './menu/main/StartMenu';
import LoadingScreen from './menu/LoadingScreen';
import { deleteReconnectData, loadReconnectData } from './game/localStorage';
import AudioController from './menu/AudioController';

export type Theme = "player-list-menu-colors" | "will-menu-colors" | "role-specific-colors" | "graveyard-menu-colors" | "wiki-menu-colors"

const ROOT = ReactDOM.createRoot(document.querySelector("#root")!);
const GAME_MANAGER: GameManager = createGameManager();
const TIME_PERIOD = 1000;
export default GAME_MANAGER;

setInterval(() => {
    GAME_MANAGER.tick(TIME_PERIOD);
}, TIME_PERIOD);

async function route(url: Location) {
    AudioController.clearQueue();
    AudioController.pauseQueue();
    const roomCode = new URLSearchParams(url.search).get("code");
    let reconnectData = loadReconnectData();
    
    const HOUR_IN_SECONDS = 3_600_000;
    if (reconnectData && reconnectData.lastSaveTime < Date.now() - HOUR_IN_SECONDS) {
        reconnectData = null;
        deleteReconnectData();
    }

    if (roomCode !== null) {

        
        await GAME_MANAGER.setOutsideLobbyState();
        
        window.history.replaceState({}, document.title, window.location.pathname);

        let success: boolean;
        try {
            const code = parseInt(roomCode, 18)
            if (reconnectData) {
                success = await GAME_MANAGER.sendRejoinPacket(code, reconnectData.playerId);
                

                if(!success) {
                    reconnectData = null;
                    deleteReconnectData();
                    success = await GAME_MANAGER.sendJoinPacket(code);
                }
            }else{
                success = await GAME_MANAGER.sendJoinPacket(code);
            }
        } catch {
            success = false;
        }
        
        if (!success) {
            await GAME_MANAGER.setDisconnectedState();
            ANCHOR_CONTROLLER?.clearCoverCard();
            ANCHOR_CONTROLLER?.setContent(<StartMenu/>)
        }
    } else if (reconnectData) {
        await GAME_MANAGER.setOutsideLobbyState();

        const success = await GAME_MANAGER.sendRejoinPacket(reconnectData.roomCode, reconnectData.playerId);
        if (!success) {
            // Don't show an error message for an auto-rejoin. The user didn't prompt it - they will be confused.
            // Reconnect data is deleted in messageListener
            await GAME_MANAGER.setDisconnectedState();
            ANCHOR_CONTROLLER?.clearCoverCard();
            ANCHOR_CONTROLLER?.setContent(<StartMenu/>);
        }
    } else {
        ANCHOR_CONTROLLER?.setContent(<StartMenu/>)
    }
}

ROOT.render(
    <Anchor onMount={() => route(window.location)}>
        <LoadingScreen type="default"/>
    </Anchor>
);

export function find(text: string): RegExp {
    // Detect if iOS <= 16.3
    // https://bugs.webkit.org/show_bug.cgi?id=174931
    // https://stackoverflow.com/a/11129615
    if(
        /(iPhone|iPod|iPad)/i.test(navigator.userAgent) && 
        /OS ([2-9]_\d)|(1[0-5]_\d)|(16_[0-3])(_\d)? like Mac OS X/i.test(navigator.userAgent)
    ) { 
        // This won't work if a keyword starts with a symbol.
        return RegExp(`\\b${regEscape(text)}(?!\\w)`, "gi");
    } else {
        return RegExp(`(?<!\\w)${regEscape(text)}(?!\\w)`, "gi");
    }
}

export function regEscape(text: string) {
    return text.replace(/[-[\]{}()*+?.,\\^$|#\s]/g, '\\$&')
}

export function replaceMentions(rawText: string, playerNames?: string[]) {

    if (playerNames === undefined) {
        playerNames = GAME_MANAGER.getPlayerNames();
        if (playerNames === undefined) {
            return rawText;
        }
    }

    let text = rawText;
    playerNames.forEach((player, i) => {
        text = text.replace(find(`@${i + 1}`), player);
    });
    playerNames.forEach((player, i) => {
        text = text.replace(find(`@${player}`), player);
    });
    return text;
}

export function modulus(n: number, m: number) {
    return ((n % m) + m) % m;
}
