import React, { ReactElement, useContext } from "react";
import { PhaseType, PhaseTimes, PHASES } from "../../game/gameState.d";
import translate from "../../game/lang";
import { isValidPhaseTime } from "../../game/gameManager";
import "./phaseTimeSelector.css";
import { GameModeContext } from "./GameModesEditor";



export default function PhaseTimesSelector(props: {
    disabled?: boolean,
    onChange: (phaseTimes: PhaseTimes) => void,
}): ReactElement {
    const {phaseTimes} = useContext(GameModeContext);

    const onChange = (phase: PhaseType, time: number) => {
        let newPhaseTimes = {...phaseTimes};
        newPhaseTimes[phase] = time;
        props.onChange(newPhaseTimes);
    }

    return <section className="phase-times-selector will-menu-colors selector-section">
        <h2>{translate("menu.lobby.timeSettings")}</h2>
        <div className="phase-times">
            {PHASES.map(phase => 
                <PhaseTimeSelector key={phase} disabled={props.disabled} phase={phase} time={phaseTimes[phase]} onChange={onChange}/>
            )}
        </div>
    </section>
}


function PhaseTimeSelector(props: {
    disabled?: boolean,
    phase: PhaseType,
    time: number,
    onChange: (phase: PhaseType, time: number) => void,
}): ReactElement {
    const phaseKey = "phase." + props.phase;
    
    return <div>
        <span>{translate(phaseKey)}</span>
        <input
            disabled={props.disabled ?? false}
            name={phaseKey}
            type="text"
            value={props.time}
            onChange={(e)=>{
                const value = Number(e.target.value);

                if (!isValidPhaseTime(value)) return
                
                props.onChange(props.phase, value);
                
            }}
            onKeyUp={(e)=>{
                if(e.key !== 'Enter') return;
                
                props.onChange(props.phase, props.time);
            }}
        />
    </div>
}