import React from "react";
import GAME_MANAGER from "../../index";
import { createGameState } from "../../game/gameState";
import Anchor from "../Anchor";
import "../../index.css"
import "./startMenu.css"
import * as LoadingScreen from "../LoadingScreen";
import JoinMenu from "./JoinMenu";
import translate from "../../game/lang";

export default class StartMenu extends React.Component {
    render(){
        let loggedIn = false /* TODO */;
        return(<div>
        <header className="sm-header">
            <h1>{translate("menu.start.title")}</h1>
            <button className="sm-login-button">
                {translate("menu.start.button." + (loggedIn ? "logout" : "login"))}
            </button><br/>
        </header>

        <div className="sm-button-area">
            <button className="sm-join-host-button" onClick={()=>{this.joinGameButton()}}>
                {translate("menu.start.button.join." + (loggedIn ? "loggedIn" : "loggedOut"))}
            </button>
            <button className="sm-join-host-button" onClick={()=>{this.hostGameButton()}}>
                {translate("menu.start.button.host." + (loggedIn ? "loggedIn" : "loggedOut"))}
            </button>
        </div>

        <footer className="credits">{translate("menu.start.credits")}</footer>
    </div>)}

    private joinGameButton() {
        GAME_MANAGER.gameState = createGameState();
        Anchor.setContent(<JoinMenu/>);
    }
    
    private async hostGameButton() {
        GAME_MANAGER.gameState = createGameState();
        
        Anchor.setContent(LoadingScreen.create(LoadingScreen.Type.Host));

        GAME_MANAGER.server.close();
        await GAME_MANAGER.server.open();

        GAME_MANAGER.sendHostPacket();
        // Lobby menu opens when AcceptHost packet is recieved
    }
}