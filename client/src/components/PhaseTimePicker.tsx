import React, { ReactElement } from "react";
import { Phase, PhaseTimes } from "../game/gameState.d";
import translate from "../game/lang";
import { isValidPhaseTime } from "../game/gameManager";



export default function PhaseTimePicker(props: {
    phaseTimes: PhaseTimes,
    onChange: (phaseTimes: PhaseTimes) => void,
}): ReactElement {

    const onChange = (phase: Phase, time: number) => {
        let newPhaseTimes = {...props.phaseTimes};
        newPhaseTimes[phase] = time;
        props.onChange(newPhaseTimes);
    }

    return <div>
        <TimePicker phase={"morning"} time={props.phaseTimes["morning"]} onChange={onChange}/>
        <TimePicker phase={"discussion"} time={props.phaseTimes["discussion"]} onChange={onChange}/>
        <TimePicker phase={"voting"} time={props.phaseTimes["voting"]} onChange={onChange}/>
        <TimePicker phase={"testimony"} time={props.phaseTimes["testimony"]} onChange={onChange}/>
        <TimePicker phase={"judgement"} time={props.phaseTimes["judgement"]} onChange={onChange}/>
        <TimePicker phase={"evening"} time={props.phaseTimes["evening"]} onChange={onChange}/>
        <TimePicker phase={"night"} time={props.phaseTimes["night"]} onChange={onChange}/>
    </div>
}


function TimePicker(props: {
    disabled?: boolean,
    phase: Phase,
    time: number,
    onChange: (phase: Phase, time: number) => void,
}): ReactElement {
    let phaseKey = "phase." + props.phase;
    
    return <div className="time-picker">
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