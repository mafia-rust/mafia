import React, { useEffect } from "react";
import { ReactElement } from "react";
import GAME_MANAGER from "../..";
import { StateEventType, StateListener } from "../../game/gameManager.d";
import translate from "../../game/lang";



export default function PhaseStartedScreen(props: {}): ReactElement {

    const [phase, setPhase] = React.useState(
        GAME_MANAGER.state.stateType==="game" ? GAME_MANAGER.state.phase : "briefing"
    );
    const [dayNumber, setDayNumber] = React.useState(
        GAME_MANAGER.state.stateType==="game" ? GAME_MANAGER.state.dayNumber : 0
    );

    useEffect(() => {
        const listener: StateListener = (type?: StateEventType) => {
            if(GAME_MANAGER.state.stateType !== "game") return;

            switch (type) {
                case "phase":
                    if(GAME_MANAGER.state.phase !== null)
                        setPhase(GAME_MANAGER.state.phase);
                        setDayNumber(GAME_MANAGER.state.dayNumber);
                    break;
            }
        };
        GAME_MANAGER.addStateListener(listener);
        return () => GAME_MANAGER.removeStateListener(listener);
    }, [setPhase, setDayNumber]);

    let subtitleText = "";
    switch (phase) {
        case "briefing":
        case "night":
        case "discussion":
            subtitleText = translate("phase."+phase+".subtitle");
            break;
        case "nomination":
            if(GAME_MANAGER.state.stateType === "game"){
                let votesRequired = GAME_MANAGER.getVotesRequired();
                
                subtitleText += votesRequired === 1 ? translate("votesRequired.1") : translate("votesRequired", votesRequired);
                subtitleText = translate("votesRequired", 7) + translate("trialsRemaining", 3);
            }
            break;
        case "testimony":
        case "judgement":
        case "finalWords":
            if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.playerOnTrial !== null)
                subtitleText = translate(
                    "phase."+phase+"subtitle", 
                    GAME_MANAGER.state.players[GAME_MANAGER.state.playerOnTrial].name
                );
            break;
        default:
            break;
    }

    return (
        <div className="phase-started-screen">
            <div className="header">
                <h1>{translate("phase."+phase)+" "+dayNumber}</h1>
            </div>
            <div className="content">
                <p>{props.subtitleText}</p>
            </div>
        </div>
    );
}