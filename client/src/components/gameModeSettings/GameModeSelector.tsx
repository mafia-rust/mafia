import React, { ReactElement, useCallback, useContext, useEffect, useState } from "react";
import { GameMode, GameModeData, SavedGameModes, deleteGameModes, loadGameModes, saveGameModes } from "../../game/localStorage";
import Anchor from "../../menu/Anchor";
import { CopyButton, PasteButton } from "../../components/ClipboardButtons";
import Icon from "../Icon";
import { Button } from "../Button";
import { DragAndDrop } from "../DragAndDrop";
import { GameModeContext } from "./GameModesEditor";
import translate from "../../game/lang";
import "./gameModeSelector.css"
import parseShareableGameModeData, { parseGameModeStorage } from "./parse";

type GameModeLocation = {
    name: string,
    players: number
}

export function GameModeSelector(props: {
    canModifySavedGameModes?: boolean,
    loadGameMode: (gameMode: GameModeData) => void,
}): ReactElement {
    const [gameModeParseResult, setGameModeParseResult] = useState(parseGameModeStorage(loadGameModes()));

    return <section className="chat-menu-colors selector-section">
        <h2>{translate("menu.lobby.gameModes")}</h2>
        {gameModeParseResult.type === "failure" 
            ? <div>
                <div>{translate("outdatedGameModeSaveData")}</div>
                <button onClick={() => {
                    deleteGameModes();
                    setGameModeParseResult(parseGameModeStorage(loadGameModes()));
                }}>
                    <Icon>delete</Icon>{translate("deleteOutdatedGameModeSaveData")}
                </button>
            </div>
            : <GameModeSelectorPanel {...props} 
                gameModeStorage={gameModeParseResult.value}
                reloadGameModeStorage={() => setGameModeParseResult(parseGameModeStorage(loadGameModes()))}
            />
        }
    </section>
}

function GameModeSelectorPanel(props: {
    canModifySavedGameModes?: boolean,
    gameModeStorage: SavedGameModes,
    reloadGameModeStorage: () => void,
    loadGameMode: (gameMode: GameModeData) => void,
}): ReactElement {
    const [gameModeNameField, setGameModeNameField] = useState<string>("");
    const {roleList, phaseTimes, disabledRoles} = useContext(GameModeContext);

    const validateName = (name: string) => {
        return name.length < 100 && name.length !== 0
    }
    
    const saveGameMode = useCallback((name: string) => {
        if(roleList.length === 0) return "noRoles";

        const newGameModeStorage: SavedGameModes = JSON.parse(JSON.stringify(props.gameModeStorage));

        const gameMode = newGameModeStorage.find(gameMode => gameMode.name === name);

        if (gameMode === undefined) {
            if (validateName(name)) {
                newGameModeStorage.push({
                    name,
                    data: { [roleList.length]: { disabledRoles, phaseTimes, roleList } }
                })
            } else {
                return "invalidName";
            }
        } else {
            if (Object.keys(gameMode.data).includes("" + roleList.length) && !window.confirm(translate("confirmOverwrite"))) {
                return "didNotConfirm";
            }

            gameMode.data[roleList.length] = {
                roleList,
                phaseTimes,
                disabledRoles
            }
        }

        saveGameModes(newGameModeStorage);
        props.reloadGameModeStorage();
        return "success";
    }, [disabledRoles, props, phaseTimes, roleList]);

    useEffect(() => {
        const listener = (e: KeyboardEvent) => {
            if (e.ctrlKey && e.key === 's') {
                e.preventDefault();

                const result = saveGameMode(gameModeNameField);

                if (result !== "success") {
                    Anchor.pushError(translate("notification.saveGameMode.failure"), translate("notification.saveGameMode.failure." + result));
                }
            }
        }
        document.addEventListener('keydown', listener);
        return () => document.removeEventListener('keydown', listener);
    }, [gameModeNameField, saveGameMode]);

    // Caller must ensure location is valid
    const loadGameMode = (location: GameModeLocation) => {
        const gameMode = props.gameModeStorage.find(gameMode => gameMode.name === location.name)!;

        setGameModeNameField(gameMode.name)
        props.loadGameMode(gameMode.data[location.players]);

        return true;
    }

    // Caller must ensure location is valid
    const deleteGameMode = (location: GameModeLocation) => {
        if(!window.confirm(translate("confirmDelete"))) return false;
        
        const newGameModeStorage: SavedGameModes = JSON.parse(JSON.stringify(props.gameModeStorage));

        const gameModeIndex = newGameModeStorage.findIndex(gameMode => gameMode.name === location.name);
        const gameMode = newGameModeStorage[gameModeIndex];

        delete gameMode.data[location.players];

        if (Object.keys(gameMode.data).length === 0) {
            newGameModeStorage.splice(gameModeIndex, 1);
        }

        saveGameModes(newGameModeStorage);
        props.reloadGameModeStorage();
        return true;
    }

    const verbose = !props.canModifySavedGameModes;

    return <>
        <div className="save-menu">
            {props.canModifySavedGameModes && <>
                <input 
                    type="text" 
                    value={gameModeNameField}
                    placeholder={translate("menu.lobby.field.namePlaceholder")}
                    onChange={(e) => setGameModeNameField(e.target.value)}
                />
            </>}
            <div>
                {props.canModifySavedGameModes && <>
                    <Button 
                        onClick={() => saveGameMode(gameModeNameField)}
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
                    name: gameModeNameField === "" ? "Unnamed Game Mode" : gameModeNameField,
                    roleList,
                    phaseTimes,
                    disabledRoles
                })}>{verbose ? <><Icon>content_copy</Icon> {translate("copyToClipboard")}</> : undefined}</CopyButton>
                <PasteButton 
                    onClipboardRead={text => {
                        const parsedGameMode = parseShareableGameModeData(text);
                        if (parsedGameMode.type === "success") {
                            if (parsedGameMode.value.name !== undefined) {
                                setGameModeNameField(parsedGameMode.value.name);
                            }
                            props.loadGameMode({
                                roleList: parsedGameMode.value.roleList,
                                phaseTimes: parsedGameMode.value.phaseTimes,
                                disabledRoles: parsedGameMode.value.disabledRoles
                            })
                        } else {
                            return "invalidData";
                        }
                    }}
                    failureText={() => translate("notification.importGameMode.failure")}
                >{verbose ? <><Icon>paste</Icon> {translate("importFromClipboard")}</> : undefined}</PasteButton>
                <button onClick={props.reloadGameModeStorage}>{translate("refresh")}</button>
            </div>
        </div>
        <div className="saved-game-modes">
            {props.canModifySavedGameModes
                ? <DragAndDrop
                    items={props.gameModeStorage}
                    onDragEnd={newItems => {
                        const newGameModeStorage: SavedGameModes = [...props.gameModeStorage];
                        
                        newGameModeStorage.sort((a, b) => newItems.indexOf(a) - newItems.indexOf(b))
                        
                        saveGameModes(newGameModeStorage);
                        props.reloadGameModeStorage();
                    }}
                    render={gameMode => 
                        <GameModeLabel
                            gameMode={gameMode}
                            modifiable={props.canModifySavedGameModes ?? true}
                            gameModeStorage={props.gameModeStorage}
                            loadGameMode={loadGameMode}
                            deleteGameMode={deleteGameMode}
                        />
                    }
                />
                : props.gameModeStorage.map((gameMode, index) => <div key={index}>
                    <GameModeLabel
                        gameMode={gameMode}
                        modifiable={props.canModifySavedGameModes ?? true}
                        gameModeStorage={props.gameModeStorage}
                        loadGameMode={loadGameMode}
                        deleteGameMode={deleteGameMode}
                    />
                </div>)
            }
            
        </div>
    </>
}

