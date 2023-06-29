import React from "react";
import GAME_MANAGER from "../../index";
import LobbyPlayerList from "./LobbyPlayerList";
import LobbyPhaseTimePane from "./LobbyPhaseTimePane";
import LobbyRolePane from "./LobbyRolePane";
import "./lobbyMenu.css";
import translate from "../../game/lang";
import { StateListener } from "../../game/gameManager.d";
import LobbyExcludedRoles from "./lobbyExcludedRoles";
import Anchor from "../Anchor";
import WikiSearch from "../../components/WikiSearch";

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
            <LobbyMenuHeader/>
            <main>
                <div className="left">
                    <LobbyPlayerList/>
                    <section>
                        <header>
                            <h2>{translate("menu.wiki.title")}</h2>
                        </header>
                        <WikiSearch/>
                    </section>
                </div>
                <div className="right">
                    {Anchor.isMobile() && <h1>{translate("menu.lobby.gameSettings")}</h1>}
                    <LobbyPhaseTimePane/>
                    <LobbyRolePane/>
                    <LobbyExcludedRoles/>
                </div>
            </main>
        </div>
    }
}

// There's probably a better way to do this that doesn't need the mobile check.
function LobbyMenuHeader(props: {}): JSX.Element {
    if (Anchor.isMobile()) {
        return <header>
            <div>
                <button className="leave" onClick={() => GAME_MANAGER.leaveGame()}>
                    {translate("menu.button.leave")}
                </button>
                <RoomCodeButton/>
                <button className="start" onClick={()=>{GAME_MANAGER.sendStartGamePacket()}}>
                    {translate("menu.lobby.button.start")}
                </button>
            </div>
            <h1>{GAME_MANAGER.gameState.myName!}</h1>
        </header>
    } else {
        return <header>
            <button className="leave" onClick={() => GAME_MANAGER.leaveGame()}>
                {translate("menu.button.leave")}
            </button>
            <RoomCodeButton/>
            <h1>{GAME_MANAGER.gameState.myName!}</h1>
            <button className="start" onClick={()=>{GAME_MANAGER.sendStartGamePacket()}}>
                {translate("menu.lobby.button.start")}
            </button>
        </header>
    }
}

function RoomCodeButton(props: {}): JSX.Element {
    return <button onClick={() => {
        let code = new URL(window.location.href);
        code.searchParams.set("code", GAME_MANAGER.roomCode!);
        if (navigator.clipboard)
            navigator.clipboard.writeText(code.toString());
    }}>
        {GAME_MANAGER.roomCode!}
    </button>
}