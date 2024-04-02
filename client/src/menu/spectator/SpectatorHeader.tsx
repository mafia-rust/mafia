import React, { ReactElement, useEffect } from "react";
import StyledText from "../../components/StyledText";
import translate from "../../game/lang";
import { PhaseState } from "../../game/gameState.d";
import GAME_MANAGER from "../..";
import { StateEventType } from "../../game/gameManager.d";

export default function SpectatorHeader(props: {
    phase: PhaseState,
    timeLeftMs: number,
    timeBarPercentage: number
}): ReactElement {

    const [dayNumber, setDayNumber] = React.useState(
        GAME_MANAGER.state.stateType==="game" ? GAME_MANAGER.state.dayNumber : 0
    );
    useEffect(() => {
        const listener = (type?: StateEventType) => {
            if(GAME_MANAGER.state.stateType !== "game") return;
            switch (type) {
                case "phase":
                    setDayNumber(GAME_MANAGER.state.dayNumber);
            }
        };
        GAME_MANAGER.addStateListener(listener);
        return () => GAME_MANAGER.removeStateListener(listener);
    }, [setDayNumber]);



    return (
        <div className="spectator-header">
            <div>
                <StyledText>
                    {translate("phase."+props.phase.type)+" "+ dayNumber} ⏳{Math.floor(props.timeLeftMs/1000).toString()}
                </StyledText>
                <div className="timer-box">
                    <div style={{
                        width:(100*props.timeBarPercentage).toString()+"%",
                        backgroundColor: "red",
                        height: "100%",
                        margin: '0 auto', // Center the timer horizontally
                    }}/>
                </div>
                
            </div>
        </div>
    );
}