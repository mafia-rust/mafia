import React, { useEffect } from "react";
import { ReactElement } from "react";
import GAME_MANAGER from "../..";
import { StateEventType, StateListener } from "../../game/gameManager.d";
import translate from "../../game/lang";
import StyledText from "../../components/StyledText";



export default function PhaseStartedScreen(props: {}): ReactElement {

    const [phase, setPhase] = React.useState(
        GAME_MANAGER.state.stateType==="game" ? GAME_MANAGER.state.phaseState : {type:"briefing" as "briefing"}
    );
    const [dayNumber, setDayNumber] = React.useState(
        GAME_MANAGER.state.stateType==="game" ? GAME_MANAGER.state.dayNumber : 0
    );

    useEffect(() => {
        const listener: StateListener = (type?: StateEventType) => {
            if(GAME_MANAGER.state.stateType !== "game") return;

            switch (type) {
                case "phase":
                    if(GAME_MANAGER.state.phaseState !== null)
                        setPhase(GAME_MANAGER.state.phaseState);
                        setDayNumber(GAME_MANAGER.state.dayNumber);
                    break;
            }
        };
        GAME_MANAGER.addStateListener(listener);
        return () => GAME_MANAGER.removeStateListener(listener);
    }, [setPhase, setDayNumber]);

    let subtitleText = "";
    switch (phase.type) {
        case "briefing":
        case "night":
        case "discussion":
            subtitleText = translate("phase."+phase.type+".subtitle");
            break;
        case "nomination":
            if(GAME_MANAGER.state.stateType === "game"){
                let votesRequired = GAME_MANAGER.getVotesRequired();

                if(votesRequired !== null){
                    subtitleText += votesRequired === 1 ? translate("votesRequired.1") : translate("votesRequired", votesRequired);
                }

                subtitleText += " "+translate("trialsRemaining", phase.trialsLeft);
            }
            break;
        case "testimony":
        case "judgement":
        case "finalWords":
            if(GAME_MANAGER.state.stateType === "game" && phase.playerOnTrial !== null){
                subtitleText = translate("phase."+phase.type+".subtitle", GAME_MANAGER.getPlayerNames()[phase.playerOnTrial].toString());
            }
            break;
        default:
            break;
    }

    return (
        <div className="phase-started-screen">
            <div className="header">
                <h1><StyledText>{translate("phase."+phase.type)+" "+dayNumber}</StyledText></h1>
            </div>
            <div className="content">
                <p><StyledText>{subtitleText}</StyledText></p>
            </div>
        </div>
    );
}