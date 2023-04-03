import React from "react";
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
            if(type==="PhaseTimes")
                this.setState(GAME_MANAGER.gameState.phaseTimes);
        }
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    phaseTimesButton() {
        //TODO Errors for some reason, this  is undefined?
        GAME_MANAGER.phaseTimesButton(this.state);
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
                this.setState({
                    [phase]: Number(e.target.value)
                } as Pick<PhaseTimes, keyof PhaseTimes>)
            }}
            onKeyUp={(e)=>{
                if(e.key === 'Enter')
                    this.phaseTimesButton();
            }}
        />
    </div>}
}


