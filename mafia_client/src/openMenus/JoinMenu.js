import React from "react";
import gameManager from "../index.js";

export class JoinMenu extends React.Component {
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
    joinGameButton(){
        gameManager.roomCode = Number(this.state.roomCodeValue);
        gameManager.Server.close();
        gameManager.Server.open();
        // Wait for server to open
        
        setTimeout(gameManager.join_button);
    }
    hostGameButton(){
        gameManager.Server.close();
        gameManager.Server.open();
        // Wait for server to open
        
        setTimeout(gameManager.host_button);
    }
    render(){return(<div>
        Room Code<br/>
        <input type="text" value={this.state.roomCodeValue} 
            onChange={(e)=>{this.setState({roomCodeValue: e.target.value})}}
            onKeyUp={(e)=>{
                if(e.key === 'Enter')
                    gameManager.roomCode = Number(this.state.roomCodeValue);
            }}
        />
        <button style={{width: "90%"}} onClick={()=>{this.button(false)}}>Join Lobby</button><br/>
    </div>)}
}