import React, { JSXElementConstructor } from 'react';
import "./globalMenu.css";
import translate, { LANGUAGES, Language, languageName, switchLanguage } from '../game/lang';
import GAME_MANAGER from '..';
import Anchor from './Anchor';
import StartMenu from './main/StartMenu';
import LoadingScreen from './LoadingScreen';
import { loadSettings, saveSettings } from '../game/localStorage';
import GameModesEditor from '../components/gameModeSettings/GameModesEditor';
import { CopyButton } from '../components/ClipboardButtons';
import ReactDOM from 'react-dom';
import WikiCoverCard from '../components/WikiCoverCard';
import Icon from '../components/Icon';
import { StateListener } from '../game/gameManager.d';
import SettingsMenu from './Settings';

type GlobalMenuProps = {
    onClickOutside: (event: MouseEvent) => void,
}
type GlobalMenuState = {
    volume: number, // 0-1
    language: Language,
    lobbyName: string,
    host: boolean,
    lobbyState: "lobby" | "game" | "disconnected" | "outsideLobby",
}

export default class GlobalMenu extends React.Component<GlobalMenuProps, GlobalMenuState> {
    handleClickOutside: (event: MouseEvent) => void;
    listener: StateListener;

    constructor(props: GlobalMenuProps) {
        super(props);

        this.state = {
            ...loadSettings(),
            lobbyName: (GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game") ? GAME_MANAGER.state.lobbyName : "",
            host: GAME_MANAGER.getMyHost() ?? false,
            lobbyState: GAME_MANAGER.state.stateType,
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

        this.listener = type => {
            if (GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game") {
                this.setState({
                    lobbyName: GAME_MANAGER.state.lobbyName,
                    host: GAME_MANAGER.getMyHost() ?? false,
                    lobbyState: GAME_MANAGER.state.stateType,
                });
            }
        }
    }
    componentDidMount(): void {
        setTimeout(() => {
            document.addEventListener("click", this.handleClickOutside);
        });
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount(): void {
        document.removeEventListener("click", this.handleClickOutside);
        GAME_MANAGER.removeStateListener(this.listener);
    }
    async quitToMainMenu() {
        if (GAME_MANAGER.state.stateType === "game") {
            GAME_MANAGER.leaveGame();
        }
        Anchor.closeGlobalMenu();
        Anchor.clearCoverCard();
        Anchor.setContent(<LoadingScreen type="disconnect"/>)
        await GAME_MANAGER.setDisconnectedState();
        Anchor.setContent(<StartMenu/>)
    }
    goToRolelistEditor() {
        Anchor.setCoverCard(<GameModesEditor/>);
        Anchor.closeGlobalMenu();
    }
    render(): React.ReactNode {
        const quitButtonBlacklist: (string | JSXElementConstructor<any>)[] = [StartMenu, LoadingScreen];

        return (
            <div className="chat-menu-colors global-menu slide-in">
                {(this.state.lobbyState === "lobby" || this.state.lobbyState === "game") && 
                    <section className="standout">
                        <h2>{this.state.lobbyName}</h2>
                        <RoomLinkButton/>
                        {(this.state.lobbyState === "game" && this.state.host) && <button onClick={()=>GAME_MANAGER.sendBackToLobbyPacket()}>
                            {translate("backToLobby")}
                        </button>}
                    </section>
                }
                <section>
                    { quitButtonBlacklist.includes(Anchor.contentType()) ||
                        <button onClick={(e)=>{this.quitToMainMenu()}}><Icon>not_interested</Icon> {translate("menu.globalMenu.quitToMenu")}</button>
                    }
                    <button onClick={() => {
                        Anchor.setCoverCard(<SettingsMenu />)
                        Anchor.closeGlobalMenu();
                    }}><Icon>settings</Icon> {translate("menu.globalMenu.settings")}</button>
                    <button onClick={() => {this.goToRolelistEditor()}}><Icon>edit</Icon> {translate("menu.globalMenu.gameSettingsEditor")}</button>
                    <button onClick={() => {
                        Anchor.setCoverCard(<WikiCoverCard />);
                        Anchor.closeGlobalMenu();
                    }}><Icon>menu_book</Icon> {translate("menu.wiki.title")}</button>
                </section>
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