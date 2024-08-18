import React, { ReactElement, useEffect, useState } from "react";
import GAME_MANAGER from "../../index";
import LobbyPlayerList from "./LobbyPlayerList";
import "./lobbyMenu.css";
import translate from "../../game/lang";
import { StateListener } from "../../game/gameManager.d";
import Anchor from "../Anchor";
import { RoomLinkButton } from "../GlobalMenu";
import { RoleOutline, RoleList } from "../../game/roleListState.d";
import LoadingScreen from "../LoadingScreen";
import StartMenu from "../main/StartMenu";
import { defaultPhaseTimes } from "../../game/gameState";
import { GameModeContext } from "../../components/gameModeSettings/GameModesEditor";
import PhaseTimesSelector from "../../components/gameModeSettings/PhaseTimeSelector";
import { OutlineListSelector } from "../../components/gameModeSettings/OutlineSelector";
import EnabledRoleSelector from "../../components/gameModeSettings/EnabledRoleSelector";
import Icon from "../../components/Icon";
import { GameModeSelector } from "../../components/gameModeSettings/GameModeSelector";
import LobbyChatMenu from "./LobbyChatMenu";

export default function LobbyMenu(): ReactElement {
    const [roleList, setRoleList] = useState(
        GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game" ? GAME_MANAGER.state.roleList : []
    );
    const [enabledRoles, setEnabledRoles] = useState(
        GAME_MANAGER.state.stateType === "lobby"  || GAME_MANAGER.state.stateType === "game" ? GAME_MANAGER.state.enabledRoles : []
    );
    const [phaseTimes, setPhaseTimes] = useState(
        GAME_MANAGER.state.stateType === "lobby"  || GAME_MANAGER.state.stateType === "game" ? GAME_MANAGER.state.phaseTimes : defaultPhaseTimes()
    );
    const [isSpectator, setIsSpectator] = useState(()=>{
        return GAME_MANAGER.state.stateType === "lobby" ? GAME_MANAGER.getMySpectator() : false
    });
    const [isHost, setHost] = useState(GAME_MANAGER.getMyHost() ?? false);

    useEffect(() => {
        const listener: StateListener = async (type) => {
            if(GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game"){
                switch (type) {
                    case "roleList":
                        setRoleList([...GAME_MANAGER.state.roleList]);
                        break;
                    case "enabledRoles":
                        setEnabledRoles([...GAME_MANAGER.state.enabledRoles]);
                        break;
                    case "phaseTimes":
                    case "phaseTime":
                        setPhaseTimes({...GAME_MANAGER.state.phaseTimes})
                        break;
                    case "playersHost":
                    case "lobbyClients":
                        setHost(GAME_MANAGER.getMyHost() ?? false)
                        setIsSpectator(GAME_MANAGER.state.stateType === "lobby" ? GAME_MANAGER.getMySpectator() : false)
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
            setEnabledRoles([...GAME_MANAGER.state.enabledRoles]);
            setPhaseTimes({...GAME_MANAGER.state.phaseTimes})
            setHost(GAME_MANAGER.getMyHost() ?? false)
        }
        GAME_MANAGER.addStateListener(listener);
        return ()=>{GAME_MANAGER.removeStateListener(listener);}
    }, [setRoleList, setEnabledRoles]);
    
    let onChangeRolePicker = (value: RoleOutline, index: number) => {
        let newRoleList = [...roleList];
        newRoleList[index] = value;
        setRoleList(newRoleList);
        GAME_MANAGER.sendSetRoleOutlinePacket(index, value);
    }

    const sendRoleList = (newRoleList: RoleList) => {
        const combinedRoleList = [...roleList];
        newRoleList.forEach((role, index) => {
            combinedRoleList[index] = role
        })
        GAME_MANAGER.sendSetRoleListPacket(combinedRoleList);
    };

    return <div className="lm">
        <LobbyMenuHeader/>
        <GameModeContext.Provider value={{roleList, enabledRoles, phaseTimes}}>
            <main>
                <div>
                    <LobbyPlayerList/>
                    <LobbyChatMenu spectator={isSpectator}/>
                </div>
                <div>
                    {Anchor.isMobile() && <h1>{translate("menu.lobby.settings")}</h1>}
                    {isHost && <GameModeSelector 
                        canModifySavedGameModes={false}
                        loadGameMode={gameMode => {
                            GAME_MANAGER.sendSetPhaseTimesPacket(gameMode.phaseTimes);
                            GAME_MANAGER.sendEnabledRolesPacket(gameMode.enabledRoles);
                            GAME_MANAGER.sendSetRoleListPacket(gameMode.roleList);
                        }}
                    />}
                    <PhaseTimesSelector 
                        disabled={!isHost}
                        onChange={pts => GAME_MANAGER.sendSetPhaseTimesPacket(pts)}
                    />
                    <OutlineListSelector
                        disabled={!isHost}
                        onChangeRolePicker={onChangeRolePicker}
                        onAddNewOutline={undefined}
                        onRemoveOutline={undefined}
                        setRoleList={sendRoleList}
                    />
                    <EnabledRoleSelector
                        onEnableRoles={roles => GAME_MANAGER.sendEnabledRolesPacket([...enabledRoles, ...roles])}
                        onDisableRoles={roles => GAME_MANAGER.sendEnabledRolesPacket(enabledRoles.filter(role => !roles.includes(role)))}
                        onIncludeAll={() => GAME_MANAGER.sendEnabledRolesPacket([])}
                        disabled={!isHost}
                    />
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
                <Icon>play_arrow</Icon>{translate("menu.lobby.button.start")}
            </button>
            <RoomLinkButton/>
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

