import React, { ReactElement, useCallback, useContext, useEffect, useState } from "react";
import { SavedGameModes, loadGameModes, saveGameModes } from "../game/localStorage";
import Anchor from "../menu/Anchor";
import { CopyButton, PasteButton } from "../components/ClipboardButtons";
import Icon from "./Icon";
import { Button } from "./Button";
import { DragAndDrop } from "./DragAndDrop";
import { PhaseTimes } from "../game/gameState.d";
import { Role } from "../game/roleState.d";
import { RoleList } from "../game/roleListState.d";
import { GameModeContext } from "./GameModesEditor";
import translate from "../game/lang";
import { defaultPhaseTimes } from "../game/gameState";
import "./gameModeSelector.css"

export function GameModeSelector(props: {
    canModifySavedGameModes?: boolean
    setPhaseTimes: (phaseTimes: PhaseTimes) => void, 
    setDisabledRoles: (disabledRoles: Role[]) => void, 
    setRoleList: (roleList: RoleList) => void
}): ReactElement {
    const [savedGameModes, setGameModes] = useState<SavedGameModes>(loadGameModes() ?? {});
    const [gameModeName, setGameModeName] = useState<string>("");
    const {roleList, phaseTimes, disabledRoles} = useContext(GameModeContext);
    
    const saveGameMode = useCallback(() => {
        const name = gameModeName;
        if(!name.match(/^[a-zA-Z0-9_ ]+$/) || name.length >= 100 || name.length <= 0) return "invalidName";
        if(roleList.length === 0) return "noRoles";
        if(savedGameModes[name] !== undefined && !window.confirm(translate("confirmOverwrite"))) return "didNotConfirm";

        const newGameModes = {...savedGameModes};
        newGameModes[name] = {
            name: gameModeName,
            roleList,
            phaseTimes,
            disabledRoles
        };
        setGameModes(newGameModes);
        saveGameModes(newGameModes);
        return "success";
    }, [disabledRoles, gameModeName, phaseTimes, roleList, savedGameModes]);

    useEffect(() => {
        const listener = (e: KeyboardEvent) => {
            if (e.ctrlKey && e.key === 's') {
                e.preventDefault();

                const result = saveGameMode();
                if (result !== "success") {
                    Anchor.pushError(translate("notification.saveGameMode.failure"), translate("notification.saveGameMode.failure." + result));
                }
            }
        }
        document.addEventListener('keydown', listener);
        return () => document.removeEventListener('keydown', listener);
    }, [saveGameMode]);

    const loadGameMode = (gameModeName: string) => {
        const gameMode = savedGameModes[gameModeName];

        setGameModeName(gameModeName);
        props.setPhaseTimes(gameMode?.phaseTimes ?? defaultPhaseTimes());
        props.setDisabledRoles(gameMode?.disabledRoles ?? []);
        props.setRoleList(gameMode?.roleList ?? []);
        return true;
    }

    const deleteGameMode = (roleListName: string) => {
        if(!window.confirm(translate("confirmDelete"))) return false;

        const newGameModes = {...savedGameModes};
        delete newGameModes[roleListName];
        setGameModes(newGameModes);
        saveGameModes(newGameModes);
        return true;
    }

    const verbose = !props.canModifySavedGameModes;

    return <section className="chat-menu-colors selector-section">
        <h2>{translate("menu.lobby.gameModes")}</h2>
        <div className="save-menu">
            {props.canModifySavedGameModes && <>
                <input 
                    type="text" 
                    value={gameModeName}
                    placeholder={translate("menu.lobby.field.namePlaceholder")}
                    onChange={(e) => setGameModeName(e.target.value)}
                />
                <Button 
                    onClick={saveGameMode}
                    pressedChildren={result => <Icon>{result === "success" ? "done" : "warning"}</Icon>}
                    pressedText={result => {
                        if (result === "success") {
                            return translate("notification.saveGameMode.success");
                        } else {
                            return translate("notification.saveGameMode.failure." + result)
                        }
                    }}
                >
                    <Icon>save</Icon>
                </Button>
            </>}
            <CopyButton text={JSON.stringify({
                name: gameModeName === "" ? "Unnamed Game Mode" : gameModeName,
                roleList,
                phaseTimes,
                disabledRoles
            })}>{verbose ? <><Icon>content_copy</Icon> {translate("copyToClipboard")}</> : undefined}</CopyButton>
            <PasteButton 
                onClipboardRead={text => {
                    try {
                        const data = JSON.parse(text);

                        setGameModeName(data.name ?? "")
                        props.setRoleList(data.roleList ?? []);
                        props.setPhaseTimes(data.phaseTimes ?? defaultPhaseTimes());
                        props.setDisabledRoles(data.disabledRoles ?? []);
                        return "success";
                    } catch (e) {
                        return "invalidData";
                    }
                }}
                failureText={() => translate("notification.importGameMode.failure")}
            >{verbose ? <><Icon>paste</Icon> {translate("importFromClipboard")}</> : undefined}</PasteButton>
        </div>
        <div className="saved-game-modes">
            {props.canModifySavedGameModes
                ? <DragAndDrop
                    items={Object.keys(savedGameModes)}
                    onDragEnd={newItems => {
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
                    render={gameModeName => <GameModeLabel 
                        gameModeName={gameModeName} 
                        numberOfPlayers={savedGameModes[gameModeName]?.roleList.length}
                        modifiable={props.canModifySavedGameModes}
                        loadGameMode={loadGameMode}
                        deleteGameMode={deleteGameMode}
                    />}
                />
                : Object.keys(savedGameModes).map(gameModeName => <div>
                    <GameModeLabel 
                        gameModeName={gameModeName} 
                        numberOfPlayers={savedGameModes[gameModeName]?.roleList.length}
                        modifiable={props.canModifySavedGameModes}
                        loadGameMode={loadGameMode}
                        deleteGameMode={deleteGameMode}
                    />
                </div>)
            }
            
        </div>
    </section>
}

function GameModeLabel(props: { 
    gameModeName: string, 
    numberOfPlayers: number, 
    loadGameMode: (name: string) => boolean, 
} & (
    {
        modifiable: true,
        deleteGameMode: (name: string) => boolean
    } | {
        modifiable?: false
    }
)): ReactElement {
    return <>
        {props.modifiable && <Icon>drag_indicator</Icon>}
        <span className="game-mode-name">{props.gameModeName}: {props.numberOfPlayers}</span>
        <div>
            <Button 
                onClick={() => props.loadGameMode(props.gameModeName)}
                pressedChildren={result => <Icon>{result ? "done" : "warning"}</Icon>}
            ><Icon>{props.modifiable ? "edit" : "launch"}</Icon></Button>
            {props.modifiable && <Button 
                onClick={() => props.deleteGameMode(props.gameModeName)}
                pressedChildren={result => <Icon>{result ? "done" : "warning"}</Icon>}
            ><Icon>delete</Icon></Button>}
        </div>
    </>
}