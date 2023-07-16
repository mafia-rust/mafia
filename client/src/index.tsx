import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import Anchor from './menu/Anchor';
import { GameManager, createGameManager } from './game/gameManager';
import { createGameState } from './game/gameState';
import StartMenu from './menu/main/StartMenu';
import * as LoadingScreen from './menu/LoadingScreen';
import StandaloneWiki from './menu/main/StandaloneWiki';

const ROOT = ReactDOM.createRoot(document.querySelector("#root")!);
const GAME_MANAGER: GameManager = createGameManager();
const TIME_PERIOD = 1000;
export default GAME_MANAGER;

GAME_MANAGER.addStateListener((type) => {
    switch (type) {
        case "acceptJoin":
        case "acceptHost":
            window.history.pushState({}, document.title, `?code=${GAME_MANAGER.roomCode}`);
    }
})

setInterval(() => {
    GAME_MANAGER.tick(TIME_PERIOD);
}, TIME_PERIOD);

// Route roomcode queries to the associated lobby
function onMount() {
    const roomCode = new URLSearchParams(window.location.search).get("code");

    if (roomCode !== null) {
        GAME_MANAGER.gameState = createGameState();
        GAME_MANAGER.tryJoinGame(roomCode);
    } else if (window.location.pathname === '/wiki') {
        // If we ever need more routing than this, use react router instead.
        Anchor.setContent(<StandaloneWiki/>);
    } else {
        Anchor.setContent(<StartMenu/>)
    }
}

ROOT.render(
    <Anchor 
        content={LoadingScreen.create()} 
        onMount={onMount}
    />
);

export function find(text: string): RegExp {
    // Detect if iOS <= 16.3
    // This code doesn't work for iOS 1. Too bad!
    // https://stackoverflow.com/a/11129615
    if(
        /(iPhone|iPod|iPad)/i.test(navigator.userAgent) && 
        /OS ([2-9]_\d)|(1[0-5]_\d)|(16_[0-3])(_\d)? like Mac OS X/i.test(navigator.userAgent)
    ) { 
        // Close enough. This won't work if a keyword starts with a symbol.
        return RegExp(`\\b${regEscape(text)}(?!\\w)`, "gi");
    } else {
        return RegExp(`(?<!\\w)${regEscape(text)}(?!\\w)`, "gi");
    }
}

export function regEscape(text: string) {
    return text.replace(/[-[\]{}()*+?.,\\^$|#\s]/g, '\\$&')
}