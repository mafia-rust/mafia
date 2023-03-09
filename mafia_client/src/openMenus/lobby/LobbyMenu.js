import React from "react";
import gameManager from "../../index.js";

export class LobbyMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {            
            nameField: "",

            morningTimeField : "5",
            discussionTimeField: "45", 
            votingTimeField: "30", 
            testimonyTimeField: "20", 
            judgementTimeField: "20", 
            eveningTimeField: "10", 
            nightTimeField: "37",

            gameState: gameManager.gameState,
            roomCode: gameManager.roomCode,

            
        };
        this.listener = ()=>{
            this.setState({
                roomCode : gameManager.roomCode,
                gameState : gameManager.gameState,
            })
        };
    }
    componentDidMount() {
        gameManager.addStateListener(this.listener);
    }
    componentWillUnmount() {
        gameManager.removeStateListener(this.listener);
    }

    phaseTimes_button() {
        //TODO Errors for some reason, this  is undefined?
        gameManager.phaseTimes_button(
            Number(this.state.morningTimeField),
            Number(this.state.discussionTimeField),
            Number(this.state.votingTimeField),
            Number(this.state.testimonyTimeField),
            Number(this.state.judgementTimeField),
            Number(this.state.eveningTimeField),
            Number(this.state.nightTimeField),
        );
    }
 
    renderName(){return(<div>
        {this.state.gameState.myName}<br/>
        <input type="text" value={this.state.nameField}
            onChange={(e)=>{this.setState({nameField: e.target.value})}}
            onKeyUp={(e)=>{
                if(e.key === 'Enter')
                    gameManager.setName_button(this.state.nameField);
            }}
        /><br/>
        <button onClick={()=>{gameManager.setName_button(this.state.nameField)}}>Set Name</button><br/>
    </div>)}
    renderSettings(){return(<div>
        {this.renderTimeSettings()}
        {this.renderRolePicker()}
    </div>)}

    renderTimeSettings(){return(<div>
        <button onClick={()=>{this.phaseTimes_button()}}>Set Time Settings</button><br/>
        
        <div style={{display:"grid",gridAutoColumns:"1fr", gridAutoRows:"1fr"}}>
            
            <div style={{gridColumn:"1"}}></div>
            <div style={{gridColumn:"4"}}></div>

            <div style={{gridColumn:"2", gridRow:"1"}}>Morning<br/>{this.state.gameState.phaseTimes.morning}</div>
            <input type="text" value={this.state.morningTimeField}
                style={{gridColumn:"3", gridRow:"1"}}
                onChange={(e)=>{this.setState({morningTimeField: e.target.value})}}
                onKeyUp={(e)=>{
                    if(e.key === 'Enter')
                        this.phaseTimes_button();
                }}
            />
            <div style={{gridColumn:"2", gridRow:"2"}}>Discussion<br/>{this.state.gameState.phaseTimes.discussion}</div>
            <input type="text" value={this.state.discussionTimeField}
                style={{gridColumn:"3", gridRow:"2"}}
                onChange={(e)=>{this.setState({discussionTimeField: e.target.value})}}
                onKeyUp={(e)=>{
                    if(e.key === 'Enter')
                        this.phaseTimes_button();
                }}
            />
            <div style={{gridColumn:"2", gridRow:"3"}}>Voting<br/>{this.state.gameState.phaseTimes.voting}</div>
            <input type="text" value={this.state.votingTimeField}
                style={{gridColumn:"3", gridRow:"3"}}
                onChange={(e)=>{this.setState({votingTimeField: e.target.value})}}
                onKeyUp={(e)=>{
                    if(e.key === 'Enter')
                        this.phaseTimes_button();  
                }}
            />
            <div style={{gridColumn:"2", gridRow:"4"}}>Testimony<br/>{this.state.gameState.phaseTimes.testimony}</div>
            <input type="text" value={this.state.testimonyTimeField}
                style={{gridColumn:"3", gridRow:"4"}}
                onChange={(e)=>{this.setState({testimonyTimeField: e.target.value})}}
                onKeyUp={(e)=>{
                    if(e.key === 'Enter')
                        this.phaseTimes_button();
                }}
            />
            <div style={{gridColumn:"2", gridRow:"5"}}>Judgement<br/>{this.state.gameState.phaseTimes.judgement}</div>
            <input type="text" value={this.state.judgementTimeField}
                style={{gridColumn:"3", gridRow:"5"}}
                onChange={(e)=>{this.setState({judgementTimeField: e.target.value})}}
                onKeyUp={(e)=>{
                    if(e.key === 'Enter')
                        this.phaseTimes_button();
                }}
            />
            <div style={{gridColumn:"2", gridRow:"6"}}>Evening<br/>{this.state.gameState.phaseTimes.evening}</div>
            <input type="text" value={this.state.eveningTimeField}
                style={{gridColumn:"3", gridRow:"6"}}
                onChange={(e)=>{this.setState({eveningTimeField: e.target.value})}}
                onKeyUp={(e)=>{
                    if(e.key === 'Enter')
                        this.phaseTimes_button();}}
            />
            <div style={{gridColumn:"2", gridRow:"7"}}>Night<br/>{this.state.gameState.phaseTimes.night}</div>
            <input type="text" value={this.state.nightTimeField}
                style={{gridColumn:"3", gridRow:"7"}}
                onChange={(e)=>{this.setState({nightTimeField: e.target.value})}}
                onKeyUp={(e)=>{
                    if(e.key === 'Enter')
                        this.phaseTimes_button();}}
            />
        </div>
    </div>)}

    renderRolePicker(){return(<div>
        <RolePicker/>
    </div>)}
    renderPlayers(){return(<div>
        {this.state.gameState.players.map((player, i)=>{
            return(<div key={i}>{player.name}<br/></div>)
        })}
    </div>)}

    render(){return(<div style={{
        display: "grid",
        gridAutoColumns: "1fr"
    }}>
        <div style={{
            gridColumn: 1
        }}>
            Room Code: "{this.state.roomCode}"<br/>
            <button style={{width: "90%"}} onClick={()=>{gameManager.startGame_button()}}>Start</button><br/>
            <br/>
            {this.renderName()}
            <br/>
            {this.renderPlayers()}
            <br/>
        </div>
        <div style={{
            gridColumn: 2
        }}>
        <br/>
        {this.renderSettings()}
        <br/>
        </div>

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