import React from "react";
import gameManager from "../../index.js";
import {LobbyPlayerList} from "./LobbyPlayerList.js";
import {LobbySettingsPane} from "./LobbySettingsPane.js";
import "./lobbyMenu.css";

export class LobbyMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            playerList: undefined,
            settings: undefined,
        };
        this.listener = ()=>{
            this.setState({
                playerList: <LobbyPlayerList/>,
                settings: <LobbySettingsPane/>,
            })
        };
    }
    componentDidMount() {
        gameManager.addStateListener(this.listener);
    }
    componentWillUnmount() {
        gameManager.removeStateListener(this.listener);
    }

    render(){return(<div style={{
        display: "flex",
        flexDirection: "column",
    }}>
        <div className="header lm-header">
            <h1 className="header-text lm-header-text">Lobby {gameManager.roomCode}</h1>
            <button className="button lm-start-button" onClick={gameManager.startGame_button}>Start Game</button>
        </div>
        <div style={{
            display: "flex",
            flexDirection: "row",
        }}>
            {this.state.playerList}
            {this.state.settings}
        </div>
    </div>)}
}
