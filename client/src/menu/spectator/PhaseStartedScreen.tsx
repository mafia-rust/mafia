import React, { useEffect } from "react";
import { ReactElement } from "react";
import GAME_MANAGER from "../..";
import { StateEventType, StateListener } from "../../game/gameManager.d";
import translate from "../../game/lang";
import StyledText from "../../components/StyledText";
import { getTranslatedSubtitle } from "./SpectatorGameScreen";



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

    let subtitleText = getTranslatedSubtitle();

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