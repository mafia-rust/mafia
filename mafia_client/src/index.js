import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import { Main } from './Main';
import { create_gameManager } from './game/gameManager';

const root = ReactDOM.createRoot(document.getElementById('root'));

let gameManager = create_gameManager();
let time_period = 1000;
setInterval(()=>{
  gameManager.tick(time_period);
}, time_period);
export default gameManager;

root.render(
  <React.StrictMode>
    <Main />
  </React.StrictMode>
);
// // If you want to start measuring performance in your app, pass a function
// // to log results (for example: reportWebVitals(console.log))
// // or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
// reportWebVitals();
