import { ReactElement, useState } from "react";
import { SavedGameModes, loadGameModes, saveGameModes } from "../game/localStorage";
import React from "react";
import { OutlineListSelector } from "../components/OutlineSelector";
import { RoleList, RoleOutline } from "../game/roleListState.d";
import translate from "../game/lang";
import "./gameModesBank.css";
import Anchor from "./Anchor";
import PhaseTimesSelector from "../components/PhaseTimeSelector";
import { PhaseTimes } from "../game/gameState.d";
import DisabledRoleSelector from "../components/DisabledRoleSelector";
import { Role } from "../game/roleState.d";
import "../components/selectorSection.css";


export default function GameModesBank(): ReactElement {

    const [savedGameModes, setGameModes] = useState<SavedGameModes>(loadGameModes() ?? new Map());

    const [currentSettingsName, setCurrentRoleListName] = useState<string>("");

    const [currentRoleList, setCurrentRoleList] = useState<RoleList>([]);
    const [currentPhaseTimes, setCurrentPhaseTimes] = useState<PhaseTimes>({
        morning: 15,
        discussion: 46,
        voting: 30,
        testimony: 24,
        judgement: 20,
        evening: 10,
        night: 37
    });
    const [currentDisabledRoles, setCurrentDisabledRoles] = useState<Role[]>([]);


    const onChangeRolePicker = (value: RoleOutline, index: number) => {
        let newRoleList = [...currentRoleList];
        newRoleList[index] = value;
        setCurrentRoleList(newRoleList);
    }
    const addOutline = () => {
        setCurrentRoleList([...currentRoleList, {type: "any"}]);
    }
    const removeOutline = (index: number) => {
        let newRoleList = [...currentRoleList];
        newRoleList.splice(index, 1);
        setCurrentRoleList(newRoleList);
    }


    const onDisableRoles = (roles: Role[]) => {
        const newDisabledRoles = [...currentDisabledRoles];
        for(const role of roles){
            if(!newDisabledRoles.includes(role)){
                newDisabledRoles.push(role);
            }
        }
        setCurrentDisabledRoles(newDisabledRoles);
    }
    const onEnableRoles = (roles: Role[]) => {
        setCurrentDisabledRoles(currentDisabledRoles.filter((role) => !roles.includes(role)));
    }
    const onIncludeAll = () => {
        setCurrentDisabledRoles([]);
    }





    const saveGameMode = () => {
        let name = currentSettingsName;
        if(!name.match(/^[a-zA-Z0-9_ ]+$/) || name.length >= 100 || name.length <= 0) return;
        if(currentRoleList.length === 0) return;
        if(savedGameModes.has(name) && !window.confirm(translate("confirmOverwrite"))) return;




        let newGameMode = new Map(savedGameModes);
        newGameMode.set(name, {
            roleList: currentRoleList,
            phaseTimes: currentPhaseTimes,
            disabledRoles: currentDisabledRoles
        });
        setGameModes(newGameMode);
        saveGameModes(newGameMode);
    }
    const loadGameMode = (settingsName: string) => {
        const gameMode = savedGameModes.get(settingsName);

        setCurrentRoleListName(settingsName);
        setCurrentPhaseTimes(gameMode?.phaseTimes ?? {
            morning: 15,
            discussion: 46,
            voting: 30,
            testimony: 24,
            judgement: 20,
            evening: 10,
            night: 37
        });
        setCurrentDisabledRoles(gameMode?.disabledRoles ?? []);
        setCurrentRoleList(gameMode?.roleList ?? []);
    }
    const deleteGameMode = (roleListName: string) => {
        if(!window.confirm(translate("confirmDelete"))) return;

        let newRoleLists = new Map(savedGameModes);
        newRoleLists.delete(roleListName);
        setGameModes(newRoleLists);
    }



    const exportGameMode = () => {
        //copies current gamemode to clipboard
        navigator.clipboard.writeText(JSON.stringify({
            roleList: currentRoleList,
            phaseTimes: currentPhaseTimes,
            disabledRoles: currentDisabledRoles
        }));
    }
    const importGameMode = () => {
        //imports gamemode from clipboard
        navigator.clipboard.readText().then((text) => {
            try {
                const data = JSON.parse(text);
                if(!data.roleList || !data.phaseTimes || !data.disabledRoles) return;
                setCurrentRoleList(data.roleList);
                setCurrentPhaseTimes(data.phaseTimes);
                setCurrentDisabledRoles(data.disabledRoles);
            } catch (e) {
                console.error(e);
            }
        });
    }


    
    return <div className="game-modes-bank">
        <button className="material-icons-round close-button" onClick={()=>{Anchor.clearCoverCard()}}>
            close
        </button>
        <header>
            <h1>{translate("menu.settings.gameSettingsBank")}</h1>
        </header>
        <main>
            <div>
                <section className="player-list-menu-colors selector-section">
                    <div className="saved-game-modes">
                        {Array.from(savedGameModes.keys()).map((gameModeName) => {
                            return <div key={gameModeName}>
                                <button onClick={()=>{deleteGameMode(gameModeName)}}>{translate("sub")}</button>
                                <button onClick={()=>{loadGameMode(gameModeName)}}>{gameModeName}: {savedGameModes.get(gameModeName)?.roleList.length}</button>
                            </div>
                        })}
                    </div>
                    <div className="save-menu">
                        <input 
                            type="text" 
                            value={currentSettingsName}
                            placeholder={translate("menu.lobby.field.namePlaceholder")}
                            onChange={(e) => {
                            setCurrentRoleListName(e.target.value);
                        }}/>
                        <button onClick={saveGameMode} className="material-icons-round">save</button>
                        <button onClick={exportGameMode}>{translate("exportToClipboard")}</button>
                        <button onClick={importGameMode}>{translate("importFromClipboard")}</button>
                    </div>
                </section>

                <PhaseTimesSelector 
                    phaseTimes={currentPhaseTimes} 
                    onChange={(newPhaseTimes) => {
                        setCurrentPhaseTimes(newPhaseTimes);
                    }}            
                />
            </div>
            <div>
                <OutlineListSelector
                    roleList={currentRoleList}
                    onChangeRolePicker={onChangeRolePicker}
                    onAddNewOutline={addOutline}
                    onRemoveOutline={removeOutline}
                />
                <DisabledRoleSelector
                    onDisableRoles={onDisableRoles}
                    onEnableRoles={onEnableRoles}
                    onIncludeAll={onIncludeAll}
                    disabledRoles={currentDisabledRoles}            
                />
            </div>
        </main>
    </div>
}