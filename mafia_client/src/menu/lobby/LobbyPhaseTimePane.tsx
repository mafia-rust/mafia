import React from "react";
import { isValidPhaseTime } from "../../game/gameManager";
import { Phase, PhaseTimes } from "../../game/gameState.d";
import translate from "../../game/lang";
import GAME_MANAGER from "../../index";
import "./lobbyMenu.css";
import phaseTimesJson from "../../resources/phasetimes.json";

const PHASE_TIME_MODES: ReadonlyMap<string, PhaseTimes> = new Map(Object.entries(phaseTimesJson));
type PhaseTimeMode = string;

type PhaseTimePaneState = {
    mode: PhaseTimeMode,
    phaseTimes: PhaseTimes
}

export default class LobbyPhaseTimePane extends React.Component<{}, PhaseTimePaneState> {
    listener: (type: any) => void;
    constructor(props: {}) {
        super(props);

        let initialPhaseTimes = {...PHASE_TIME_MODES.get("Classic")!};

        this.state = {
            mode: this.determineModeFromPhaseTimes(initialPhaseTimes),
            phaseTimes: initialPhaseTimes
        };

        this.listener = (type)=>{
            if(type==="PhaseTime")
                this.setState({
                    mode: this.determineModeFromPhaseTimes(GAME_MANAGER.gameState.phaseTimes),
                    phaseTimes: GAME_MANAGER.gameState.phaseTimes
                });
        }
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    render(){return(<section>
        <header>
            <h2>Time settings:</h2>
            {this.renderTimeModeDropdown()}
        </header>
        <div className="input-column">
            {this.renderTimePicker(Phase.Morning)}
            {this.renderTimePicker(Phase.Discussion)}
            {this.renderTimePicker(Phase.Voting)}
            {this.renderTimePicker(Phase.Testimony)}
            {this.renderTimePicker(Phase.Judgement)}
            {this.renderTimePicker(Phase.Evening)}
            {this.renderTimePicker(Phase.Night)}
        </div>
    </section>)}

    renderTimeModeDropdown() {
        return <select 
            value={this.state.mode.toString()}
            onChange={(e) => {
                let mode = e.target.value as PhaseTimeMode;
                let phaseTimes = this.determinePhaseTimesFromMode(mode);
                this.setState({
                    mode: mode,
                    phaseTimes: phaseTimes
                });
                for (let [phase, time] of Object.entries(phaseTimes)) {
                    GAME_MANAGER.phaseTimeButton(phase as Phase, time);
                }
            }}
        >{
            // TODO lang
            this.state.mode == "Custom" ? <option key={"Custom"}>{"Custom"}</option> : null
        }{
            Object.keys(phaseTimesJson)
                .map((phase) => {return <option key={phase}>{phase}</option>})
        }</select>
    }

    renderTimePicker(phase: Phase) {
        let phaseKey = "phase." + phase;
        return <div className="lm-time-input-box">
            <label htmlFor={phaseKey}>{translate(phaseKey)}:</label>
            <input name={phaseKey} type="text" value={this.state.phaseTimes[phase]}
                onChange={(e)=>{ 
                    let value = Number(e.target.value);

                    if (isValidPhaseTime(value)) {
                        let newPhaseTimes = this.state.phaseTimes;
                        newPhaseTimes[phase] = value;

                        this.setState({
                            mode: this.determineModeFromPhaseTimes(newPhaseTimes),
                            phaseTimes: {...newPhaseTimes}
                        });
                    }
                    GAME_MANAGER.phaseTimeButton(phase, value);
                }}
                onKeyUp={(e)=>{
                    if(e.key === 'Enter')
                        GAME_MANAGER.phaseTimeButton(phase, this.state.phaseTimes[phase]);
                }}
            />
        </div>
    }
    
    determineModeFromPhaseTimes(phaseTimes: PhaseTimes): PhaseTimeMode {
        for (let [mode, times] of PHASE_TIME_MODES) {
            if (JSON.stringify(times) === JSON.stringify(phaseTimes)) {
                return mode as PhaseTimeMode;
            }
        }
        return "Custom" as PhaseTimeMode;
    }

    determinePhaseTimesFromMode(mode: PhaseTimeMode): PhaseTimes {
        if (mode === "Custom") {
            return this.state.phaseTimes;
        }
        return {...PHASE_TIME_MODES.get(mode.toString())} as PhaseTimes;
    }
}
