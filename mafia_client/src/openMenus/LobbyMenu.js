import React from "react";
import gameManager from "../game/gameManager";

export class LobbyMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            showTimeSettings: false,

            
            nameField: "",

            morningTimeField : "5",
            discussionTimeField: "45", 
            votingTimeField: "30", 
            testimonyTimeField: "20", 
            judgementTimeField: "20", 
            eveningTimeField: "10", 
            nightTimeField: "37",


            nameSynced: "",

            morningTimeSynced : 5,
            discussionTimeSynced: 45, 
            votingTimeSynced: 30, 
            testimonyTimeSynced: 20, 
            judgementTimeSynced: 20, 
            eveningTimeSynced: 10, 
            nightTimeSynced: 37,

            listener : {func : ()=>{
                this.setState({
                    nameValueSynced : gameManager.gameState.myName,
                })
            }},
        };
                
    }
    componentDidMount() {
        gameManager.addStateListner(this.state.listener);
    }
    componentWillUnmount() {
        gameManager.removeStateListner(this.state.listener);
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
        Name<br/>
        {this.state.nameSynced}<br/>
        <input type="text" value={this.state.nameField}
            onChange={(e)=>{this.setState({nameField: e.target.value})}}
            onKeyUp={(e)=>{
                if(e.key === 'Enter')
                    gameManager.setName_button(this.state.nameField);
            }}
        /><br/>
        <button onClick={()=>{gameManager.setName_button(this.state.nameField)}}>Set Name</button><br/>
        <br/>
    </div>)}
    renderSettings(){return(<div>
        Settings<br/>
        {(()=>{
            if(this.state.showTimeSettings)
                return this.renderTimeSettings();
            else
                return (<button onClick={()=>{this.setState({showTimeSettings: true})}}>Time Settings<br/></button>)
        })()}
        
    </div>)}

    renderTimeSettings(){return(<div>
        <button onClick={this.phaseTimes_button}>Set Time Settings</button><br/>
        Morning:<input type="text" value={this.state.morningTimeField}
            onChange={(e)=>{this.setState({morningTimeField: e.target.value})}}
            onKeyUp={(e)=>{
                if(e.key === 'Enter')
                    this.phaseTimes_button();
            }}
        /><br/>
        Discussion:<input type="text" value={this.state.discussionTimeField}
            onChange={(e)=>{this.setState({discussionTimeField: e.target.value})}}
            onKeyUp={(e)=>{
                if(e.key === 'Enter')
                    this.phaseTimes_button();
            }}
        /><br/>
        Voting:<input type="text" value={this.state.votingTimeField}
            onChange={(e)=>{this.setState({votingTimeField: e.target.value})}}
            onKeyUp={(e)=>{
                if(e.key === 'Enter')
                    this.phaseTimes_button();  
            }}
        /><br/>
        Testimony:<input type="text" value={this.state.testimonyTimeField}
            onChange={(e)=>{this.setState({testimonyTimeField: e.target.value})}}
            onKeyUp={(e)=>{
                if(e.key === 'Enter')
                    this.phaseTimes_button();
            }}
        /><br/>
        Judgement:<input type="text" value={this.state.judgementTimeField}
            onChange={(e)=>{this.setState({judgementTimeField: e.target.value})}}
            onKeyUp={(e)=>{
                if(e.key === 'Enter')
                    this.phaseTimes_button();
            }}
        /><br/>
        Evening:<input type="text" value={this.state.eveningTimeField}
            onChange={(e)=>{this.setState({eveningTimeField: e.target.value})}}
            onKeyUp={(e)=>{
                if(e.key === 'Enter')
                    this.phaseTimes_button();}}
        /><br/>
        Night:<input type="text" value={this.state.nightTimeField}
            onChange={(e)=>{this.setState({nightTimeField: e.target.value})}}
            onKeyUp={(e)=>{
                if(e.key === 'Enter')
                    this.phaseTimes_button();}}
        /><br/>
        <button onClick={()=>{this.setState({showTimeSettings : false})}}>Close Time Settings</button><br/>
        <br/>
    </div>)}

    renderRolePicker(){return(<div>
        Role List
    </div>)}
    renderPlayers(){return(<div>
        Players
    </div>)}

    render(){return(<div>
        {this.renderName()}
        {this.renderSettings()}
        {this.renderRolePicker()}
        {this.renderPlayers()}
        <br/>
        <button style={{width: "90%"}} onClick={()=>{gameManager.startGame_button()}}>Start</button><br/>
    </div>)}
}