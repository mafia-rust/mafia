import React, { ReactElement, useContext } from "react";
import "../game/gameScreen.css"
import HeaderMenu, { MenuButtons } from "../game/HeaderMenu";
import { MenuController, useMenuController, MenuControllerContext, GameScreenMenus } from "../game/GameScreen";
import { MobileContext } from "../Anchor";

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
    const mobile = useContext(MobileContext)!;

    const contentController = useMenuController<SpectatorContentMenus>(
        mobile ? 2 : Infinity,
        {
            GraveyardMenu: !mobile,
            PlayerListMenu: true,
            ChatMenu: true
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
                <GameScreenMenus />
                {mobile && <MenuButtons chatMenuNotification={false}/>}
            </div>
        </MenuControllerContext.Provider>
    );
    
}