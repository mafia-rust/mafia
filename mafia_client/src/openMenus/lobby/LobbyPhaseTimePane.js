import React from "react";
import { translate } from "../../game/lang.js";
import gameManager from "../../index.js";
import "./lobbyMenu.css";

export class LobbyPhaseTimePane extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            morningTimeField: "5",
            discussionTimeField: "45", 
            votingTimeField: "30", 
            testimonyTimeField: "20", 
            judgementTimeField: "20", 
            eveningTimeField: "10", 
            nightTimeField: "37",
        };
        this.listener = (type)=>{
            if(type==="PhaseTimes")
                this.setState({

                    morningTimeField: gameManager.gameState.phaseTimes.morning,
                    discussionTimeField: gameManager.gameState.phaseTimes.discussion, 
                    votingTimeField: gameManager.gameState.phaseTimes.voting, 
                    testimonyTimeField: gameManager.gameState.phaseTimes.testimony, 
                    judgementTimeField: gameManager.gameState.phaseTimes.judgement, 
                    eveningTimeField: gameManager.gameState.phaseTimes.evening, 
                    nightTimeField: gameManager.gameState.phaseTimes.night,
                });
        }
    }
    componentDidMount() {
        gameManager.addStateListener(this.listener);
    }
    componentWillUnmount() {
        gameManager.removeStateListener(this.listener);
    }

    phaseTimesButton() {
        //TODO Errors for some reason, this  is undefined?
        gameManager.phaseTimesButton(
            Number(this.state.morningTimeField),
            Number(this.state.discussionTimeField),
            Number(this.state.votingTimeField),
            Number(this.state.testimonyTimeField),
            Number(this.state.judgementTimeField),
            Number(this.state.eveningTimeField),
            Number(this.state.nightTimeField),
        );
    }

    render(){return(<div className="lm-settings-pane">
        <button className="button lm-set-time-button" onClick={()=>{this.phaseTimesButton()}}>
            {translate("menu.lobby.button.set_time_settings")}
        </button>
        
        <div className="lm-time-select-region">
            {this.renderTimePicker("Morning", this.state.morningTimeField, 
                (val) => { this.setState({morningTimeField: val}) }
            )}
            {this.renderTimePicker("Discussion", this.state.discussionTimeField, 
                (val) => { this.setState({discussionTimeField: val}) }
            )}
            {this.renderTimePicker("Voting", this.state.votingTimeField, 
                (val) => { this.setState({votingTimeField: val}) }
            )}
            {this.renderTimePicker("Testimony", this.state.testimonyTimeField, 
                (val) => { this.setState({testimonyTimeField: val}) }
            )}
            {this.renderTimePicker("Judgement", this.state.judgementTimeField, 
                (val) => { this.setState({judgementTimeField: val}) }
            )}
            {this.renderTimePicker("Evening", this.state.eveningTimeField, 
                (val) => { this.setState({eveningTimeField: val}) }
            )}
            {this.renderTimePicker("Night", this.state.nightTimeField, 
                (val) => { this.setState({nightTimeField: val}) }
            )}
        </div>
    </div>)}

    renderTimePicker(name, value, setter) {return <div className="input-box">
        <div className="input-box-label">{translate("phase." + name)}</div>
        <input type="text" value={value}
            className="input-field"
            onChange={(e)=>{setter(e.target.value)}}
            onKeyUp={(e)=>{
                if(e.key === 'Enter')
                    this.phaseTimesButton();
            }}
        />
    </div>}
}


