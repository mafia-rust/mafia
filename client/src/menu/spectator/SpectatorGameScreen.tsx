import React, { ReactElement, useContext } from "react";
import "./spectatorGameScreen.css";
import PhaseStartedScreen from "./PhaseStartedScreen";
import { useGameState } from "../../components/useHooks";
import "../game/gameScreen.css"
import ChatMenu from "../game/gameScreenContent/ChatMenu";
import PlayerListMenu from "../game/gameScreenContent/PlayerListMenu";
import GraveyardMenu from "../game/gameScreenContent/GraveyardMenu";
import HeaderMenu from "../game/HeaderMenu";
import { MenuController, ContentMenu, useMenuController, MenuControllerContext } from "../game/GameScreen";
import { MobileContext } from "../Anchor";


const DEFAULT_START_PHASE_SCREEN_TIME = 3;

let CONTENT_CONTROLLER: MenuController | undefined;

export function getSpectatorScreenContentController(): MenuController | undefined {
    return CONTENT_CONTROLLER;
}

type SpectatorContentMenus = {
    ChatMenu: boolean,
    PlayerListMenu: boolean,
    GraveyardMenu: boolean
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

    const mobile = useContext(MobileContext)!;

    const contentController = useMenuController<SpectatorContentMenus>(
        mobile ? 2 : Infinity,
        {
            ChatMenu: true,
            PlayerListMenu: true,
            GraveyardMenu: !mobile
        },
        () => CONTENT_CONTROLLER!,
        contentController => CONTENT_CONTROLLER = contentController
    );


    return (
        <MenuControllerContext.Provider value={contentController}>
            <div className="game-screen spectator-game-screen">
                <div className="header">
                    <HeaderMenu chatMenuNotification={false}/>
                </div>
                {showStartedScreen 
                    ? <PhaseStartedScreen/>
                    : <div className="content">
                        {contentController.menuOpen(ContentMenu.ChatMenu) && <ChatMenu/>}
                        {contentController.menuOpen(ContentMenu.PlayerListMenu) && <PlayerListMenu/>}
                        {contentController.menuOpen(ContentMenu.GraveyardMenu) && <GraveyardMenu/>}
                    </div>}
            </div>
        </MenuControllerContext.Provider>
    );
    
}