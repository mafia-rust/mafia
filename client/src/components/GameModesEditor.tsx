import { ReactElement, useState } from "react";
import { SavedGameModes, loadGameModes, saveGameModes } from "../game/localStorage";
import React from "react";
import { OutlineListSelector } from "../components/OutlineSelector";
import { RoleList, RoleOutline } from "../game/roleListState.d";
import translate from "../game/lang";
import "./gameModesEditor.css";
import PhaseTimesSelector from "../components/PhaseTimeSelector";
import { PhaseTimes } from "../game/gameState.d";
import DisabledRoleSelector from "../components/DisabledRoleSelector";
import { Role } from "../game/roleState.d";
import "../components/selectorSection.css";
import { defaultPhaseTimes } from "../game/gameState";
import Anchor from "../menu/Anchor";
import { CopyButton, PasteButton } from "../components/ClipboardButtons";


export default function GameModesEditor(): ReactElement {

    const [savedGameModes, setGameModes] = useState<SavedGameModes>(loadGameModes() ?? new Map());

    const [currentSettingsName, setCurrentRoleListName] = useState<string>("");

    const [currentRoleList, setCurrentRoleList] = useState<RoleList>([]);
    const [currentPhaseTimes, setCurrentPhaseTimes] = useState<PhaseTimes>(defaultPhaseTimes());
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




        let newGameModes = new Map(savedGameModes);
        newGameModes.set(name, {
            name: currentSettingsName,
            roleList: currentRoleList,
            phaseTimes: currentPhaseTimes,
            disabledRoles: currentDisabledRoles
        });
        setGameModes(newGameModes);
        saveGameModes(newGameModes);
    }
    const loadGameMode = (settingsName: string) => {
        const gameMode = savedGameModes.get(settingsName);

        setCurrentRoleListName(settingsName);
        setCurrentPhaseTimes(gameMode?.phaseTimes ?? defaultPhaseTimes());
        setCurrentDisabledRoles(gameMode?.disabledRoles ?? []);
        setCurrentRoleList(gameMode?.roleList ?? []);
    }
    const deleteGameMode = (roleListName: string) => {
        if(!window.confirm(translate("confirmDelete"))) return;

        let newGameModes = new Map(savedGameModes);
        newGameModes.delete(roleListName);
        setGameModes(newGameModes);
        saveGameModes(newGameModes);
    }
    
    return <div className="game-modes-editor">
        <header>
            <h1>{translate("menu.settings.gameSettingsEditor")}</h1>
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
                        <CopyButton text={JSON.stringify({
                            name: currentSettingsName==="" ? "Unnamed Game Mode" : currentSettingsName,
                            roleList: currentRoleList,
                            phaseTimes: currentPhaseTimes,
                            disabledRoles: currentDisabledRoles
                        })}/>
                        <PasteButton onPasteSuccessful={text => {
                            try {
                                const data = JSON.parse(text);

                                setCurrentRoleListName(data.name ?? "")
                                setCurrentRoleList(data.roleList ?? []);
                                setCurrentPhaseTimes(data.phaseTimes ?? defaultPhaseTimes());
                                setCurrentDisabledRoles(data.disabledRoles ?? []);
                            } catch (e) {
                                Anchor.pushError(
                                    translate("notification.importGameMode.failure"), 
                                    translate("notification.importGameMode.failure.details")
                                );
                                return false;
                            }}
                        }/>
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