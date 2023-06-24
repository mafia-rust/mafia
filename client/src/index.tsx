import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import Anchor from './menu/Anchor';
import { GameManager, createGameManager } from './game/gameManager';
import { createGameState } from './game/gameState';
import StartMenu from './menu/main/StartMenu';
import * as LoadingScreen from './menu/LoadingScreen'; 

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
