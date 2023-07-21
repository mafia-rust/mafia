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

function route(url: Location) {
    const roomCode = new URLSearchParams(url.search).get("code");

    if (roomCode !== null) {
        GAME_MANAGER.gameState = createGameState();
        GAME_MANAGER.tryJoinGame(roomCode);
    } else if (url.pathname === '/wiki') {
        Anchor.setContent(<StandaloneWiki/>);
    } else {
        Anchor.setContent(<StartMenu/>)
    }
    // If we ever need more routing than this, use react router instead.
}

ROOT.render(
    <Anchor 
        content={LoadingScreen.create()} 
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