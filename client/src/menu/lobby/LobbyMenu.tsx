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
import LoadingScreen from "../LoadingScreen";
import StartMenu from "../main/StartMenu";

export default function LobbyMenu(): ReactElement {
    const [roleList, setRoleList] = useState(
        GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game" ? GAME_MANAGER.state.roleList : []
    );
    const [excludedRoles, setExcludedRoles] = useState(
        GAME_MANAGER.state.stateType === "lobby"  || GAME_MANAGER.state.stateType === "game" ? GAME_MANAGER.state.excludedRoles : []
    );

    useEffect(() => {
        const listener: StateListener = async (type) => {
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
                    case "rejectJoin":
                        // Kicked, probably
                        Anchor.setContent(<LoadingScreen type="disconnect"/>);
                        await GAME_MANAGER.setDisconnectedState();
                        Anchor.setContent(<StartMenu />);
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

    const importFromClipboard = () => {
        navigator.clipboard.readText().then((text) => {
            try {
                const data = JSON.parse(text);
                if(!data.roleList || !data.phaseTimes || !data.disabledRoles) return;
                GAME_MANAGER.sendExcludedRolesPacket(data.disabledRoles);
                GAME_MANAGER.sendSetRoleListPacket(data.roleList);
                GAME_MANAGER.sendSetPhaseTimesPacket(data.phaseTimes);
            } catch (e) {
                console.error(e);
            }
        });
    }

    return <div className="lm">
        <LobbyMenuHeader/>
        <main>
            <div>
                <LobbyPlayerList/>
                {Anchor.isMobile() || <section className="wiki-menu-colors selector-section">
                    <h2>{translate("menu.wiki.title")}</h2>
                    <WikiSearch excludedRoles={
                        getRolesComplement(getRolesFromRoleListRemoveExclusionsAddConversions(roleList, excludedRoles))
                    }/>
                </section>}
            </div>
            <div>
                {Anchor.isMobile() && <h1>{translate("menu.lobby.settings")}</h1>}
                <button className="player-list-menu-colors" onClick={importFromClipboard}>{translate("importFromClipboard")}</button>
                <LobbyPhaseTimePane/>
                <LobbyRolePane/>
                <LobbyExcludedRoles/>
                {Anchor.isMobile() && <section className="wiki-menu-colors selector-section">
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

        setHost(GAME_MANAGER.getMyHost() ?? false);
        if(GAME_MANAGER.state.stateType === "lobby")
            setLobbyName(GAME_MANAGER.state.lobbyName);

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
                onKeyUp={(e)=>{
                    if(e.key !== 'Enter') return;
                    
                    const newLobbyName = (e.target as HTMLInputElement).value;
                    setLobbyName(newLobbyName);
                    GAME_MANAGER.sendSetLobbyNamePacket(newLobbyName);
                    
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

