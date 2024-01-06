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
                    <div className="settingsSection">
                        <h2>{translate("menu.settings.volume")}</h2>
                        <input type="range" min="0" max="1" step="0.01" 
                            value={this.props.volume} 
                            onChange={(e) => {
                                console.log("Volume changed to " + e.target.value);
                                let volume = parseFloat(e.target.value);
                                this.props.onVolumeChange(volume);
                            }
                        }/>
                    </div>
                </div>
            </div>
        );
    }

}