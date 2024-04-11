import React, { ReactElement, useEffect, useState } from "react";
import GAME_MANAGER from "../../index";
import LobbyPlayerList from "./LobbyPlayerList";
import "./lobbyMenu.css";
import translate from "../../game/lang";
import { StateListener } from "../../game/gameManager.d";
import Anchor from "../Anchor";
import { RoomCodeButton } from "../Settings";
import { getRolesFromRoleListRemoveExclusionsAddConversions, getRolesComplement, RoleOutline } from "../../game/roleListState.d";
import LoadingScreen from "../LoadingScreen";
import StartMenu from "../main/StartMenu";
import Wiki from "../../components/Wiki";
import { defaultPhaseTimes } from "../../game/gameState";
import { PasteButton } from "../../components/ClipboardButtons";
import { GameModeContext } from "../../components/GameModesEditor";
import PhaseTimesSelector from "../../components/PhaseTimeSelector";
import { OutlineListSelector } from "../../components/OutlineSelector";
import DisabledRoleSelector from "../../components/DisabledRoleSelector";

export default function LobbyMenu(): ReactElement {
    const [roleList, setRoleList] = useState(
        GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game" ? GAME_MANAGER.state.roleList : []
    );
    const [disabledRoles, setDisabledRoles] = useState(
        GAME_MANAGER.state.stateType === "lobby"  || GAME_MANAGER.state.stateType === "game" ? GAME_MANAGER.state.excludedRoles : []
    );
    const [phaseTimes, setPhaseTimes] = useState(
        GAME_MANAGER.state.stateType === "lobby"  || GAME_MANAGER.state.stateType === "game" ? GAME_MANAGER.state.phaseTimes : defaultPhaseTimes()
    );
    const [isHost, setHost] = useState(GAME_MANAGER.getMyHost() ?? false);

    useEffect(() => {
        const listener: StateListener = async (type) => {
            if(GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game"){
                switch (type) {
                    case "roleList":
                    case "roleOutline":
                        setRoleList([...GAME_MANAGER.state.roleList]);
                        break;
                    case "excludedRoles":
                        setDisabledRoles([...GAME_MANAGER.state.excludedRoles]);
                        break;
                    case "phaseTimes":
                    case "phaseTime":
                        setPhaseTimes({...GAME_MANAGER.state.phaseTimes})
                        break;
                    case "playersHost":
                    case "lobbyClients":
                        setHost(GAME_MANAGER.getMyHost() ?? false)
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
            setDisabledRoles([...GAME_MANAGER.state.excludedRoles]);
            setPhaseTimes({...GAME_MANAGER.state.phaseTimes})
            setHost(GAME_MANAGER.getMyHost() ?? false)
        }
        GAME_MANAGER.addStateListener(listener);
        return ()=>{GAME_MANAGER.removeStateListener(listener);}
    }, [setRoleList, setDisabledRoles]);
    
    let onChangeRolePicker = (value: RoleOutline, index: number) => {
        let newRoleList = [...roleList];
        newRoleList[index] = value;
        setRoleList(newRoleList);
        GAME_MANAGER.sendSetRoleOutlinePacket(index, value);
    }

    return <div className="lm">
        <LobbyMenuHeader/>
        <GameModeContext.Provider value={{roleList, disabledRoles, phaseTimes}}>
            <main>
                <div>
                    <LobbyPlayerList/>
                    {Anchor.isMobile() || <section className="wiki-menu-colors selector-section">
                        <h2>{translate("menu.wiki.title")}</h2>
                        <Wiki disabledRoles={
                            getRolesComplement(getRolesFromRoleListRemoveExclusionsAddConversions(roleList, disabledRoles))
                        }/>
                    </section>}
                </div>
                <div>
                    {Anchor.isMobile() && <h1>{translate("menu.lobby.settings")}</h1>}
                    <PasteButton 
                        className="player-list-menu-colors" 
                        disabled={!GAME_MANAGER.getMyHost() ?? false}
                        onClipboardRead={text => {
                            try {
                                const data = JSON.parse(text);

                                GAME_MANAGER.sendExcludedRolesPacket(data.disabledRoles ?? []);
                                GAME_MANAGER.sendSetRoleListPacket(data.roleList ?? []);
                                GAME_MANAGER.sendSetPhaseTimesPacket(data.phaseTimes ?? defaultPhaseTimes());
                            } catch (e) {
                                Anchor.pushError(
                                    translate("notification.importGameMode.failure"), 
                                    translate("notification.importGameMode.failure.details")
                                );
                            }
                        }}
                    >{translate("importFromClipboard")}</PasteButton>
                    <PhaseTimesSelector 
                        disabled={!isHost}
                        onChange={GAME_MANAGER.sendSetPhaseTimesPacket}
                    />
                    <OutlineListSelector
                        disabled={!isHost}
                        onChangeRolePicker={onChangeRolePicker}
                        onAddNewOutline={undefined}
                        onRemoveOutline={undefined}
                        setRoleList={newRoleList => {
                            const combinedRoleList = [...roleList];
                            newRoleList.forEach((role, index) => {
                                combinedRoleList[index] = role
                            })
                            setRoleList(combinedRoleList)
                        }}
                    />
                    <DisabledRoleSelector
                        onDisableRoles={roles => GAME_MANAGER.sendExcludedRolesPacket([...disabledRoles, ...roles])}
                        onEnableRoles={roles => GAME_MANAGER.sendExcludedRolesPacket(disabledRoles.filter(role => !roles.includes(role)))}
                        onIncludeAll={() => GAME_MANAGER.sendExcludedRolesPacket([])}
                        disabled={!isHost}
                    />
                    {Anchor.isMobile() && <section className="wiki-menu-colors selector-section">
                        <h2>{translate("menu.wiki.title")}</h2>
                        <Wiki disabledRoles={
                            getRolesComplement(getRolesFromRoleListRemoveExclusionsAddConversions(roleList, disabledRoles))
                        }/>
                    </section>}
                </div>
            </main>
        </GameModeContext.Provider>
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

