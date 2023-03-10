import React from "react";
import gameManager from "../index.js";
import "../index.css";
import "./joinMenu.css";

export class JoinMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            roomCode: "",
            name: "",
        };
    }
    componentDidMount() {
    }
    componentWillUnmount() {
    }
    setRoomCode(code) {
        this.setState({roomCode: code})
    }
    setName(name) {
        this.setState({name: name})
    }
    joinGameButton(){
        gameManager.roomCode = Number(this.state.roomCode);
        gameManager.name = this.state.name;

        gameManager.Server.close();
        gameManager.Server.open();
        // Wait for server to open
        
        setTimeout(gameManager.join_button);
    }
    // hostGameButton(){
    //     gameManager.Server.close();
    //     gameManager.Server.open();
    //     // Wait for server to open
        
    //     setTimeout(gameManager.host_button);
    // }
    render(){return(<div style={{display: "flex", flexDirection: "column"}}>
        <div className="header jm-header">
            <h1 className="header-text jm-header-text">Join Game</h1>
        </div>
        <div className="jm-input-column">
            <div className="input-box">
                <h3 className="input-box-label">Room code</h3>
                <input className="input-field" type="text" value={this.state.roomCode} 
                    onChange={(e)=>{this.setRoomCode(e.target.value)}}
                    onKeyUp={(e)=>{
                        if(e.key === 'Enter')
                            gameManager.roomCode = Number(this.state.roomCode);
                    }}
                />
            </div>
            <div className="input-box">
                <h3 className="input-box-label">Name</h3>
                <input className="input-field" type="text" value={this.state.name} 
                    onChange={(e)=>{this.setName(e.target.value)}}
                    onKeyUp={(e)=>{
                        if(e.key === 'Enter')
                            gameManager.name = this.state.name;
                    }}
                />
            </div>
            <button className="button jm-button" onClick={()=>{this.joinGameButton()}}>Join Lobby</button>
        </div>
    </div>)}
}