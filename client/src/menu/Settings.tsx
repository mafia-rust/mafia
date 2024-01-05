import React from 'react';
import "./settings.css";

type SettingsProps = {
    volume: number, // 0-1
    onVolumeChange: (volume: number) => void
}
type SettingsState = {
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
    render(): React.ReactNode {
        //volume slider
        return (
            <div className="settings slide-in">
                <div >
                    <div className="settingsTitle">
                        <h1>NO LANG YET</h1>
                        <h1>Settings</h1>
                    </div>
                    <div className="settingsContent">
                        <div className="settingsSection">
                            <h2>Volume</h2>
                            <input type="range" min="0" max="1" step="0.01" value={this.props.volume} onChange={(e) => {
                                let volume = parseFloat(e.target.value);
                                this.props.onVolumeChange(volume);
                            }}/>
                        </div>
                    </div>
                </div>
            </div>
        );
    }

}