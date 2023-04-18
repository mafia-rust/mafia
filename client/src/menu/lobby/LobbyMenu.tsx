import React from "react";
import GAME_MANAGER from "../../index";
import LobbyPlayerList from "./LobbyPlayerList";
import LobbyPhaseTimePane from "./LobbyPhaseTimePane";
import LobbyRolePane from "./LobbyRolePane";
import "./lobbyMenu.css";
import translate from "../../game/lang";

export function create() {
    return <div style={{
        display: "flex",
        flexDirection: "column",
    }}>
        <header className="lm-header">
            <h1>
                {translate("menu.lobby.title", GAME_MANAGER.roomCode!)}
            </h1>
            <button className="lm-start-button" onClick={GAME_MANAGER.startGame_button}>
                {translate("menu.lobby.button.start")}
            </button>
        </header>
        <main className="lm-main">
            <LobbyPlayerList/>
            <div className="lm-settings">
                <h2>Game settings</h2>
                <LobbyPhaseTimePane/>
                <LobbyRolePane/>
            </div>

        </main>
    </div>
}
