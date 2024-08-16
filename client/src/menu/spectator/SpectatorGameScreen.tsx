import React, { ReactElement } from "react";
import "./spectatorGameScreen.css";
import PhaseStartedScreen from "./PhaseStartedScreen";
import { useGameState } from "../../components/useHooks";
import "../game/gameScreen.css"
import ChatMenu from "../game/gameScreenContent/ChatMenu";
import PlayerListMenu from "../game/gameScreenContent/PlayerListMenu";
import GraveyardMenu from "../game/gameScreenContent/GraveyardMenu";
import HeaderMenu from "../game/HeaderMenu";


const DEFAULT_START_PHASE_SCREEN_TIME = 3;

export default function SpectatorGameScreen (): ReactElement {
    const showStartedScreen = useGameState(
        gameState => {
            if (
                gameState.phaseState.type === "briefing"
                || gameState.phaseState.type === "obituary"
            ) return true;

            const maxTime = gameState.phaseTimes[gameState.phaseState.type];
            const timePassed = Math.floor(maxTime - gameState.timeLeftMs/1000);
            return timePassed < DEFAULT_START_PHASE_SCREEN_TIME;
        },
        ["phase", "phaseTimeLeft", "tick"],
        true
    )!


    return (
        <div className="game-screen spectator-game-screen">
            <div className="header">
                <HeaderMenu chatMenuNotification={false}/>
            </div>
            {showStartedScreen 
                ? <PhaseStartedScreen/>
                : <div className="content">
                    <ChatMenu/>
                    <PlayerListMenu/>
                    <GraveyardMenu/>
                </div>}
        </div>
    );
    
}