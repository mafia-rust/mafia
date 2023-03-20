import React from "react";
import gameManager from "../index.js";
import { create_gameState } from "../game/gameState";
import {Main} from "../Main";
import "../index.css"
import "./startMenu.css"
import { LoadingMenu } from "./LoadingMenu.js";
import { JoinMenu } from "./JoinMenu.js";

export class StartMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            // User
        };
    }
    componentDidMount() {
    }
    componentWillUnmount() {
    }
    joinGameButton(){
        gameManager.gameState = create_gameState();
        Main.instance.setContent(<JoinMenu/>);
    }
    hostGameButton(){
        gameManager.gameState = create_gameState();
        
        Main.instance.setContent(<LoadingMenu value="Starting server..."/>);

        gameManager.Server.close();
        gameManager.Server.open();

        // Wait for server to open
        setTimeout(gameManager.host_button, 5000);  //TODO
        // Lobby menu opens when AcceptHost packet is recieved
    }
    render(){return(<div>
        <div className="header sm-header">
            <h1 className="header-text">Mafia</h1>
            <button className="button sm-login-button">Login</button><br/>
        </div>

        <div className="sm-button-area">
            <button className="button sm-join-host-button" onClick={()=>{this.joinGameButton()}}>
                {Main.instance?.isLoggedIn() ? "Join" : "Join as guest"}</button>
            <button className="button sm-join-host-button" onClick={()=>{this.hostGameButton()}}>
                {Main.instance?.isLoggedIn() ? "Host" : "Host as guest"}</button>
        </div>

        <p className="credits">Mafia, made by Samuel Maselli, Jack Papel, Isaac Worsencroft<br></br> and add your name here</p>
    </div>)}
}