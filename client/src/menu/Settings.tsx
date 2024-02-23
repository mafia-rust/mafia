import React from 'react';
import "./settings.css";
import translate from '../game/lang';
import GAME_MANAGER from '..';
import Anchor from './Anchor';
import StartMenu from './main/StartMenu';
import LoadingScreen from './LoadingScreen';
import { saveSettings } from '../game/localStorage';
import GameModesEditor from './GameModesEditor';

type SettingsProps = {
    volume: number, // 0-1
    onVolumeChange: (volume: number) => void
}
type SettingsState = {
}

//default settings
export const DEFAULT_SETTINGS = {
    volume: 0.5
}

export default class Settings extends React.Component<SettingsProps, SettingsState> {
    constructor(props: SettingsProps) {
        super(props);

        this.state = {
        };
    }
    componentDidMount() {
    }
    componentWillUnmount() {
    }
    saveSettings(volume: number) {
        saveSettings(volume);
        console.log("Loaded settings: " + JSON.stringify(volume));
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
                
                <h2>{translate("menu.settings.volume")}
                <input className="settings-volume" type="range" min="0" max="1" step="0.01" 
                    value={this.props.volume} 
                    onChange={(e) => {
                        let volume = parseFloat(e.target.value);
                        this.props.onVolumeChange(volume);
                    }
                }/></h2>
                <button onClick={(e)=>{this.quitToMainMenu()}}>{translate("menu.settings.quitToMenu")}</button>
                <button onClick={() => {this.goToRolelistEditor()}}>{translate("menu.settings.gameSettingsEditor")}</button>
                <button onClick={()=>{
                    if(!window.confirm(translate("confirmDelete"))) return;
                    localStorage.clear();
                }}>{translate('menu.settings.eraseSaveData')}</button>
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