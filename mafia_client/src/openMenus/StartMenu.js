import React from "react";
import gameManager from "../index.js";
import { create_gameState } from "../game/gameState";
import {Main} from "../Main";
import { JoinMenu } from "./JoinMenu";
import "../index.css"
import "./startMenu.css"

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
    render(){return(<div>
        <div className="header sm-header">
            <h1 className="header-text">Mafia</h1>
            <button className="button sm-login-button">Login</button><br/>
        </div>

        <div className="sm-button-area">
            <button className="button sm-join-host-button" onClick={()=>{
                gameManager.gameState = create_gameState();
                Main.instance.setContent(<JoinMenu/>);
            }}>{Main.instance?.isLoggedIn() ? "Join" : "Join as guest"}</button>
            <button className="button sm-join-host-button" onClick={()=>{
                // Create gamestate, generate lobby,
                // and enter LobbyMenu screen
            }}>{Main.instance?.isLoggedIn() ? "Host" : "Host as guest"}</button>
        </div>

        <p className="credits">Mafia, made by Sammy Maselli, Jack Papel, and add your name here</p>
    </div>)}
}