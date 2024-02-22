import React, { ReactElement, useEffect, useState } from "react";
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
import { RoomCodeButton } from "../Settings";
import { getRolesFromRoleListRemoveExclusionsAddConversions, getRolesComplement } from "../../game/roleListState.d";
import LobbyModifierMenu from "./LobbyModifierMenu";
import LoadingScreen from "../LoadingScreen";

export default function LobbyMenu(): ReactElement {
    const [roleList, setRoleList] = useState(
        GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game" ? GAME_MANAGER.state.roleList : []
    );
    const [excludedRoles, setExcludedRoles] = useState(
        GAME_MANAGER.state.stateType === "lobby"  || GAME_MANAGER.state.stateType === "game" ? GAME_MANAGER.state.excludedRoles : []
    );

    useEffect(() => {
        const listener: StateListener = (type) => {
            if(GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game"){
                switch (type) {
                    case "roleList":
                        setRoleList([...GAME_MANAGER.state.roleList]);
                        break;
                    case "roleOutline":
                        setRoleList([...GAME_MANAGER.state.roleList]);
                        break;
                    case "excludedRoles":
                        setExcludedRoles([...GAME_MANAGER.state.excludedRoles]);
                        break;
                }
            }
        }

        if(GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game"){
            setRoleList([...GAME_MANAGER.state.roleList]);
            setExcludedRoles([...GAME_MANAGER.state.excludedRoles]);
        }
        GAME_MANAGER.addStateListener(listener);
        return ()=>{GAME_MANAGER.removeStateListener(listener);}
    }, [setRoleList, setExcludedRoles]);


    return <div className="lm">
        <LobbyMenuHeader/>
        <main>
            <div>
                <LobbyPlayerList/>
                {Anchor.isMobile() || <section className="wiki-menu-colors">
                    <h2>{translate("menu.wiki.title")}</h2>
                    <WikiSearch excludedRoles={
                        getRolesComplement(getRolesFromRoleListRemoveExclusionsAddConversions(roleList, excludedRoles))
                    }/>
                </section>}
            </div>
            <div>
                {Anchor.isMobile() && <h1>{translate("menu.lobby.settings")}</h1>}
                <LobbyModifierMenu/>
                <LobbyPhaseTimePane/>
                <LobbyRolePane/>
                <LobbyExcludedRoles/>
                {Anchor.isMobile() && <section className="wiki-menu-colors">
                    <h2>{translate("menu.wiki.title")}</h2>
                    <WikiSearch excludedRoles={
                        getRolesComplement(getRolesFromRoleListRemoveExclusionsAddConversions(roleList, excludedRoles))
                    }/>
                </section>}
            </div>
        </main>
    </div>
}

// There's probably a better way to do this that doesn't need the mobile check.
function LobbyMenuHeader(): JSX.Element {
    const [lobbyName, setLobbyName] = useState<string>(GAME_MANAGER.state.stateType === "lobby" ? GAME_MANAGER.state.lobbyName : "Mafia Lobby");
    const [host, setHost] = useState(GAME_MANAGER.getMyHost() ?? false);

    useEffect(() => {
        const listener: StateListener = (type) => {
            switch (type) {
                case "playersHost":
                    setHost(GAME_MANAGER.getMyHost() ?? false);
                    break;
                case "lobbyName":
                    if(GAME_MANAGER.state.stateType === "lobby")
                        setLobbyName(GAME_MANAGER.state.lobbyName);
                    break;
            }
        }

        GAME_MANAGER.addStateListener(listener)
        
        return ()=>{GAME_MANAGER.removeStateListener(listener);}
    }, [setHost, setLobbyName]);

    return <header>
        <div>
            <button disabled={!host} className="start" onClick={async ()=>{
                Anchor.setContent(<LoadingScreen type="default"/>);
                if (!await GAME_MANAGER.sendStartGamePacket()) {
                    Anchor.setContent(<LobbyMenu/>)
                }
            }}>
                {translate("menu.lobby.button.start")}
            </button>
            <RoomCodeButton/>
        </div>
        { host ? 
            <input 
                type="text" 
                value={lobbyName}
                onInput={e => {
                    setLobbyName((e.target as HTMLInputElement).value);
                }}
                onBlur={e => {
                    const newLobbyName = (e.target as HTMLInputElement).value;
                    setLobbyName(newLobbyName);
                    GAME_MANAGER.sendSetLobbyNamePacket(newLobbyName);
                }}
            /> : 
            <h1>{lobbyName}</h1>
        }
        
    </header>
}

