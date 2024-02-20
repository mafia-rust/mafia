import React from 'react';
import "./settings.css";
import translate, { LANGUAGES, Language, languageName, switchLanguage } from '../game/lang';
import GAME_MANAGER from '..';
import Anchor from './Anchor';
import StartMenu from './main/StartMenu';
import LoadingScreen from './LoadingScreen';
import { loadSettings, saveSettings } from '../game/localStorage';
import GameModesEditor from './GameModesEditor';

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
    componentDidMount() {
    }
    componentWillUnmount() {
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
                <div>
                    {
                        GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game" ? 
                        (<>
                            <h2>{translate("menu.play.field.roomCode")}
                            <RoomCodeButton/></h2>
                        </>
                        ) : null
                    }
                    <h2>{translate("menu.settings.volume")}
                    <input className="settings-volume" type="range" min="0" max="1" step="0.01" 
                        value={this.state.volume} 
                        onChange={(e) => {
                            const volume = parseFloat(e.target.value);
                            saveSettings({volume});
                            this.setState({volume}, () => this.props.onVolumeChange(volume));
                        }
                    }/></h2>
                    <h2>{translate("menu.settings.language")}
                        <select 
                            name="lang-select" 
                            defaultValue={loadSettings()?.language ?? "en_us"}
                            onChange={e => {
                                const language = e.target.options[e.target.selectedIndex].value;
                                switchLanguage(language);
                                saveSettings({language});
                                window.location.reload();
                            }}
                        >
                            {LANGUAGES.map(lang => <option value={lang}>{languageName(lang)}</option>)}
                        </select>
                    </h2>
                    <button onClick={(e)=>{this.quitToMainMenu()}}>{translate("menu.settings.quitToMenu")}</button>
                    <button onClick={() => {this.goToRolelistEditor()}}>{translate("menu.settings.gameSettingsEditor")}</button>
                    <button onClick={()=>{
                        if(!window.confirm(translate("confirmDelete"))) return;
                        localStorage.clear();
                    }}>{translate('menu.settings.eraseSaveData')}</button>
                </div>
            </div>
        );
    }
}

export function RoomCodeButton(props: {}): JSX.Element {
    return <button onClick={() => {
        let code = new URL(window.location.href);
        
        if (GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game")
            code.searchParams.set("code", GAME_MANAGER.state.roomCode.toString(18));

        if (navigator.clipboard)
            navigator.clipboard.writeText(code.toString());
    }}>
        {
            GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game" ? 
            GAME_MANAGER.state.roomCode.toString(18) : ""
        }
    </button>
}