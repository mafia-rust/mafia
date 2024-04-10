import React, { ReactElement, useContext } from "react";
import { PhaseType, PhaseTimes } from "../game/gameState.d";
import translate from "../game/lang";
import { isValidPhaseTime } from "../game/gameManager";
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

    return <section className="will-menu-colors selector-section">
        <h2>{translate("menu.lobby.timeSettings")}</h2>
        <PhaseTimeSelector disabled={props.disabled} phase={"briefing"} time={phaseTimes["briefing"]} onChange={onChange}/>
        <PhaseTimeSelector disabled={props.disabled} phase={"obituary"} time={phaseTimes["obituary"]} onChange={onChange}/>
        <PhaseTimeSelector disabled={props.disabled} phase={"discussion"} time={phaseTimes["discussion"]} onChange={onChange}/>
        <PhaseTimeSelector disabled={props.disabled} phase={"nomination"} time={phaseTimes["nomination"]} onChange={onChange}/>
        <PhaseTimeSelector disabled={props.disabled} phase={"testimony"} time={phaseTimes["testimony"]} onChange={onChange}/>
        <PhaseTimeSelector disabled={props.disabled} phase={"judgement"} time={phaseTimes["judgement"]} onChange={onChange}/>
        <PhaseTimeSelector disabled={props.disabled} phase={"finalWords"} time={phaseTimes["finalWords"]} onChange={onChange}/>
        <PhaseTimeSelector disabled={props.disabled} phase={"dusk"} time={phaseTimes["dusk"]} onChange={onChange}/>
        <PhaseTimeSelector disabled={props.disabled} phase={"night"} time={phaseTimes["night"]} onChange={onChange}/>
    </section>
}


function PhaseTimeSelector(props: {
    disabled?: boolean,
    phase: PhaseType,
    time: number,
    onChange: (phase: PhaseType, time: number) => void,
}): ReactElement {
    let phaseKey = "phase." + props.phase;
    
    return <div className="phase-time-selector">
        <label htmlFor={phaseKey}>{translate(phaseKey)}:</label>
        <input
            disabled={props.disabled??false}
            name={phaseKey}
            type="text"
            value={props.time??10}
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