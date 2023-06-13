import React from "react";
import GAME_MANAGER from "../../index";
import { createGameState } from "../../game/gameState";
import Anchor from "../Anchor";
import "../../index.css"
import "./startMenu.css"
import * as LoadingScreen from "../LoadingScreen";
import JoinMenu from "./JoinMenu";
import translate from "../../game/lang";
import WikiSearch from "../WikiSearch";

type StartMenuProps = {
}
type StartMenuState = {
} 

export default class StartMenu extends React.Component<StartMenuProps, StartMenuState> {
    private joinGameButton() {
        GAME_MANAGER.gameState = createGameState();
        Anchor.setContent(<JoinMenu roomCode={null}/>);
    }
    
    private async hostGameButton() {
        GAME_MANAGER.gameState = createGameState();
        
        Anchor.setContent(LoadingScreen.create(LoadingScreen.Type.Host));

        GAME_MANAGER.server.close();
        await GAME_MANAGER.server.open();

        GAME_MANAGER.sendHostPacket();
        // Lobby menu opens when AcceptHost packet is recieved
    }

    render(){return(<div className="sm">
        <div className="hero">
            <div className="sm-center-content sm-main">
                <header>
                    <h1>{translate("menu.start.title")}</h1>
                </header>
                
                <div>
                    <button onClick={()=>{this.joinGameButton()}}>
                        {translate("menu.start.button.join")}
                    </button>
                    <button onClick={()=>{this.hostGameButton()}}>
                        {translate("menu.start.button.host")}
                    </button>
                </div>
            </div>
        </div>
        <div className="hero">
            <div className="sm-center-content sm-wiki">
                <header>
                    <h2>{translate("menu.wiki.title")}</h2>
                </header>
                
                <WikiSearch/>
            </div>
        </div>
    </div>)}
}