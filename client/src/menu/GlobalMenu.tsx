import React, { JSXElementConstructor, ReactElement, useEffect, useRef } from 'react';
import "./globalMenu.css";
import translate from '../game/lang';
import GAME_MANAGER from '..';
import Anchor from './Anchor';
import StartMenu from './main/StartMenu';
import LoadingScreen from './LoadingScreen';
import GameModesEditor from '../components/gameModeSettings/GameModesEditor';
import { CopyButton } from '../components/ClipboardButtons';
import WikiCoverCard from '../components/WikiCoverCard';
import Icon from '../components/Icon';
import SettingsMenu from './Settings';
import { useLobbyOrGameState } from '../components/useHooks';

export default function GlobalMenu( props: Readonly<{
    closeGlobalMenu: (event: MouseEvent) => void
}> ): ReactElement {
    const lobbyName = useLobbyOrGameState(
        state => state.lobbyName,
        ["lobbyName"]
    )!;
    const host = useLobbyOrGameState(
        state => {
            if (state.stateType === "game") {
                return state.host
            } else {
                return state.players.get(state.myId!)?.host
            }
        },
        ["lobbyClients", "playersHost", "gamePlayers"]
    )!;
    const stateType = useLobbyOrGameState(
        state => state.stateType,
        ["acceptJoin", "rejectJoin", "rejectStart", "gameInitializationComplete", "startGame"]
    )!;
    const ref = useRef<HTMLDivElement>(null);

    useEffect(() => {
        const handleClickOutside = (event: MouseEvent) => {
            if (!ref.current?.contains(event.target as Node)) {
                setTimeout(() => {
                    props.closeGlobalMenu(event);
                })
            }
        };
        setTimeout(() => {
            document.addEventListener("click", handleClickOutside);
        });
        return () => document.removeEventListener("click", handleClickOutside);
    });
    
    async function quitToMainMenu() {
        if (GAME_MANAGER.state.stateType === "game") {
            GAME_MANAGER.leaveGame();
        }
        Anchor.closeGlobalMenu();
        Anchor.clearCoverCard();
        Anchor.setContent(<LoadingScreen type="disconnect"/>)
        await GAME_MANAGER.setDisconnectedState();
        Anchor.setContent(<StartMenu/>)
    }
    function goToRolelistEditor() {
        Anchor.setCoverCard(<GameModesEditor/>);
        Anchor.closeGlobalMenu();
    }
    const quitButtonBlacklist: (string | JSXElementConstructor<any>)[] = [StartMenu, LoadingScreen];

    return (
        <div className="chat-menu-colors global-menu slide-in" ref={ref}>
            {(stateType === "lobby" || stateType === "game") && 
                <section className="standout">
                    <h2>{lobbyName}</h2>
                    <RoomLinkButton/>
                    {(stateType === "game" && host) && <button onClick={()=>GAME_MANAGER.sendBackToLobbyPacket()}>
                        {translate("backToLobby")}
                    </button>}
                </section>
            }
            <section>
                { quitButtonBlacklist.includes(Anchor.contentType()) ||
                    <button onClick={() => quitToMainMenu()}><Icon>not_interested</Icon> {translate("menu.globalMenu.quitToMenu")}</button>
                }
                <button onClick={() => {
                    Anchor.setCoverCard(<SettingsMenu />)
                    Anchor.closeGlobalMenu();
                }}><Icon>settings</Icon> {translate("menu.globalMenu.settings")}</button>
                <button onClick={() => goToRolelistEditor()}><Icon>edit</Icon> {translate("menu.globalMenu.gameSettingsEditor")}</button>
                <button onClick={() => {
                    Anchor.setCoverCard(<WikiCoverCard />);
                    Anchor.closeGlobalMenu();
                }}><Icon>menu_book</Icon> {translate("menu.wiki.title")}</button>
            </section>
        </div>
    );
}

export function RoomLinkButton(): JSX.Element {
    let code = new URL(window.location.href);
    
    if (GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game")
        code.searchParams.set("code", GAME_MANAGER.state.roomCode.toString(18));
    
    return <CopyButton text={code.toString()}>
        <Icon>link</Icon> {translate("menu.play.field.roomCode")}
    </CopyButton>
}