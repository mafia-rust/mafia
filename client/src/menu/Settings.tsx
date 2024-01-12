import React from 'react';
import "./settings.css";
import translate from '../game/lang';
import GAME_MANAGER from '..';

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
        GAME_MANAGER.saveSettings(volume);
        console.log("Loaded settings: " + JSON.stringify(volume));
    }
    render(): React.ReactNode {
        //volume slider
        return (
            <div className="settings slide-in">
                <div className="settingsTitle">
                    <h1>{translate("menu.settings.title")}</h1>
                </div>
                <div className="settingsContent">
                    <h2>{translate("menu.settings.volume")}</h2>
                    <input className="settings-volume" type="range" min="0" max="1" step="0.01" 
                        value={this.props.volume} 
                        onChange={(e) => {
                            console.log("Volume changed to " + e.target.value);
                            let volume = parseFloat(e.target.value);
                            this.props.onVolumeChange(volume);
                        }
                    }/>
                    {
                        GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game" ? 
                        (<>
                            <h2>{translate("menu.play.field.roomCode")}</h2>
                            <RoomCodeButton/>
                        </>) : null
                    }
                </div>
            </div>
        );
    }
}

export function RoomCodeButton(props: {}): JSX.Element {
    return <button onClick={() => {
        let code = new URL(window.location.href);
        
        if (GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game")
            code.searchParams.set("code", GAME_MANAGER.state.roomCode!);

        if (navigator.clipboard)
            navigator.clipboard.writeText(code.toString());
    }}>
        {
            GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game" ? 
            GAME_MANAGER.state.roomCode : ""
        }
    </button>
}