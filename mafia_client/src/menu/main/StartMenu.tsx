import React from "react";
import gameManager from "@";
import { create_gameState } from "@game/gameState";
import Anchor from "@menu/Anchor";
import "@/index.css"
import "./startMenu.css"
import * as LoadingScreen from "@menu/LoadingScreen";
import JoinMenu from "./JoinMenu";
import translate from "@game/lang";

export default class StartMenu extends React.Component {
    render(){
        let logged_in = false /* TODO */;
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

    private joinGameButton() {
        gameManager.gameState = create_gameState();
        Anchor.setContent(<JoinMenu/>);
    }
    
    private hostGameButton() {
        gameManager.gameState = create_gameState();
        
        Anchor.setContent(LoadingScreen.create(LoadingScreen.Type.Host));

        gameManager.Server.close();
        gameManager.Server.open();

        // Wait for server to open
        setTimeout(gameManager.host_button, 5000);  //TODO
        // Lobby menu opens when AcceptHost packet is recieved
    }
}