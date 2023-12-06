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
    name: string,
    host: boolean
}
export default class LobbyMenu extends React.Component<LobbyMenuProps, LobbyMenuState> {
    constructor(props: LobbyMenuProps) {
        super(props);
        this.state = {
            name: "",
            host: GAME_MANAGER.gameState.host
        }
        this.listener = (type)=>{
            this.setState({
                name: GAME_MANAGER.gameState.myName!,
                host: GAME_MANAGER.gameState.host
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
            <LobbyMenuHeader host={this.state.host}/>
            <main>
                <div>
                    <LobbyPlayerList/>
                    {Anchor.isMobile() || <section>
                        <h2>{translate("menu.wiki.title")}</h2>
                        <WikiSearch/>
                    </section>}
                </div>
                <div>
                    {Anchor.isMobile() && <h1>{translate("menu.lobby.settings")}</h1>}
                    <LobbyPhaseTimePane/>
                    <LobbyRolePane/>
                    <LobbyExcludedRoles/>
                    {Anchor.isMobile() && <section>
                        <h2>{translate("menu.wiki.title")}</h2>
                        <WikiSearch/>
                    </section>}
                </div>
            </main>
        </div>
    }
}

// There's probably a better way to do this that doesn't need the mobile check.
function LobbyMenuHeader(props: { host?: boolean }): JSX.Element {
    return <header>
        <button className="leave" onClick={() => GAME_MANAGER.leaveGame()}>
            {translate("menu.button.leave")}
        </button>
        <RoomCodeButton/>
        {Anchor.isMobile() || <h1>{GAME_MANAGER.gameState.myName!}</h1>}
        <button disabled={!props.host} className="start" onClick={()=>{GAME_MANAGER.sendStartGamePacket()}}>
            {translate("menu.lobby.button.start")}
        </button>
        {Anchor.isMobile() && <h1>{GAME_MANAGER.gameState.myName!}</h1>}
    </header>
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