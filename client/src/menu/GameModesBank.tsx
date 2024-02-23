import { ReactElement, useState } from "react";
import { SavedGameModes, loadGameModes, saveGameModes } from "../game/localStorage";
import React from "react";
import { OutlineListSelector } from "../components/RolePicker";
import { RoleList, RoleOutline, simplifyRoleOutline } from "../game/roleListState.d";
import translate from "../game/lang";
import "./gameModesBank.css";
import Anchor from "./Anchor";
import PhaseTimesSelector from "../components/PhaseTimePicker";
import { PhaseTimes } from "../game/gameState.d";
import RoleOutlineSelector from "../components/RolePicker";


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
    const [currentExcludedRoles, setCurrentExcludedRoles] = useState<RoleOutline | null>(null);


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





    const saveGameMode = () => {
        let name = currentSettingsName;
        if(!name.match(/^[a-zA-Z0-9_ ]+$/) || name.length >= 100 || name.length <= 0) return;
        if(currentRoleList.length === 0) return;
        if(savedGameModes.has(name) && !window.confirm(translate("confirmOverwrite"))) return;




        let newGameMode = new Map(savedGameModes);
        newGameMode.set(name, {
            roleList: currentRoleList,
            phaseTimes: currentPhaseTimes,
            excludedRoles: currentExcludedRoles
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
        setCurrentExcludedRoles(gameMode?.excludedRoles ?? null);
        setCurrentRoleList(gameMode?.roleList ?? []);
    }
    const deleteGameMode = (roleListName: string) => {
        if(!window.confirm(translate("confirmDelete"))) return;

        let newRoleLists = new Map(savedGameModes);
        newRoleLists.delete(roleListName);
        setGameModes(newRoleLists);
    }



    
    return <div className="game-modes-bank">
        <h1>{translate("menu.settings.gameSettingsBank")}</h1>
        <button className="material-icons-round close-button" onClick={()=>{Anchor.clearCoverCard()}}>close</button>
        {Array.from(savedGameModes.keys()).map((gameModeName) => {
            return <section key={gameModeName}>
                <button onClick={()=>{deleteGameMode(gameModeName)}}>{translate("sub")}</button>
                <button onClick={()=>{loadGameMode(gameModeName)}}>{gameModeName}: {savedGameModes.get(gameModeName)?.roleList.length}</button>
            </section>
        })}

        <div>
            <input type="text" value={currentSettingsName} onChange={(e) => {
                setCurrentRoleListName(e.target.value);
            }}/>
            <button onClick={saveGameMode} className="material-icons-round">save</button>
            
            <PhaseTimesSelector 
                phaseTimes={currentPhaseTimes} 
                onChange={(newPhaseTimes) => {
                    setCurrentPhaseTimes(newPhaseTimes);
                }}            
            />
            <h2>{translate("menu.lobby.roleList")}</h2>
            <OutlineListSelector
                roleList={currentRoleList}
                onChangeRolePicker={onChangeRolePicker}
                onAddNewOutline={addOutline}
                onRemoveOutline={removeOutline}
            />
            <h2>{translate("menu.lobby.excludedRoles")}</h2>
            {
                currentExcludedRoles !== null ? 
                <>
                    <button onClick={()=>{setCurrentExcludedRoles(simplifyRoleOutline(currentExcludedRoles))}}>{translate("simplify")}</button>
                    <RoleOutlineSelector
                        roleOutline={currentExcludedRoles}
                        onChange={setCurrentExcludedRoles}
                    />
                    <button onClick={()=>{setCurrentExcludedRoles(null)}}>{translate("sub")}</button>
                </> 
                :
                <button onClick={()=>{
                    setCurrentExcludedRoles({type: "any"})
                }}>{translate("add")}</button>
            }
            
        </div>
    </div>
}