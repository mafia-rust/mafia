import React from "react";
import gameManager from "../../index.js";
import {LobbyPlayerList} from "./LobbyPlayerList.js";
import { LobbyPhaseTimePane } from "./LobbyPhaseTimePane.js";
import { LobbyRolePane } from "./LobbyRolePane.js";
import "./lobbyMenu.css";
import { translate } from "../../game/lang.js";

export class LobbyMenu extends React.Component {
    constructor(props) {
        super(props);
    }
    componentDidMount() {
    }
    componentWillUnmount() {
    }

    render(){return(<div style={{
        display: "flex",
        flexDirection: "column",
    }}>
        <div className="header lm-header">
            <h1 className="header-text lm-header-text">
                {translate("menu.lobby.title", gameManager.roomCode)}
            </h1>
            <button className="button lm-start-button" onClick={gameManager.startGame_button}>
                {translate("menu.lobby.button.start")}
            </button>
        </div>
        <div style={{
            display: "flex",
            flexDirection: "row",
        }}>
            <LobbyPlayerList/>
            <div style={{
                display: "flex",
                flexDirection: "column",
            }}>
                <LobbyPhaseTimePane/>
                <LobbyRolePane/>
            </div>
            
        </div>
    </div>)}
}
