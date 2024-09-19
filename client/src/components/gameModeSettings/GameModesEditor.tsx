import { ReactElement, createContext, useCallback, useState } from "react";
import React from "react";
import { OutlineListSelector } from "./OutlineSelector";
import { getAllRoles, RoleList, RoleOutline } from "../../game/roleListState.d";
import translate from "../../game/lang";
import "./gameModesEditor.css";
import PhaseTimesSelector from "./PhaseTimeSelector";
import { PhaseTimes } from "../../game/gameState.d";
import EnabledRoleSelector from "./EnabledRoleSelector";
import { Role } from "../../game/roleState.d";
import "./selectorSection.css";
import { defaultPhaseTimes } from "../../game/gameState";
import { GameModeSelector } from "./GameModeSelector";
import { ShareableGameMode } from "./gameMode";

const GameModeContext = createContext({
    roleList: [] as RoleList,
    phaseTimes: defaultPhaseTimes(),
    enabledRoles: [] as Role[]
});
export {GameModeContext};


export default function GameModesEditor(props: Readonly<{
    initialGameMode?: ShareableGameMode
}>): ReactElement {

    const [roleList, setRoleList] = useState<RoleList>(()=>{
        if(props.initialGameMode){
            return props.initialGameMode.roleList;
        }
        return [];
    });
    const [phaseTimes, setPhaseTimes] = useState<PhaseTimes>(()=>{
        if(props.initialGameMode){
            return props.initialGameMode.phaseTimes;
        }
        return defaultPhaseTimes()
    });
    const [enabledRoles, setEnabledRoles] = useState<Role[]>(()=>{
        if(props.initialGameMode){
            return props.initialGameMode.enabledRoles;
        }
        return [];
    });


    const onChangeRolePicker = useCallback((value: RoleOutline, index: number) => {
        const newRoleList = [...roleList];
        newRoleList[index] = value;
        setRoleList(newRoleList);
    }, [roleList]);
    
    const addOutline = () => {
        setRoleList([...roleList, {type: "any"}]);
    }
    const removeOutline = (index: number) => {
        let newRoleList = [...roleList];
        newRoleList.splice(index, 1);
        setRoleList(newRoleList);
    }


    const onEnableRoles = (roles: Role[]) => {
        const newEnabledRoles = [...enabledRoles];
        for(const role of roles){
            if(!newEnabledRoles.includes(role)){
                newEnabledRoles.push(role);
            }
        }
        setEnabledRoles(newEnabledRoles);
    }
    const onDisableRoles = (roles: Role[]) => {
        setEnabledRoles(enabledRoles.filter((role) => !roles.includes(role)));
    }
    const onEnableAll = () => {
        setEnabledRoles(getAllRoles());
    }
    
    return <div className="game-modes-editor">
        <header>
            <h1>{translate("menu.globalMenu.gameSettingsEditor")}</h1>
        </header>
        <GameModeContext.Provider value={{roleList, phaseTimes, enabledRoles}}>
            <main>
                <div>
                    <GameModeSelector 
                        canModifySavedGameModes={true}
                        loadGameMode={gameMode => {
                            setRoleList(gameMode.roleList);
                            setEnabledRoles(gameMode.enabledRoles);
                            setPhaseTimes(gameMode.phaseTimes);
                        }}
                    />
                    <PhaseTimesSelector 
                        onChange={(newPhaseTimes) => {
                            setPhaseTimes(newPhaseTimes);
                        }}            
                    />
                </div>
                <div>
                    <OutlineListSelector
                        onChangeRolePicker={onChangeRolePicker}
                        onAddNewOutline={addOutline}
                        onRemoveOutline={removeOutline}
                        setRoleList={setRoleList}
                    />
                    <EnabledRoleSelector
                        onDisableRoles={onDisableRoles}
                        onEnableRoles={onEnableRoles}
                        onIncludeAll={onEnableAll}         
                    />
                </div>
            </main>
        </GameModeContext.Provider>
    </div>
}
