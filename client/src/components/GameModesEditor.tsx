import { ReactElement, createContext, useContext, useState } from "react";
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
import Icon from "./Icon";
import { Button } from "./Button";
import { DragAndDrop } from "./DragAndDrop";

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


    const onChangeRolePicker = (value: RoleOutline, index: number) => {
        let newRoleList = [...roleList];
        newRoleList[index] = value;
        setRoleList(newRoleList);
    }
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

function GameModeSelector(props: { 
    setPhaseTimes: (phaseTimes: PhaseTimes) => void, 
    setDisabledRoles: (disabledRoles: Role[]) => void, 
    setRoleList: (roleList: RoleList) => void
}): ReactElement {
    const [savedGameModes, setGameModes] = useState<SavedGameModes>(loadGameModes() ?? {});
    const [gameModeName, setGameModeName] = useState<string>("");
    const {roleList, phaseTimes, disabledRoles} = useContext(GameModeContext);
    
    const saveGameMode = () => {
        const name = gameModeName;
        if(!name.match(/^[a-zA-Z0-9_ ]+$/) || name.length >= 100 || name.length <= 0) return false;
        if(roleList.length === 0) return false;
        if(savedGameModes[name] !== undefined && !window.confirm(translate("confirmOverwrite"))) return false;

        let newGameModes = {...savedGameModes};
        newGameModes[name] = {
            name: gameModeName,
            roleList,
            phaseTimes,
            disabledRoles
        };
        setGameModes(newGameModes);
        saveGameModes(newGameModes);
        return true;
    }
    const loadGameMode = (gameModeName: string) => {
        const gameMode = savedGameModes[gameModeName];

        setGameModeName(gameModeName);
        props.setPhaseTimes(gameMode?.phaseTimes ?? defaultPhaseTimes());
        props.setDisabledRoles(gameMode?.disabledRoles ?? []);
        props.setRoleList(gameMode?.roleList ?? []);
    }
    const deleteGameMode = (roleListName: string) => {
        if(!window.confirm(translate("confirmDelete"))) return false;

        const newGameModes = {...savedGameModes};
        delete newGameModes[roleListName];
        setGameModes(newGameModes);
        saveGameModes(newGameModes);
        return true;
    }

    return <section className="player-list-menu-colors selector-section">
        <div className="saved-game-modes">
            <DragAndDrop<string> 
                items={Object.keys(savedGameModes)}
                onDragEnd={(newItems: string[]) => {
                    // Reordering object keys: https://stackoverflow.com/a/31102605/9157590
                    const oldOrder = {...savedGameModes};
                    
                    const newOrder: SavedGameModes = Object.keys(oldOrder)
                        .sort((a, b) => newItems.indexOf(a) - newItems.indexOf(b))
                        .reduce(
                            (obj, key) => { 
                                obj[key] = oldOrder[key]; 
                                return obj;
                            }, 
                            {} as SavedGameModes
                        );
                    
                    setGameModes(newOrder);
                    saveGameModes(newOrder);
                }}
                render={gameModeName => <>
                    <Icon>drag_indicator</Icon>
                    <span>{gameModeName}: {savedGameModes[gameModeName]?.roleList.length}</span>
                    <div>
                        <Button 
                            onClick={()=>{
                                loadGameMode(gameModeName)
                                return true;
                            }}
                        ><Icon>edit</Icon></Button>
                        <Button 
                            onClick={()=>deleteGameMode(gameModeName)}
                        ><Icon>delete</Icon></Button>
                    </div>
                </>}
            />
        </div>
        <div className="save-menu">
            <input 
                type="text" 
                value={gameModeName}
                placeholder={translate("menu.lobby.field.namePlaceholder")}
                onChange={(e) => {
                setGameModeName(e.target.value);
            }}/>
            <Button 
                onClick={saveGameMode}
                successText={translate("notification.saveGameMode.success")}
                failureText={translate("notification.saveGameMode.failure")}
            >
                <Icon>save</Icon>
            </Button>
            <CopyButton text={JSON.stringify({
                name: gameModeName==="" ? "Unnamed Game Mode" : gameModeName,
                roleList,
                phaseTimes,
                disabledRoles
            })}/>
            <PasteButton onPasteSuccessful={text => {
                try {
                    const data = JSON.parse(text);

                    setGameModeName(data.name ?? "")
                    props.setRoleList(data.roleList ?? []);
                    props.setPhaseTimes(data.phaseTimes ?? defaultPhaseTimes());
                    props.setDisabledRoles(data.disabledRoles ?? []);
                    return true;
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
}