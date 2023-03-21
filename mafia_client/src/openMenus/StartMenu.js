import React from "react";
import gameManager from "../index.js";
import { create_gameState } from "../game/gameState";
import {Main} from "../Main";
import "../index.css"
import "./startMenu.css"
import { LoadingMenu } from "./LoadingMenu.js";
import { JoinMenu } from "./JoinMenu.js";
import { translate } from "../game/lang.js";

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
        
        Main.instance.setContent(<LoadingMenu value={translate("menu.loading.host")}/>);

        gameManager.Server.close();
        gameManager.Server.open();

        // Wait for server to open
        setTimeout(gameManager.host_button, 5000);  //TODO
        // Lobby menu opens when AcceptHost packet is recieved
    }
    render(){
        let logged_in = Main.instance?.isLoggedIn();
        return(<div>
        <div className="header sm-header">
            <h1 className="header-text">{translate("menu.start.title")}</h1>
            <button className="button sm-login-button">
                {translate("menu.start.button." + (logged_in ? "logout" : "login"))}
            </button><br/>
        </div>

        <div className="sm-button-area">
            <button className="button sm-join-host-button" onClick={()=>{this.joinGameButton()}}>
                {translate("menu.start.button.join." + (logged_in ? "logged_out" : "logged_in"))}
            </button>
            <button className="button sm-join-host-button" onClick={()=>{this.hostGameButton()}}>
                {translate("menu.start.button.host." + (logged_in ? "logged_out" : "logged_in"))}
            </button>
        </div>

        <p className="credits">{translate("menu.start.credits")}</p>
    </div>)}
}