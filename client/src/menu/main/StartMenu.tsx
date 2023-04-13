import React from "react";
import GAME_MANAGER from "../../index";
import { create_gameState } from "../../game/gameState";
import Anchor from "../Anchor";
import "../../index.css"
import "./startMenu.css"
import * as LoadingScreen from "../LoadingScreen";
import JoinMenu from "./JoinMenu";
import translate from "../../game/lang";

export default class StartMenu extends React.Component {
    render(){
        let logged_in = false /* TODO */;
        return(<div>
        <header className="sm-header">
            <h1>{translate("menu.start.title")}</h1>
            <button className="sm-login-button">
                {translate("menu.start.button." + (logged_in ? "logout" : "login"))}
            </button><br/>
        </header>

        <div className="sm-button-area">
            <button className="sm-join-host-button" onClick={()=>{this.joinGameButton()}}>
                {translate("menu.start.button.join." + (logged_in ? "logged_out" : "logged_in"))}
            </button>
            <button className="sm-join-host-button" onClick={()=>{this.hostGameButton()}}>
                {translate("menu.start.button.host." + (logged_in ? "logged_out" : "logged_in"))}
            </button>
        </div>

        <footer className="credits">{translate("menu.start.credits")}</footer>
    </div>)}

    private joinGameButton() {
        GAME_MANAGER.gameState = create_gameState();
        Anchor.setContent(<JoinMenu/>);
    }
    
    private async hostGameButton() {
        GAME_MANAGER.gameState = create_gameState();
        
        Anchor.setContent(LoadingScreen.create(LoadingScreen.Type.Host));

        GAME_MANAGER.Server.close();
        await GAME_MANAGER.Server.open();

        GAME_MANAGER.host_button();
        // Lobby menu opens when AcceptHost packet is recieved
    }
}