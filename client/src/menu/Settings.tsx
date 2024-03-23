import React, { ReactElement, useEffect, useState } from 'react';
import "./settings.css";
import translate, { LANGUAGES, Language, languageName, switchLanguage } from '../game/lang';
import GAME_MANAGER from '..';
import Anchor from './Anchor';
import StartMenu from './main/StartMenu';
import LoadingScreen from './LoadingScreen';
import { loadSettings, saveSettings } from '../game/localStorage';
import GameModesEditor from './GameModesEditor';
import { CopyButton } from '../components/ClipboardButtons';
import Wiki from '../components/Wiki';
import { Role } from '../game/roleState.d';
import { StateListener } from '../game/gameManager.d';

export type Settings = {
    volume: number,
    language: Language,
}

type SettingsProps = {
    onVolumeChange: (volume: number) => void
}
type SettingsState = {
    volume: number, // 0-1
    language: Language
}

//default settings
export const DEFAULT_SETTINGS: Settings = {
    volume: 0.5,
    language: "en_us"
}

export default class SettingsMenu extends React.Component<SettingsProps, SettingsState> {
    constructor(props: SettingsProps) {
        super(props);

        this.state = {
            ...DEFAULT_SETTINGS,
            ...loadSettings()
        };
    }
    async quitToMainMenu() {
        GAME_MANAGER.leaveGame();
        Anchor.closeSettings();
        Anchor.clearCoverCard();
        Anchor.setContent(<LoadingScreen type="disconnect"/>)
        await GAME_MANAGER.setDisconnectedState();
        Anchor.setContent(<StartMenu/>)
    }
    goToRolelistEditor() {
        Anchor.setCoverCard(<GameModesEditor/>);
        Anchor.closeSettings();
    }
    render(): React.ReactNode {
        return (
            <div className="settings slide-in">
                {
                    GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game" ? 
                    (<>
                        <h2>{translate("menu.play.field.roomCode")}
                        <RoomCodeButton/></h2>
                    </>
                    ) : null
                }
                <section>
                    <h2>{translate("menu.settings.volume")}</h2>
                    <input className="settings-volume" type="range" min="0" max="1" step="0.01" 
                        value={this.state.volume} 
                        onChange={(e) => {
                            const volume = parseFloat(e.target.value);
                            saveSettings({volume});
                            this.setState({volume}, () => this.props.onVolumeChange(volume));
                        }
                    }/>
                </section>
                <section>
                <h2>{translate("menu.settings.language")}</h2>
                    <select 
                        name="lang-select" 
                        defaultValue={loadSettings().language ?? "en_us"}
                        onChange={e => {
                            const language = e.target.options[e.target.selectedIndex].value as Language;
                            switchLanguage(language);
                            saveSettings({language});
                            Anchor.reloadContent();
                        }}
                    >
                        {LANGUAGES.map(lang => <option key={lang} value={lang}>{languageName(lang)}</option>)}
                    </select>
                </section>
                { GAME_MANAGER.state.stateType !== "disconnected" ? 
                    <button onClick={(e)=>{this.quitToMainMenu()}}>{translate("menu.settings.quitToMenu")}</button>
                : null}
                <button onClick={() => {this.goToRolelistEditor()}}>{translate("menu.settings.gameSettingsEditor")}</button>
                <button onClick={()=>{
                    if(!window.confirm(translate("confirmDelete"))) return;
                    localStorage.clear();
                }}>{translate('menu.settings.eraseSaveData')}</button>
                <button onClick={() => {
                    Anchor.setCoverCard(<WikiCoverCard />);
                    Anchor.closeSettings();
                }}>{translate("menu.wiki.title")}</button>
            </div>
        );
    }
}

function WikiCoverCard(): ReactElement {
    let defaultDisabledRoles: Role[];
    switch (GAME_MANAGER.state.stateType) {
        case "disconnected":
        case "outsideLobby":
            defaultDisabledRoles = [];
        break;
        case "game":
        case "lobby":
            defaultDisabledRoles = GAME_MANAGER.state.excludedRoles;
        break;
    }
    const [disabledRoles, setDisabledRoles] = useState(defaultDisabledRoles);

    useEffect(() => {
        const listener: StateListener = (type) => {
            if (type === "excludedRoles" && (GAME_MANAGER.state.stateType === "game" || GAME_MANAGER.state.stateType === "lobby")) {
                setDisabledRoles(GAME_MANAGER.state.excludedRoles)
            }
        }
        GAME_MANAGER.addStateListener(listener);
        return () => GAME_MANAGER.removeStateListener(listener);
    }, []);

    return <div className='wiki-cover-card'>
        <h1>{translate("menu.wiki.title")}</h1>
        <Wiki disabledRoles={disabledRoles}/>
    </div>
}

export function RoomCodeButton(): JSX.Element {
    let code = new URL(window.location.href);
    
    if (GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game")
        code.searchParams.set("code", GAME_MANAGER.state.roomCode.toString(18));
    
    return <CopyButton text={code.toString()}>
        {
            GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game" ? 
            GAME_MANAGER.state.roomCode.toString(18) : undefined
        }
    </CopyButton>
}