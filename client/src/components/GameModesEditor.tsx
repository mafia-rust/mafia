import { ReactElement, createContext, useCallback, useState } from "react";
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
import { GameModeSelector } from "./GameModeSelector";

const GameModeContext = createContext({
    roleList: [] as RoleList,
    phaseTimes: defaultPhaseTimes(),
    disabledRoles: [] as Role[]
});
export {GameModeContext};


export default function GameModesEditor(): ReactElement {
    const [roleList, setRoleList] = useState<RoleList>([]);
    const [phaseTimes, setPhaseTimes] = useState<PhaseTimes>(defaultPhaseTimes());
    const [disabledRoles, setDisabledRoles] = useState<Role[]>([]);


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


    const onDisableRoles = (roles: Role[]) => {
        const newDisabledRoles = [...disabledRoles];
        for(const role of roles){
            if(!newDisabledRoles.includes(role)){
                newDisabledRoles.push(role);
            }
        }
        setDisabledRoles(newDisabledRoles);
    }
    const onEnableRoles = (roles: Role[]) => {
        setDisabledRoles(disabledRoles.filter((role) => !roles.includes(role)));
    }
    const onIncludeAll = () => {
        setDisabledRoles([]);
    }
    
    return <div className="game-modes-editor">
        <header>
            <h1>{translate("menu.settings.gameSettingsEditor")}</h1>
        </header>
        <GameModeContext.Provider value={{roleList, phaseTimes, disabledRoles}}>
            <main>
                <div>
                    <GameModeSelector 
                        canModifySavedGameModes={true}
                        setRoleList={setRoleList}
                        setDisabledRoles={setDisabledRoles}
                        setPhaseTimes={setPhaseTimes}
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
                    <DisabledRoleSelector
                        onDisableRoles={onDisableRoles}
                        onEnableRoles={onEnableRoles}
                        onIncludeAll={onIncludeAll}         
                    />
                </div>
            </main>
        </GameModeContext.Provider>
    </div>
}
