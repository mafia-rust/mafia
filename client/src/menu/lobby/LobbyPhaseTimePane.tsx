import React from "react";
import { isValidPhaseTime } from "../../game/gameManager";
import { Phase, PhaseTimes } from "../../game/gameState.d";
import translate from "../../game/lang";
import GAME_MANAGER from "../../index";
import "./lobbyMenu.css";
import phaseTimesJson from "../../resources/phaseTimes.json";
import { StateListener } from "../../game/gameManager.d";

const PHASE_TIME_MODES: ReadonlyMap<string, PhaseTimes> = new Map(Object.entries(phaseTimesJson));
type PhaseTimeMode = string;

type PhaseTimePaneState = {
    mode: PhaseTimeMode,
    advancedEditing: boolean,
    phaseTimes: PhaseTimes,
    host: boolean
}

export default class LobbyPhaseTimePane extends React.Component<{}, PhaseTimePaneState> {
    listener: StateListener;
    constructor(props: {}) {
        super(props);

        let initialPhaseTimes = GAME_MANAGER.gameState.phaseTimes ?? {...PHASE_TIME_MODES.get("Classic")!};

        this.state = {
            mode: this.determineModeFromPhaseTimes(initialPhaseTimes),
            advancedEditing: false,
            phaseTimes: initialPhaseTimes,
            host: GAME_MANAGER.gameState.host
        };

        this.listener = (type)=>{
            if(type==="phaseTime" || type==="phaseTimes")
                this.setState({
                    mode: this.determineModeFromPhaseTimes(GAME_MANAGER.gameState.phaseTimes),
                    phaseTimes: GAME_MANAGER.gameState.phaseTimes
                });
            else if (type === "youAreHost") {
                this.setState({ host: GAME_MANAGER.gameState.host });
            }
        }
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    renderTimeModeDropdown() {
        return <select 
            disabled={!this.state.host}
            value={this.state.mode.toString()}
            onChange={(e) => {
                let mode = e.target.value as PhaseTimeMode;
                let phaseTimes = this.determinePhaseTimesFromMode(mode);
                this.setState({
                    mode: mode,
                    phaseTimes: phaseTimes
                });
                GAME_MANAGER.sendSetPhaseTimesPacket(phaseTimes);
            }}
        >{
            // TODO lang
            this.state.mode === "Custom" ? <option key={"Custom"}>{"Custom"}</option> : null
        }{
            Object.keys(phaseTimesJson)
                .map((phase) => {return <option key={phase}>{phase}</option>})
        }</select>
    }

    renderEditButton() {
        return <button className="lm-edit-button" 
            onClick={() => {
                this.setState({
                    advancedEditing: !this.state.advancedEditing
                });
            }}
        >{this.state.advancedEditing ? "Hide" : "Advanced"}</button>
    }

    renderInputColumn() {
        return <div>
            {this.renderTimePicker("morning")}
            {this.renderTimePicker("discussion")}
            {this.renderTimePicker("voting")}
            {this.renderTimePicker("testimony")}
            {this.renderTimePicker("judgement")}
            {this.renderTimePicker("evening")}
            {this.renderTimePicker("night")}
        </div>
    }

    renderTimePicker(phase: Phase) {
        let phaseKey = "phase." + phase;
        return <div className="time-picker">
            <label htmlFor={phaseKey}>{translate(phaseKey)}:</label>
            <input 
                disabled={!this.state.host}
                name={phaseKey} 
                type="text" 
                value={this.state.phaseTimes[phase]}
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
                    GAME_MANAGER.sendSetPhaseTimePacket(phase, value);
                }}
                onKeyUp={(e)=>{
                    if(e.key === 'Enter')
                        GAME_MANAGER.sendSetPhaseTimePacket(phase, this.state.phaseTimes[phase]);
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

    render() {return(<section className="time">
        <h2>{translate("menu.lobby.timeSettings")}</h2>
        <div>
            {this.renderTimeModeDropdown()}
            {this.renderEditButton()}
        </div>
        {this.state.advancedEditing ? this.renderInputColumn() : null}
    </section>)}
}
