import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import Anchor from './menu/Anchor';
import { GameManager, createGameManager } from './game/gameManager';
import { createGameState } from './game/gameState';
import StartMenu from './menu/main/StartMenu';
import JoinMenu from './menu/main/JoinMenu';


const ROOT = ReactDOM.createRoot(document.getElementById('root')!);
const GAME_MANAGER: GameManager = createGameManager();
const TIME_PERIOD = 1000;
export default GAME_MANAGER;

setInterval(() => {
    GAME_MANAGER.tick(TIME_PERIOD);
}, TIME_PERIOD);


let anchorMenu = <StartMenu/>;
let roomCode = (new URL(window.location.href)).searchParams.get("code");
window.history.replaceState({}, document.title, window.location.pathname);
if(roomCode != null) {
    GAME_MANAGER.gameState = createGameState();
    anchorMenu = <JoinMenu roomCode={roomCode}/>;
}


ROOT.render(
    // <React.StrictMode>
        <Anchor content={anchorMenu} />
    // </React.StrictMode>
);
// // If you want to start measuring performance in your app, pass a function
// // to log results (for example: reportWebVitals(console.log))
// // or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
// reportWebVitals();
