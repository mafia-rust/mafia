import React from "react";
import { isValidPhaseTime } from "../../game/gameManager";
import { Phase, PhaseTimes } from "../../game/gameState.d";
import translate from "../../game/lang";
import GAME_MANAGER from "../../index";
import "./lobbyMenu.css";
import phaseTimesJson from "../../resources/phasetimes.json";

const PHASE_TIME_MODES = new Map<string, PhaseTimes>(Object.entries(phaseTimesJson));
type PhaseTimeMode = string;

type PhaseTimePaneState = {
    mode: PhaseTimeMode,
    phaseTimes: PhaseTimes
}

export default class LobbyPhaseTimePane extends React.Component<{}, PhaseTimePaneState> {
    listener: (type: any) => void;
    constructor(props: {}) {
        super(props);

        let initialPhaseTimes = {
            [Phase.Morning]: 5,
            [Phase.Discussion]: 45, 
            [Phase.Voting]: 30, 
            [Phase.Testimony]: 20, 
            [Phase.Judgement]: 20, 
            [Phase.Evening]: 10, 
            [Phase.Night]: 37,
        }

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

    render(){return(<div className="lm-settings-pane">
        <div className="lm-time-select-header">
            <h2 className="lm-time-select-header-text">Time settings:</h2>
            {this.renderTimeModeDropdown()}
        </div>
        <div className="lm-time-select-region">
            {this.renderTimePicker(Phase.Morning)}
            {this.renderTimePicker(Phase.Discussion)}
            {this.renderTimePicker(Phase.Voting)}
            {this.renderTimePicker(Phase.Testimony)}
            {this.renderTimePicker(Phase.Judgement)}
            {this.renderTimePicker(Phase.Evening)}
            {this.renderTimePicker(Phase.Night)}
        </div>
    </div>)}

    renderTimeModeDropdown() {
        return <select 
            className="dropdown lm-time-select-dropdown"
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
            this.state.mode == "Custom" ? <option key={"Custom"}>{"Custom"}</option> : null
        }{
            Object.keys(phaseTimesJson)
                .map((phase) => {return <option key={phase}>{phase}</option>})
        }</select>
    }

    renderTimePicker(phase: Phase) {return <div className="input-box">
        <div className="input-box-label">{translate("phase." + phase)}</div>
        <input type="text" value={this.state.phaseTimes[phase]}
            className="input-field"
            onChange={(e)=>{ 
                let value = Number(e.target.value);
                if (isValidPhaseTime(value)) {
                    let newPhaseTimes = this.state.phaseTimes;
                    newPhaseTimes[phase] = value;

                    this.setState({
                        mode: this.determineModeFromPhaseTimes(newPhaseTimes),
                        phaseTimes: newPhaseTimes
                    })
                }
                GAME_MANAGER.phaseTimeButton(phase, value);
            }}
            onKeyUp={(e)=>{
                if(e.key === 'Enter')
                    GAME_MANAGER.phaseTimeButton(phase, this.state.phaseTimes[phase]);
            }}
        />
    </div>}
    
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
        return PHASE_TIME_MODES.get(mode.toString()) as PhaseTimes;
    }
}
