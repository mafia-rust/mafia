import React from "react";
import GAME_MANAGER from "../../index";
import LobbyPlayerList from "./LobbyPlayerList";
import LobbyPhaseTimePane from "./LobbyPhaseTimePane";
import LobbyRolePane from "./LobbyRolePane";
import "./lobbyMenu.css";
import translate from "../../game/lang";
import { StateListener } from "../../game/gameManager.d";
import LobbyExcludedRoles from "./lobbyExcludedRoles";
import WikiSearch from "../WikiSearch";

type LobbyMenuProps = {}

type LobbyMenuState = {
    name: string
}
export default class LobbyMenu extends React.Component<LobbyMenuProps, LobbyMenuState> {
    constructor(props: LobbyMenuProps) {
        super(props);
        this.state = {
            name: ""
        }
        this.listener = (type)=>{
            this.setState({
                name: GAME_MANAGER.gameState.myName!
            });
        }
    }
    listener: StateListener;
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    render() {
        return <div className="lm">
            <header>
                {/* TODO: Place this nicely */}
                <button className="leave-button" onClick={() => GAME_MANAGER.leaveGame()}>
                    {translate("menu.button.leave")}
                </button>
                <button onClick={() => {
                    let code = new URL(window.location.href);
                    code.searchParams.set("code", GAME_MANAGER.roomCode!);
                    if (navigator.clipboard)
                        navigator.clipboard.writeText(code.toString());
                }}>
                    {GAME_MANAGER.roomCode!}
                </button> 
                <h1>{GAME_MANAGER.gameState.myName!}</h1>
                <button onClick={()=>{GAME_MANAGER.sendStartGamePacket()}}>
                    {translate("menu.lobby.button.start")}
                </button>
                
            </header>

            <main>
                <div className="left">
                    <LobbyPlayerList/>
                    <LobbyExcludedRoles/>
                    <WikiSearch/>
                </div>
                <div className="right">
                    <LobbyPhaseTimePane/>
                    <LobbyRolePane/>
                </div>

            </main>
        </div>
    }
}
