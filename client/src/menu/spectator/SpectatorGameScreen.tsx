import React, { ReactElement, useContext } from "react";
import "../game/gameScreen.css"
import HeaderMenu from "../game/HeaderMenu";
import { MenuController, useMenuController, MenuControllerContext, MENU_ELEMENTS } from "../game/GameScreen";
import { MobileContext } from "../Anchor";
import { Panel, PanelGroup, PanelResizeHandle } from "react-resizable-panels";
import translate from "../../game/lang";

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
                <PanelGroup direction="horizontal" className="content">
                    {contentController.menusOpen().map((menu, index, menusOpen) => {
                        const MenuElement = MENU_ELEMENTS[menu];
                        return <>
                            <Panel minSize={10}>
                                <MenuElement />
                            </Panel>
                            {menusOpen.some((_, i) => i > index) && <PanelResizeHandle />}
                        </>
                    })}
                    {contentController.menusOpen().length === 0 && <Panel><div className="no-content">
                        {translate("menu.gameScreen.noContent")}
                    </div></Panel>}
                </PanelGroup>
            </div>
        </MenuControllerContext.Provider>
    );
    
}