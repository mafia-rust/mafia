import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import Anchor from './menu/Anchor';
import { GameManager, create_gameManager } from './game/net/gameManager';

const ROOT = ReactDOM.createRoot(document.getElementById('root')!);

const GAME_MANAGER: GameManager = create_gameManager();
export default GAME_MANAGER;

const TIME_PERIOD = 1000;

setInterval(() => {
  GAME_MANAGER.tick(TIME_PERIOD);
}, TIME_PERIOD);

ROOT.render(
    <React.StrictMode>
        <Anchor/>
    </React.StrictMode>
);
// // If you want to start measuring performance in your app, pass a function
// // to log results (for example: reportWebVitals(console.log))
// // or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
// reportWebVitals();
