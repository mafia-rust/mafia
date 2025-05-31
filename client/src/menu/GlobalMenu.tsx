import React, {JSXElementConstructor, ReactElement, useContext, useEffect, useRef } from 'react';
import "./globalMenu.css";
import translate from '../game/lang';
import LoadingScreen from './LoadingScreen';
import GameModesEditor from '../components/gameModeSettings/GameModesEditor';
import { CopyButton } from '../components/ClipboardButtons';
import WikiCoverCard from '../components/WikiCoverCard';
import Icon from '../components/Icon';
import SettingsMenu from './Settings';
import { Button } from '../components/Button';
import HostMenu from './HostMenu';
import { AnchorContext } from './AnchorContext';
import { useLobbyOrGameState } from './lobby/LobbyContext';

export default function GlobalMenu(): ReactElement {

    const lobbyName = useLobbyOrGameState()!.lobbyName;
    const host = useLobbyOrGameState(
        state => {
            if (state.type === "game") {
                return state.host !== null
            } else {
                return state.players.get(state.myId!)?.ready === "host"
            }
        }
    )!;
    const stateType = useLobbyOrGameState(state => state.type)!;

    const ref = useRef<HTMLDivElement>(null);
    const anchorController = useContext(AnchorContext)!;

    useEffect(() => {
        const handleClickOutside = (event: MouseEvent) => {
            if (!ref.current?.contains(event.target as Node)) {
                anchorController.closeGlobalMenu();
            }
        };

        setTimeout(() => {
            document.addEventListener("click", handleClickOutside);
        })
        return () => document.removeEventListener("click", handleClickOutside);
    }, [anchorController]);
    
    async function quitToMainMenu() {
        if (stateType === "game") {
            GAME_MANAGER.leaveGame();
        }
        anchorController.closeGlobalMenu();
        anchorController.clearCoverCard();
        anchorController.setContent(<LoadingScreen type="disconnect"/>)
        window.history.replaceState({}, '', '/')
        await GAME_MANAGER.setDisconnectedState();
        anchorController.setContent({type:"main"})
    }
    function goToRolelistEditor() {
        anchorController.setCoverCard(<GameModesEditor/>);
        anchorController.closeGlobalMenu();
    }
    const quitButtonBlacklist: (string | JSXElementConstructor<any>)[] = ["main"];

    return (
        <div className="chat-menu-colors global-menu slide-in" ref={ref}>
            {(stateType === "lobby" || stateType === "game") && 
                <section className="standout">
                    <h2>{lobbyName}</h2>
                    <RoomLinkButton/>
                    {(stateType === "game" && host) && <>
                        <Button onClick={()=>GAME_MANAGER.sendBackToLobbyPacket()}>
                            {translate("backToLobby")}
                        </Button>
                        <Button onClick={()=>anchorController.setCoverCard(<HostMenu />)}>
                            {translate("menu.hostSettings.title")}
                        </Button>
                    </>}
                </section>
            }
            <section>
                {quitButtonBlacklist.includes(anchorController.contentType.type) ||
                    <Button onClick={() => quitToMainMenu()}><Icon>not_interested</Icon> {translate("menu.globalMenu.quitToMenu")}</Button>
                }
                <Button onClick={() => {
                    anchorController.setCoverCard(<SettingsMenu />)
                    anchorController.closeGlobalMenu();
                }}><Icon>settings</Icon> {translate("menu.globalMenu.settings")}</Button>
                <Button onClick={() => goToRolelistEditor()}><Icon>edit</Icon> {translate("menu.globalMenu.gameSettingsEditor")}</Button>
                <Button onClick={() => {
                    anchorController.setCoverCard(<WikiCoverCard />);
                    anchorController.closeGlobalMenu();
                }}><Icon>menu_book</Icon> {translate("menu.wiki.title")}</Button>
            </section>
        </div>
    );
}

export function RoomLinkButton(): JSX.Element {
    const code = useLobbyOrGameState(
        state => {
            const code = new URL(window.location.href);
            code.pathname = "/connect"
            code.searchParams.set("code", state.roomCode.toString(18))
            return code;
        }
    )!;
    
    return <CopyButton text={code.toString()}>
        <Icon>link</Icon> {translate("menu.play.field.roomCode")}
    </CopyButton>
}