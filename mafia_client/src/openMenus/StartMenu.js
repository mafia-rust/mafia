import React from "react";
import gameManager from "../index.js";
import { create_gameState } from "../game/gameState";
import {Main} from "../Main";
import { JoinMenu } from "./JoinMenu";
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
        <div className="header">
            <h1 className="title">Mafia</h1>
            <button className="login-button">Login</button><br/>
        </div>

        <div className="button-area">
            <button className="join-host-button" onClick={()=>{
                gameManager.gameState = create_gameState();
                Main.instance.setContent(<JoinMenu/>);
            }}>{Main.instance?.isLoggedIn() ? "Join" : "Join as guest"}</button>
            <button className="join-host-button" onClick={()=>{
                gameManager.gameState = create_gameState();
                Main.instance.setContent(<JoinMenu/>);
            }}>{Main.instance?.isLoggedIn() ? "Host" : "Host as guest"}</button>
        </div>

        <p className="credits">Mafia, made by Sammy Maselli, Jack Papel, and add your name here</p>
    </div>)}
}