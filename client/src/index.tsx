import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import Anchor from './menu/Anchor';
import { GameManager, createGameManager } from './game/gameManager';
import StartMenu from './menu/main/StartMenu';
import { Player } from './game/gameState.d';
import LoadingScreen from './menu/LoadingScreen';

const ROOT = ReactDOM.createRoot(document.querySelector("#root")!);
const GAME_MANAGER: GameManager = createGameManager();
const TIME_PERIOD = 1000;
export default GAME_MANAGER;

setInterval(() => {
    GAME_MANAGER.tick(TIME_PERIOD);
}, TIME_PERIOD);

async function route(url: Location) {
    Anchor.stopAudio();
    const roomCode = new URLSearchParams(url.search).get("code");
    let reconnectData = GAME_MANAGER.loadReconnectData();
    
    const HOUR_IN_SECONDS = 3_600_000;
    if (reconnectData && reconnectData.lastSaveTime < Date.now() - HOUR_IN_SECONDS) {
        reconnectData = null;
        GAME_MANAGER.deleteReconnectData();
    }

    if (roomCode !== null) {
        await GAME_MANAGER.setOutsideLobbyState();
        
        window.history.replaceState({}, document.title, window.location.pathname);

        const success = await GAME_MANAGER.sendJoinPacket(roomCode);
        if (!success) {
            Anchor.setContent(<StartMenu/>)
        }
    } else if (reconnectData) {        
        await GAME_MANAGER.setOutsideLobbyState();

        const success = await GAME_MANAGER.sendRejoinPacket(reconnectData.roomCode, reconnectData.playerId);
        if (!success) {
            // Don't show an error message for an auto-rejoin. The user didn't prompt it - they will be confused.
            // Reconnect data is deleted in messageListener
            Anchor.clearError();
            Anchor.setContent(<StartMenu/>);
        }
    } else {
        Anchor.setContent(<StartMenu/>)
    }
}

ROOT.render(
    <Anchor 
        content={<LoadingScreen type="default"/>} 
        onMount={() => route(window.location)}
    />
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

export function replaceMentions(rawText: string, players: Player[]) {
    let text = rawText;
    players.forEach(player => {
        text = text.replace(find(`@${player.index + 1}`), player.toString());
    });
    players.forEach(player => {
        text = text.replace(find(`@${player.name}`), player.toString());
    });
    return text;
}

export function modulus(n: number, m: number) {
    return ((n % m) + m) % m;
}

window.addEventListener("blur", () => {
    Anchor.playAudioFile("/audio/longSpeech.mp4",true);
    Anchor.setLeftGame(true);
    alert("lame")
});