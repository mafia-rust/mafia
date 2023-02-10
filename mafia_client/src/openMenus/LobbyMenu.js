import React from "react";
import gameManager from "../game/gameManager";

export class LobbyMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            nameFieldValue: "",

            listener : {func : ()=>{
                this.setState({
                    nameValue : gameManager.gameState.myName,
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
 
    renderSettings(){return(<div>
        Settings
    </div>)}
    renderRolePicker(){return(<div>
        Role List
    </div>)}
    renderPlayers(){return(<div>
        Players
    </div>)}

    render(){return(<div>
        Name<br/>
        {(()=>{
            if(this.state.nameValue)
                return(<div>{this.state.nameValue}<br/></div>);
        })()}
        <input type="text" value={this.state.nameFieldValue}
            onChange={(e)=>{this.setState({nameFieldValue: e.target.value})}}
            onKeyUp={(e)=>{
                if(e.key === 'Enter')
                    gameManager.setName_button(this.state.nameFieldValue);
            }}
        /><br/>
        <button onClick={()=>{gameManager.setName_button(this.state.nameFieldValue)}}>Set Name</button><br/>
        <br/>
        {this.renderSettings()}
        {this.renderRolePicker()}
        {this.renderPlayers()}
        <br/>
        <button style={{width: "90%"}} onClick={()=>{gameManager.startGame_button()}}>Start</button><br/>
    </div>)}
}