import React, { JSXElementConstructor } from 'react';
import "./settings.css";
import translate, { LANGUAGES, Language, languageName, switchLanguage } from '../game/lang';
import GAME_MANAGER from '..';
import Anchor from './Anchor';
import StartMenu from './main/StartMenu';
import LoadingScreen from './LoadingScreen';
import { loadSettings, saveSettings } from '../game/localStorage';
import GameModesEditor from '../components/GameModesEditor';
import { CopyButton } from '../components/ClipboardButtons';
import ReactDOM from 'react-dom';
import WikiCoverCard from '../components/WikiCoverCard';
import Icon from '../components/Icon';

export type Settings = {
    volume: number,
    language: Language,
}

type SettingsProps = {
    onVolumeChange: (volume: number) => void,
    onClickOutside: (event: MouseEvent) => void,
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
    handleClickOutside: (event: MouseEvent) => void;
    constructor(props: SettingsProps) {
        super(props);

        this.state = {
            ...DEFAULT_SETTINGS,
            ...loadSettings()
        };

        this.handleClickOutside = (event: MouseEvent) => {
            // https://stackoverflow.com/a/45323523
            const domNode = ReactDOM.findDOMNode(this);
    
            if (!domNode || !domNode.contains(event.target as Node)) {
                setTimeout(() => {
                    this.props.onClickOutside(event);
                })
            }
        };
    }
    componentDidMount(): void {
        setTimeout(() => {
            document.addEventListener("click", this.handleClickOutside);
        });
    }
    componentWillUnmount(): void {
        document.removeEventListener("click", this.handleClickOutside);
    }
    async quitToMainMenu() {
        if (GAME_MANAGER.state.stateType === "game") {
            GAME_MANAGER.leaveGame();
        }
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
        const quitButtonBlacklist: (string | JSXElementConstructor<any>)[] = [StartMenu, LoadingScreen];

        return (
            <div className="settings slide-in">
                <section>
                    <h2><Icon>volume_up</Icon> {translate("menu.settings.volume")}</h2>
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
                    <h2><Icon>language</Icon> {translate("menu.settings.language")}</h2>
                    <select 
                        name="lang-select" 
                        defaultValue={loadSettings().language ?? "en_us"}
                        onChange={e => {
                            const language = e.target.options[e.target.selectedIndex].value as Language;
                            switchLanguage(language);
                            saveSettings({language});
                            Anchor.reload();
                        }}
                    >
                        {LANGUAGES.map(lang => <option key={lang} value={lang}>{languageName(lang)}</option>)}
                    </select>
                </section>
                { quitButtonBlacklist.includes(Anchor.contentType()) ||
                    <button onClick={(e)=>{this.quitToMainMenu()}}><Icon>not_interested</Icon> {translate("menu.settings.quitToMenu")}</button>
                }
                <button onClick={() => {this.goToRolelistEditor()}}><Icon>edit</Icon> {translate("menu.settings.gameSettingsEditor")}</button>
                <button onClick={()=>{
                    if(!window.confirm(translate("confirmDelete"))) return;
                    localStorage.clear();
                }}><Icon>delete_forever</Icon> {translate('menu.settings.eraseSaveData')}</button>
                <button onClick={() => {
                    Anchor.setCoverCard(<WikiCoverCard />, "wiki-menu-colors");
                    Anchor.closeSettings();
                }}><Icon>menu_book</Icon> {translate("menu.wiki.title")}</button>
                {(GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game") && <RoomLinkButton/>}
            </div>
        );
    }
}

export function RoomLinkButton(): JSX.Element {
    let code = new URL(window.location.href);
    
    if (GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game")
        code.searchParams.set("code", GAME_MANAGER.state.roomCode.toString(18));
    
    return <CopyButton text={code.toString()}>
        <Icon>link</Icon> {translate("menu.play.field.roomCode")}
    </CopyButton>
}