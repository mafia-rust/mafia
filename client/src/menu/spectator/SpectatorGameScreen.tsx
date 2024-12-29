import React, { ReactElement, useContext } from "react";
import "../game/gameScreen.css"
import HeaderMenu, { MenuButtons } from "../game/HeaderMenu";
import { MenuController, useMenuController, MenuControllerContext, GameScreenMenus, ContentMenu } from "../game/GameScreen";
import { MobileContext } from "../Anchor";
import { loadSettingsParsed } from "../../game/localStorage";

let CONTENT_CONTROLLER: MenuController | undefined;

export function getSpectatorScreenContentController(): MenuController | undefined {
    return CONTENT_CONTROLLER;
}

export default function SpectatorGameScreen(): ReactElement {
    const mobile = useContext(MobileContext)!;
    const { maxMenus, menuOrder } = loadSettingsParsed();

    const menusOpen: [ContentMenu, boolean | undefined][] = [
        [ContentMenu.WikiMenu, undefined ],
        [ContentMenu.GraveyardMenu, maxMenus > 2 ],
        [ContentMenu.PlayerListMenu, maxMenus > 1 ],
        [ContentMenu.ChatMenu, true ],
        [ContentMenu.WillMenu, undefined ],
        [ContentMenu.RoleSpecificMenu, undefined ],
    ];

    menusOpen.sort((a, b) => menuOrder.indexOf(a[0]) - menuOrder.indexOf(b[0]))

    const contentController = useMenuController(
        maxMenus,
        Object.fromEntries(menusOpen),
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
                {mobile === true && <MenuButtons chatMenuNotification={false}/>}
            </div>
        </MenuControllerContext.Provider>
    );
    
}