import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import Anchor from './menu/Anchor';
import { GameManager, createGameManager } from './game/gameManager';
import { createGameState } from './game/gameState';
import StartMenu from './menu/main/StartMenu';


const ROOT = ReactDOM.createRoot(document.getElementById('root')!);
const GAME_MANAGER: GameManager = createGameManager();
const TIME_PERIOD = 1000;
export default GAME_MANAGER;

setInterval(() => {
    GAME_MANAGER.tick(TIME_PERIOD);
}, TIME_PERIOD);

// Route roomcode queries to the associated lobby
function onMount() {
    const roomCode = (new URL(window.location.href)).searchParams.get("code");
    
    if (roomCode !== null) {
        GAME_MANAGER.gameState = createGameState();
        GAME_MANAGER.tryJoinGame(roomCode);
    }

    window.history.replaceState({}, document.title, window.location.pathname);
}

ROOT.render(
    <Anchor 
        content={<StartMenu/>} 
        onMount={onMount}
    />
);
