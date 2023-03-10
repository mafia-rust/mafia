import React from "react";
import gameManager from "../../index.js";
import "./lobbyMenu.css";

export class LobbySettingsPane extends React.Component {
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
    }
    componentDidMount() {
    }
    componentWillUnmount() {
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
        {this.renderTimeSettings()}
        {this.renderRolePicker()}
    </div>)}

    renderTimeSettings(){return(<div>
        <button className="button lm-set-time-button" onClick={this.phaseTimesButton}>Set Time Settings</button>
        
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
        <div className="input-box-label">{name}</div>
        <input type="text" value={value}
            className="input-field"
            onChange={(e)=>{setter(e.target.value)}}
            onKeyUp={(e)=>{
                if(e.key === 'Enter')
                    this.phaseTimesButton();
            }}
        />
    </div>}

    renderRolePicker(){return(<div>
        <RolePicker/>
    </div>)}
}



class RolePicker extends React.Component {
    constructor(props) {
      super(props);
      this.state = {
        faction: '',
        alignment: '',
        role: ''
      };
  
      this.handleFactionChange = this.handleFactionChange.bind(this);
      this.handleAlignmentChange = this.handleAlignmentChange.bind(this);
      this.handleRoleChange = this.handleRoleChange.bind(this);
    }
  
    handleFactionChange(event) {
      this.setState({ faction: event.target.value });
      this.setState({ alignment: '' });
      this.setState({ role: '' });
    }
  
    handleAlignmentChange(event) {
      this.setState({ alignment: event.target.value });
      this.setState({ role: '' });
    }
  
    handleRoleChange(event) {
      this.setState({ role: event.target.value });
    }
  
    render() {
      const factions = ['Faction 1', 'Faction 2', 'Faction 3'];
      const alignments = ['Alignment 1', 'Alignment 2', 'Alignment 3'];
      const roles = ['Role 1', 'Role 2', 'Role 3'];
  
      return (
        <div>
          <h2>Role Picker</h2>
          <form>
            <label>
              Faction:
              <select value={this.state.faction} onChange={this.handleFactionChange}>
                <option value="">Select Faction</option>
                {factions.map(faction => <option key={faction} value={faction}>{faction}</option>)}
              </select>
            </label>
            {this.state.faction && !this.state.alignment &&
              <label>
                Alignment:
                <select value={this.state.alignment} onChange={this.handleAlignmentChange}>
                  <option value="">Select Alignment</option>
                  {alignments.map(alignment => <option key={alignment} value={alignment}>{alignment}</option>)}
                </select>
              </label>
            }
            {(this.state.faction && this.state.alignment) || (!this.state.faction && !this.state.alignment) &&
              <label>
                Role:
                <select value={this.state.role} onChange={this.handleRoleChange}>
                  <option value="">Select Role</option>
                  {roles.map(role => <option key={role} value={role}>{role}</option>)}
                </select>
              </label>
            }
          </form>
        </div>
      );
    }
}