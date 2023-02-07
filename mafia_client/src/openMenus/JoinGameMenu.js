import React from "react";
import gameManager from "../game/gameManager";
import namesJson from "../names"

export class JoinGameMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            roomCodeValue: "",
        };
    }
    componentDidMount() {
        //generate random name
        this.setState({nameValue: 
            namesJson.defaultNames[
                Math.floor(
                    Math.random()*namesJson.defaultNames.length
                )
            ]
        });
    }
    componentWillUnmount() {
    }
    
    button(host){

        gameManager.Server.close();
        gameManager.Server.open();
        //wait for server to open

        // TODO fix this garbage thing to wait
        setTimeout(()=>{
            switch(host){
                case true:
                    this.hostGameButton();
                    break;
                case false:
                    this.joinGameButton();
                    break;
                default:
                    break;
            }
        },1000);

        
    }
    joinGameButton(){
        gameManager.join_button();
    }
    hostGameButton(){
        gameManager.host_button();
    }

    render(){return(<div>
        Room Code<br/>
        <input type="text" value={this.state.roomCodeValue} 
            onChange={(e)=>{this.setState({roomCodeValue: e.target.value})}}
        /><br/>
        <br/>
        <br/>
        <button style={{width: "90%"}} onClick={()=>{this.button(false)}}>Join Lobby</button><br/>
        <br/>
        <button style={{width: "90%"}} onClick={()=>{this.button(true)}}>New Lobby</button><br/>
    </div>)}
}