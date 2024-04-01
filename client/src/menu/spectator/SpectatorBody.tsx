import React, { useEffect } from "react";
import { ReactElement } from "react";
import { StateEventType, StateListener } from "../../game/gameManager.d";
import GAME_MANAGER from "../..";
import ObituaryScreen from "./ObituaryScreen";

export default function SpectatorBody(): ReactElement {

    const [phase, setPhase] = React.useState(
        GAME_MANAGER.state.stateType==="game" ? GAME_MANAGER.state.phaseState : {type:"briefing" as "briefing"}
    );
    useEffect(() => {
        const listener: StateListener = (type?: StateEventType) => {
            if(GAME_MANAGER.state.stateType !== "game") return;

            switch (type) {
                case "phase":
                    setPhase(GAME_MANAGER.state.phaseState);
                    break;
            }
        };
        GAME_MANAGER.addStateListener(listener);
        return () => GAME_MANAGER.removeStateListener(listener);
    }, [setPhase]);

    switch(phase.type) {
        case "briefing":
        case "night":
        case "discussion":
        case "nomination":
        case "testimony":
        case "judgement":
        case "finalWords":
        case "dusk":
            return (
                <div className="spectator-body">
                    Spectator Body
                </div>
            );
            
        case "obituary":
            return (
                <ObituaryScreen/>
            );
    }    
}