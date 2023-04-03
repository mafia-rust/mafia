import React from "react";
import { isValidPhaseTime } from "../../game/gameManager";
import { Phase, PhaseTimes } from "../../game/gameState.d";
import translate from "../../game/lang";
import GAME_MANAGER from "../../index";
import "./lobbyMenu.css";

export default class LobbyPhaseTimePane extends React.Component<{}, PhaseTimes> {
    listener: (type: any) => void;
    constructor(props: {}) {
        super(props);

        this.state = {
            [Phase.Morning]: 5,
            [Phase.Discussion]: 45, 
            [Phase.Voting]: 30, 
            [Phase.Testimony]: 20, 
            [Phase.Judgement]: 20, 
            [Phase.Evening]: 10, 
            [Phase.Night]: 37,
        };

        this.listener = (type)=>{
            if(type==="PhaseTime")
                this.setState(GAME_MANAGER.gameState.phaseTimes);
        }
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    render(){return(<div className="lm-settings-pane">
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

    renderTimePicker(phase: Phase) {return <div className="input-box">
        <div className="input-box-label">{translate("phase." + phase)}</div>
        <input type="text" value={this.state[phase]}
            className="input-field"
            onChange={(e)=>{ 
                let value = Number(e.target.value);
                this.setState({
                    [phase]: isValidPhaseTime(value) ? value : this.state[phase]
                } as Pick<PhaseTimes, keyof PhaseTimes>)
                GAME_MANAGER.phaseTimeButton(phase, value);
            }}
            onKeyUp={(e)=>{
                if(e.key === 'Enter')
                    GAME_MANAGER.phaseTimeButton(phase, this.state[phase]);
            }}
        />
    </div>}
}


