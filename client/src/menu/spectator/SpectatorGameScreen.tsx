import React, { ReactElement, useCallback, useContext, useEffect, useMemo, useState } from "react";
import "./spectatorGameScreen.css";
import PhaseStartedScreen from "./PhaseStartedScreen";
import { useGameState } from "../../components/useHooks";
import "../game/gameScreen.css"
import ChatMenu from "../game/gameScreenContent/ChatMenu";
import PlayerListMenu from "../game/gameScreenContent/PlayerListMenu";
import GraveyardMenu from "../game/gameScreenContent/GraveyardMenu";
import HeaderMenu from "../game/HeaderMenu";
import { ContentController, ContentMenu } from "../game/GameScreen";
import { AnchorContext } from "../Anchor";


const DEFAULT_START_PHASE_SCREEN_TIME = 3;

let CONTENT_CONTROLLER: ContentController | undefined;

export function getSpectatorScreenContentController(): ContentController | undefined {
    return CONTENT_CONTROLLER;
}

type SpectatorContentMenus = {
    chatMenu: boolean,
    playerListMenu: boolean,
    graveyardMenu: boolean
}

export default function SpectatorGameScreen(): ReactElement {
    const showStartedScreen = useGameState(
        gameState => {
            if (
                gameState.phaseState.type === "briefing"
                || gameState.phaseState.type === "obituary"
            ) return true;

            const maxTime = gameState.phaseTimes[gameState.phaseState.type];
            const timePassed = Math.floor(maxTime - gameState.timeLeftMs/1000);
            return timePassed < DEFAULT_START_PHASE_SCREEN_TIME;
        },
        ["phase", "phaseTimeLeft", "tick"],
        true
    )!

    const { mobile } = useContext(AnchorContext);

    const maxContent = useMemo(() => {
        return mobile ? 2 : undefined
    }, [mobile]);

    const [contentMenus, setContentMenus] = useState<SpectatorContentMenus>({
        chatMenu: true,
        playerListMenu: true,
        graveyardMenu: (maxContent ?? Infinity) > 2
    });

    const initializeContentController = useCallback(() => {
        function setContentMenu(menu: ContentMenu, open: boolean) {
            const newMenus = {...contentMenus};

            switch (menu) {
                case ContentMenu.ChatMenu:
                    newMenus.chatMenu = open;
                    break;
                case ContentMenu.PlayerListMenu:
                    newMenus.playerListMenu = open;
                    break;
                case ContentMenu.GraveyardMenu:
                    newMenus.graveyardMenu = open;
                    break;
                default:
                    console.log(`Spectator game screen does not have a ${menu} menu.`);
                    return;
            }

            // Obey props.maxContent
            // This is a bit hard-coded, but since there's only three menus it's fine.
            if (open === true && CONTENT_CONTROLLER!.menusOpen().length >= (maxContent ?? Infinity)) {
                if (menu === ContentMenu.ChatMenu || menu === ContentMenu.PlayerListMenu) {
                    newMenus.graveyardMenu = false;
                } else if (menu === ContentMenu.GraveyardMenu) {
                    newMenus.playerListMenu = false;
                }
            }

            // Keep one menu open
            if (open === false && CONTENT_CONTROLLER!.menusOpen().length === 1) {
                newMenus.chatMenu = true;
            }

            setContentMenus(newMenus);
        }

        CONTENT_CONTROLLER = {
            closeMenu(menu) {
                setContentMenu(menu, false)
            },
            closeOrOpenMenu(menu) {
                if (CONTENT_CONTROLLER?.menusOpen().includes(menu)) {
                    CONTENT_CONTROLLER?.closeMenu(menu)
                } else {
                    CONTENT_CONTROLLER?.openMenu(menu, () => {});
                }
            },
            openMenu(menu, callback) {
                setContentMenu(menu, true);
                
                // This isn't correct but it probably works.
                callback();
            },
            menusOpen(): ContentMenu[] {
                const openMenus = [];
                if (contentMenus.chatMenu) openMenus.push(ContentMenu.ChatMenu);
                if (contentMenus.playerListMenu) openMenus.push(ContentMenu.PlayerListMenu);
                if (contentMenus.graveyardMenu) openMenus.push(ContentMenu.GraveyardMenu);
                return openMenus
            },
        }
    }, [contentMenus, maxContent]);

    // Initialize on component load so MenuButtons component doesn't freak out
    initializeContentController();
    useEffect(() => {
        initializeContentController();
        return () => CONTENT_CONTROLLER = undefined;
    }, [initializeContentController])


    return (
        <div className="game-screen spectator-game-screen">
            <div className="header">
                <HeaderMenu chatMenuNotification={false}/>
            </div>
            {showStartedScreen 
                ? <PhaseStartedScreen/>
                : <div className="content">
                    {contentMenus.chatMenu && <ChatMenu/>}
                    {contentMenus.playerListMenu && <PlayerListMenu/>}
                    {contentMenus.graveyardMenu && <GraveyardMenu/>}
                </div>}
        </div>
    );
    
}