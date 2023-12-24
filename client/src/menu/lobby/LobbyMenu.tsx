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
        
        if(GAME_MANAGER.state.stateType === "lobby")
            this.state = {
                name: GAME_MANAGER.getMyName() ?? "",
                host: GAME_MANAGER.getMyHost() ?? false,
            }
        this.listener = (type)=>{
            if(GAME_MANAGER.state.stateType === "lobby")
                this.setState({
                    name: GAME_MANAGER.getMyName() ?? "",
                    host: GAME_MANAGER.getMyHost() ?? false,
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
                    {Anchor.isMobile() || <section className="wiki-menu-colors">
                        <h2>{translate("menu.wiki.title")}</h2>
                        <WikiSearch excludedRoles={
                            GAME_MANAGER.state.stateType === "lobby" ?
                            GAME_MANAGER.state.excludedRoles :
                            []
                        }/>
                    </section>}
                </div>
                <div>
                    {Anchor.isMobile() && <h1>{translate("menu.lobby.settings")}</h1>}
                    <LobbyPhaseTimePane/>
                    <LobbyRolePane/>
                    <LobbyExcludedRoles/>
                    {Anchor.isMobile() && <section className="wiki-menu-colors">
                        <h2>{translate("menu.wiki.title")}</h2>
                        <WikiSearch  excludedRoles={
                            GAME_MANAGER.state.stateType === "lobby" ?
                            GAME_MANAGER.state.excludedRoles :
                            []
                        }/>
                    </section>}
                </div>
            </main>
        </div>
    }
}

// There's probably a better way to do this that doesn't need the mobile check.
function LobbyMenuHeader(props: { host?: boolean }): JSX.Element {
    return <header>
        <div>
            <button disabled={!props.host} className="start" onClick={()=>{GAME_MANAGER.sendStartGamePacket()}}>
                {translate("menu.lobby.button.start")}
            </button>
            <RoomCodeButton/>
        </div>
        <button className="leave" onClick={() => GAME_MANAGER.leaveGame()}>
            {translate("menu.button.leave")}
        </button>
        
    </header>
}

function RoomCodeButton(props: {}): JSX.Element {
    return <button onClick={() => {
        let code = new URL(window.location.href);
        
        if (GAME_MANAGER.state.stateType === "lobby")
            code.searchParams.set("code", GAME_MANAGER.state.roomCode!);

        if (navigator.clipboard)
            navigator.clipboard.writeText(code.toString());
    }}>
        {
            GAME_MANAGER.state.stateType === "lobby" ?
            GAME_MANAGER.state.roomCode : ""
        }
    </button>
}