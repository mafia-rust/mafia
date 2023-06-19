import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import Anchor from './menu/Anchor';
import { GameManager, createGameManager } from './game/gameManager';
import { createGameState } from './game/gameState';
import StartMenu from './menu/main/StartMenu';
import * as LoadingScreen from './menu/LoadingScreen'; 

const QUERY_PARAMS = new URLSearchParams(window.location.search);
// Clear query parameters from visible URL
window.history.replaceState({}, document.title, window.location.pathname);

const ROOT = ReactDOM.createRoot(document.getElementById('root')!);
const GAME_MANAGER: GameManager = createGameManager();
const TIME_PERIOD = 1000;
export default GAME_MANAGER;

function addRoomCodeToURL() {
    window.history.pushState({}, document.title, `?code=${GAME_MANAGER.roomCode}`);
}

GAME_MANAGER.addStateListener("acceptJoin", addRoomCodeToURL);
GAME_MANAGER.addStateListener("acceptHost", addRoomCodeToURL);

setInterval(() => {
    GAME_MANAGER.tick(TIME_PERIOD);
}, TIME_PERIOD);

// Route roomcode queries to the associated lobby
function onMount() {
    const roomCode = QUERY_PARAMS.get("code");

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
