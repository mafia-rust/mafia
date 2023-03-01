import React from "react";
import gameManager from "../game/gameManager";

export class JoinGameMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            roomCodeValue: "",
        };
    }
    componentDidMount() {
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
        gameManager.roomCode = Number(this.state.roomCodeValue);
        gameManager.join_button();
    }
    hostGameButton(){
        gameManager.host_button();
    }

    render(){return(<div>
        Room Code<br/>
        <input type="text" value={this.state.roomCodeValue} 
            onChange={(e)=>{this.setState({roomCodeValue: e.target.value})}}
            onKeyUp={(e)=>{
                if(e.key === 'Enter')
                    gameManager.roomCode = Number(this.state.roomCodeValue);
            }}
        /><br/>
        <br/>
        <br/>
        <button style={{width: "90%"}} onClick={()=>{this.button(false)}}>Join Lobby</button><br/>
        <br/>
        <button style={{width: "90%"}} onClick={()=>{this.button(true)}}>New Lobby</button><br/>
    </div>)}
}