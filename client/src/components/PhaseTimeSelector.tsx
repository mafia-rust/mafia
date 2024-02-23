import React, { ReactElement } from "react";
import { Phase, PhaseTimes } from "../game/gameState.d";
import translate from "../game/lang";
import { isValidPhaseTime } from "../game/gameManager";
import "./phaseTimeSelector.css";



export default function PhaseTimesSelector(props: {
    disabled?: boolean,
    phaseTimes: PhaseTimes,
    onChange: (phaseTimes: PhaseTimes) => void,
}): ReactElement {

    const onChange = (phase: Phase, time: number) => {
        let newPhaseTimes = {...props.phaseTimes};
        newPhaseTimes[phase] = time;
        props.onChange(newPhaseTimes);
    }

    return <section className="will-menu-colors selector-section">
        <h2>{translate("menu.lobby.timeSettings")}</h2>
        <PhaseTimeSelector disabled={props.disabled} phase={"morning"} time={props.phaseTimes["morning"]} onChange={onChange}/>
        <PhaseTimeSelector disabled={props.disabled} phase={"discussion"} time={props.phaseTimes["discussion"]} onChange={onChange}/>
        <PhaseTimeSelector disabled={props.disabled} phase={"voting"} time={props.phaseTimes["voting"]} onChange={onChange}/>
        <PhaseTimeSelector disabled={props.disabled} phase={"testimony"} time={props.phaseTimes["testimony"]} onChange={onChange}/>
        <PhaseTimeSelector disabled={props.disabled} phase={"judgement"} time={props.phaseTimes["judgement"]} onChange={onChange}/>
        <PhaseTimeSelector disabled={props.disabled} phase={"evening"} time={props.phaseTimes["evening"]} onChange={onChange}/>
        <PhaseTimeSelector disabled={props.disabled} phase={"night"} time={props.phaseTimes["night"]} onChange={onChange}/>
    </section>
}


function PhaseTimeSelector(props: {
    disabled?: boolean,
    phase: Phase,
    time: number,
    onChange: (phase: Phase, time: number) => void,
}): ReactElement {
    let phaseKey = "phase." + props.phase;
    
    return <div className="phase-time-selector">
        <label htmlFor={phaseKey}>{translate(phaseKey)}:</label>
        <input
            disabled={props.disabled??false}
            name={phaseKey}
            type="text"
            value={props.time}
            onChange={(e)=>{
                let value = Number(e.target.value);

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