import { ReactElement, useEffect } from "react";
import "./spectatorGameScreen.css";
import React from "react";
import PhaseStartedScreen from "./PhaseStartedScreen";
import GAME_MANAGER from "../..";
import { StateEventType, StateListener } from "../../game/gameManager.d";
import SpectatorHeader from "./SpectatorHeader";
import SpectatorBody from "./SpectatorBody";



/*
    briefing - Click the fast forward button to vote to start the game
    dusk (grave as subtitle if possible from final words)
        (Chat, rolelist, playerlist (if possible, sorted by verdict) )
    night - use your ability
        (Playerlist)
    obituary
        (Shows graves)
    discussion
        (Shows chat, playerlist, rolelist)
    nomination - 7 votes required, 3 trials remain
        (Shows chat, rolelist, and playerlist (with vote bars))
    testimony - Sammy is on trial
        (shows chat, rolelist, and playerlist (says if they voted))
    judgement - Sammy is on trial
        (shows chat, rolelist, and playerlist (says if they voted))
    final words - Sammy will be executed 
        (shows chat, rolelist, and playerlist (sorted by verdict))
*/

const DEFAULT_START_PHASE_SCREEN_TIME = 3;

export default function SpectatorGameScreen (props: {}): ReactElement {


    const [phase, setPhase] = React.useState(()=>{
        return GAME_MANAGER.state.stateType==="game" ? GAME_MANAGER.state.phaseState : {type:"briefing" as "briefing"}
    });
    const [timeLeftMs, setTimeLeftMs] = React.useState(()=>{
        return GAME_MANAGER.state.stateType==="game" ? GAME_MANAGER.state.timeLeftMs : 0
    });

    useEffect(() => {
        const listener: StateListener = (type?: StateEventType) => {

            if(GAME_MANAGER.state.stateType !== "game") return;

            switch (type) {
                case "phase":
                    setPhase(GAME_MANAGER.state.phaseState);
                    break;
                case "phaseTimeLeft":
                case "tick":
                    setTimeLeftMs(GAME_MANAGER.state.timeLeftMs);
                    break;
            }
        };
        GAME_MANAGER.addStateListener(listener);
        return () => GAME_MANAGER.removeStateListener(listener);
    }, [
        setPhase,
        setTimeLeftMs
    ]);

    let showStartedScreen = true;
    let maxTime = 0;
    if(GAME_MANAGER.state.stateType === "game"){
        maxTime = GAME_MANAGER.state.phaseTimes[phase.type];
        let timePassed = maxTime - Math.floor(timeLeftMs/1000);
        showStartedScreen = timePassed < DEFAULT_START_PHASE_SCREEN_TIME;
    }
    if(phase.type === "briefing") showStartedScreen = true;

    if(showStartedScreen){
        return (
            <div className="spectator-game-screen">
                <PhaseStartedScreen/>
            </div>
        );
    }else{
        return (
            <div className="spectator-game-screen">
                <SpectatorHeader phase={phase} timeLeftMs={timeLeftMs} timeBarPercentage={timeLeftMs/(maxTime*1000)}/>
                <SpectatorBody/>
            </div>
        );
    }
}