function GameModeLabel(props: { 
    gameMode: GameMode,
    modifiable: boolean,
    gameModeStorage: SavedGameModes,
    loadGameMode: (location: GameModeLocation) => boolean, 
    deleteGameMode: (location: GameModeLocation) => boolean
}): ReactElement {
    if (Object.keys(props.gameMode.data).length === 1) {
        return <GameModeSingleLabel 
            location={{ name: props.gameMode.name, players: parseInt(Object.keys(props.gameMode.data)[0]) }}
            modifiable={props.modifiable}
            gameModeStorage={props.gameModeStorage}
            loadGameMode={props.loadGameMode}
            deleteGameMode={props.deleteGameMode}
        />
    } else {
        return <GameModeFolderLabel
            gameModeName={props.gameMode.name}
            modifiable={props.modifiable}
            gameModeStorage={props.gameModeStorage}
            loadGameMode={props.loadGameMode}
            deleteGameMode={props.deleteGameMode}
        />
    }
}

function GameModeFolderLabel(props: {
    gameModeName: string,
    modifiable: boolean,
    gameModeStorage: SavedGameModes,
    loadGameMode: (location: GameModeLocation) => boolean,
    deleteGameMode: (location: GameModeLocation) => boolean,
}): ReactElement {
    const [expanded, setExpanded] = useState<boolean>(false);

    useEffect(() => {
        setExpanded(false)
    }, [props.gameModeName])
    
    const gameMode = props.gameModeStorage.find(gameMode => gameMode.name === props.gameModeName)!

    return <div className="game-mode-label">
        {props.modifiable && <Icon>drag_indicator</Icon>}
        <div className="game-mode-folder">
            <div className="game-mode-folder-header">
                <span className="game-mode-name">{props.gameModeName}</span>
                <div>
                    <Button
                        onClick={() => setExpanded(!expanded)}
                    ><Icon>{expanded ? "expand_less" : "expand_more"}</Icon></Button>
                </div>
            </div>
            {expanded && <div className="game-mode-folder-content">
                {Object.keys(gameMode.data).map(key => <GameModeSingleLabel
                    location={{ name: props.gameModeName, players: parseInt(key) }}
                    gameModeStorage={props.gameModeStorage}
                    modifiable={props.modifiable}
                    draggable={false}
                    loadGameMode={props.loadGameMode}
                    deleteGameMode={props.deleteGameMode}
                />)}
            </div>}
        </div>
    </div>
}

function GameModeSingleLabel(props: { 
    location: GameModeLocation, 
    gameModeStorage: SavedGameModes,
    loadGameMode: (location: GameModeLocation) => boolean, 
} & (
    {
        modifiable: true,
        draggable?: boolean,
        deleteGameMode: (location: GameModeLocation) => boolean
    } | {
        modifiable?: false
    }
)): ReactElement {
    return <div className="game-mode-label">
        {props.modifiable && (props.draggable ?? true) && <Icon>drag_indicator</Icon>}
        <span className="game-mode-name">{props.location.name}: {props.location.players}</span>
        <div className="game-mode-label-buttons">
            <Button 
                onClick={() => props.loadGameMode(props.location)}
                pressedChildren={result => <Icon>{result ? "done" : "warning"}</Icon>}
            ><Icon>{props.modifiable ? "edit" : "launch"}</Icon></Button>
            {props.modifiable && <Button 
                onClick={() => props.deleteGameMode(props.location)}
                pressedChildren={result => <Icon>{result ? "done" : "warning"}</Icon>}
            ><Icon>delete</Icon></Button>}
        </div>
    </div>
}