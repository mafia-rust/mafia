import React from "react";
import gameManager from "../game/gameManager";

export class LobbyMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            nameValue: "",

            listener : {func : ()=>{
                this.setState({
                    nameValue : gameManager.myName,
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
        <input type="text" value={this.state.nameValue} 
            onChange={(e)=>{this.setState({nameValue: e.target.value})}}
        /><br/>

        <button onClick={()=>{gameManager.setName_button(this.state.nameValue)}}>Set Name</button>
        <br/>
        {this.renderSettings()}
        {this.renderRolePicker()}
        {this.renderPlayers()}
        <br/>
        <button style={{width: "90%"}}>Start</button><br/>
    </div>)}
}