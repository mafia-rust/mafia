import React from "react";
import GAME_MANAGER from "../../index";
import LobbyPlayerList from "./LobbyPlayerList";
import LobbyPhaseTimePane from "./LobbyPhaseTimePane";
import LobbyRolePane from "./LobbyRolePane";
import "./lobbyMenu.css";
import translate from "../../game/lang";

export default class LobbyMenu extends React.Component {
    render(){return(<div style={{
        display: "flex",
        flexDirection: "column",
    }}>
        <div className="header lm-header">
            <h1 className="header-text lm-header-text">
                {translate("menu.lobby.title", GAME_MANAGER.roomCode)}
            </h1>
            <button className="button lm-start-button" onClick={GAME_MANAGER.startGame_button}>
                {translate("menu.lobby.button.start")}
            </button>
        </div>
        <div style={{
            display: "flex",
            flexDirection: "row",
        }}>
            <LobbyPlayerList/>
            <div className="lm-settings-column">
                <LobbyPhaseTimePane/>
                <LobbyRolePane/>
            </div>
            
        </div>
    </div>)}
}
