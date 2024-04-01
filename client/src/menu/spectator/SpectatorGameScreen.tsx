import { ReactElement, useEffect } from "react";
import "./spectatorGameScreen.css";
import React from "react";
import PhaseStartedScreen from "./PhaseStartedScreen";
import GAME_MANAGER from "../..";
import { StateEventType, StateListener } from "../../game/gameManager.d";



/*
    briefing - Click the fast forward button to vote to start the game
    dusk 
        (Chat, rolelist, playerlist (if possible, sorted by verdict))
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
export default function SpectatorGameScreen (props: {}): ReactElement {


    const [phase, setPhase] = React.useState(
        GAME_MANAGER.state.stateType==="game" ? GAME_MANAGER.state.phaseState.type : "briefing"
    );
    const [timeLeftMs, setTimeLeftMs] = React.useState(
        GAME_MANAGER.state.stateType==="game" ? GAME_MANAGER.state.timeLeftMs : 0
    );

    useEffect(() => {
        const listener: StateListener = (type?: StateEventType) => {

            if(GAME_MANAGER.state.stateType !== "game") return;

            switch (type) {
                case "phase":
                    if(GAME_MANAGER.state.phaseState !== null)
                        setPhase(GAME_MANAGER.state.phaseState.type);
                    break;
                case "phaseTimeLeft":
                    if(GAME_MANAGER.state.timeLeftMs !== null)
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

    


    return (
        <div className="spectator-game-screen">
            <PhaseStartedScreen/>
        </div>
    );
}