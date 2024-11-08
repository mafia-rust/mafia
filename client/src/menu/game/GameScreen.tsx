import React, { createContext, ReactElement, useCallback, useContext, useEffect, useState } from "react";
import HeaderMenu from "./HeaderMenu";
import GraveyardMenu from "./gameScreenContent/GraveyardMenu";
import ChatMenu from "./gameScreenContent/ChatMenu";
import PlayerListMenu from "./gameScreenContent/PlayerListMenu";
import WillMenu from "./gameScreenContent/WillMenu";
import GAME_MANAGER, { modulus } from "../..";
import WikiMenu from "./gameScreenContent/WikiMenu";
import "../../index.css";
import "./gameScreen.css";
import AbilityMenu from "./gameScreenContent/AbilityMenu/AbilityMenu";
import { addSwipeEventListener, MobileContext, removeSwipeEventListener } from "../Anchor";
import StyledText from "../../components/StyledText";
import { WikiArticleLink } from "../../components/WikiArticleLink";
import Icon from "../../components/Icon";
import { Button } from "../../components/Button";
import translate from "../../game/lang";
import { useGameState } from "../../components/useHooks";

export enum ContentMenu {
    ChatMenu = "ChatMenu",
    PlayerListMenu = "PlayerListMenu",
    RoleSpecificMenu = "RoleSpecificMenu",
    WillMenu = "WillMenu",
    GraveyardMenu = "GraveyardMenu",
    WikiMenu = "WikiMenu",
}
const ALL_CONTENT_MENUS = Object.values(ContentMenu);

export interface MenuController {
    closeOrOpenMenu(menu: ContentMenu): void;
    closeMenu(menu: ContentMenu): void;
    openMenu(menu: ContentMenu, callback?: ()=>void): void;
    menusOpen(): ContentMenu[];
    menuOpen(menu: ContentMenu): boolean;
    canOpen(menu: ContentMenu): boolean;
}

export function useMenuController<C extends Partial<Record<ContentMenu, boolean>>>(
    maxContent: number, 
    initial: C,
    getMenuController: () => MenuController,
    setMenuController: (menuController: MenuController | undefined) => void,
): MenuController {
    const [contentMenus, setContentMenus] = useState<C>(initial);

    const [callbacks, setCallbacks] = useState<(() => void)[]>([]);

    useEffect(() => {
        for (const callback of callbacks) {
            callback();
        }
        if (callbacks.length !== 0) {
            setCallbacks([])
        }
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [contentMenus])

    const initializeMenuController = useCallback(() => {
        function setAndGetContentMenus(menu: ContentMenu, open: boolean): C {
            const newMenus = {...contentMenus};

            if (newMenus[menu] === undefined) {
                console.log(`This screen does not have a ${menu} menu.`);
                return newMenus;
            } else {
                newMenus[menu] = open;
            }

            return newMenus;
        }

        function setContentMenu(menu: ContentMenu, open: boolean) {
            const newMenus = setAndGetContentMenus(menu, open)

            const menusOpen = getMenuController().menusOpen();
            if(menusOpen.length + 1 > maxContent && menusOpen.length > 0 && open) {
                const menuToClose = menusOpen[menusOpen.length - 1];
                newMenus[menuToClose] = false;
            }

            setContentMenus(newMenus);
        }

        setMenuController({
            closeMenu(menu) {
                setContentMenu(menu, false)
            },
            closeOrOpenMenu(menu) {
                if (getMenuController().menusOpen().includes(menu)) {
                    getMenuController().closeMenu(menu)
                } else {
                    getMenuController().openMenu(menu, () => {});
                }
            },
            openMenu(menu, callback) {
                setContentMenu(menu, true);
                
                if (callback) {
                    setCallbacks(callbacks => callbacks.concat(callback))
                }
            },
            menusOpen(): ContentMenu[] {
                return Object.entries(contentMenus)
                    .filter(([_, open]) => open)
                    .map(([menu, _]) => menu) as ContentMenu[];
            },
            menuOpen(menu): boolean {
                return this.menusOpen().includes(menu);
            },
            canOpen(menu): boolean {
                return contentMenus[menu] !== undefined
            }
        })
    }, [contentMenus, getMenuController, maxContent, setMenuController]);

    // Initialize on component load so MenuButtons component doesn't freak out
    initializeMenuController();
    useEffect(() => {
        initializeMenuController();
        return () => setMenuController(undefined);
    }, [initializeMenuController, setMenuController])

    return getMenuController();
}

const MENU_CONTROLLER_HOLDER: { controller: MenuController | undefined } = {
    controller: undefined
}

const MenuControllerContext = createContext<MenuController | undefined>(undefined)
export { MenuControllerContext }

export default function GameScreen(): ReactElement {
    const mobile = useContext(MobileContext)!;

    const menuController = useMenuController(
        mobile ? 2 : Infinity, 
        {
            ChatMenu: true,
            PlayerListMenu: true,
            RoleSpecificMenu: !mobile,
            WillMenu: !mobile,
            GraveyardMenu: !mobile,
            WikiMenu: false,
        },
        () => MENU_CONTROLLER_HOLDER.controller!,
        menuController => MENU_CONTROLLER_HOLDER.controller = menuController
    );

    const chatMenuNotification = useGameState(
        () => !menuController.menusOpen().includes(ContentMenu.ChatMenu),
        ["addChatMessages"]
    )!;

    useEffect(() => {
        const swipeEventListener = (right: boolean) => {
            // Close the furthest right menu, open the next one to the left or right
            
            const menusOpen = menuController.menusOpen();
            if (menusOpen.length === 0) {
                return;
            }

            const allowedMenus = ALL_CONTENT_MENUS.filter(menu => { 
                return !menusOpen.includes(menu)
            });

            const rightMostMenu = menusOpen[menusOpen.length - 1];
            const index = ALL_CONTENT_MENUS.indexOf(rightMostMenu);
            let nextMenu = ALL_CONTENT_MENUS[modulus(index + (right ? -1 : 1), ALL_CONTENT_MENUS.length)];
            while (!allowedMenus.includes(nextMenu)) {
                nextMenu = ALL_CONTENT_MENUS[modulus(ALL_CONTENT_MENUS.indexOf(nextMenu) + (right ? -1 : 1), ALL_CONTENT_MENUS.length)];
            }

            menuController.closeMenu(rightMostMenu);
            menuController.openMenu(nextMenu);
        }

        addSwipeEventListener(swipeEventListener);
        return () => removeSwipeEventListener(swipeEventListener);
    })

    const allMenusClosed = menuController.menusOpen().length === 0;

    return <MenuControllerContext.Provider value={menuController}>
        <div className="game-screen">
            <div className="header">
                <HeaderMenu chatMenuNotification={chatMenuNotification}/>
            </div>
            <div className="content">
                {menuController.menuOpen(ContentMenu.ChatMenu) && <ChatMenu/>}
                {menuController.menuOpen(ContentMenu.PlayerListMenu) && <PlayerListMenu/>}
                {menuController.menuOpen(ContentMenu.WillMenu) && <WillMenu/>}
                {menuController.menuOpen(ContentMenu.RoleSpecificMenu) && <AbilityMenu/>}
                {menuController.menuOpen(ContentMenu.GraveyardMenu) && <GraveyardMenu/>}
                {menuController.menuOpen(ContentMenu.WikiMenu) && <WikiMenu/>}
                {allMenusClosed && <div className="no-content">
                    {translate("menu.gameScreen.noContent")}
                </div>}
            </div>
        </div>
    </MenuControllerContext.Provider>
}

export function ContentTab(props: Readonly<{
    helpMenu: WikiArticleLink | null
    close: ContentMenu | false, 
    children: string 
}>): ReactElement {
    const menuController = useContext(MenuControllerContext)!;
    const spectator = useGameState(
        gameState => gameState.clientState.type === "spectator",
        ["gamePlayers"]
    )!;
    const mobile = useContext(MobileContext)!;

    return <div className="content-tab">
        <div>
            <StyledText>
                {props.children}
            </StyledText>
        </div>

        {props.close && (!spectator || mobile) && <Button className="close"
            onClick={()=>menuController.closeMenu(props.close as ContentMenu)}
        >
            <Icon size="small">close</Icon>
        </Button>}
        {props.helpMenu && !spectator && <Button className="help"
            onClick={()=>{
                menuController.openMenu(ContentMenu.WikiMenu, ()=>{
                    props.helpMenu && GAME_MANAGER.setWikiArticle(props.helpMenu);
                });
            }}
        >
            <Icon size="small">question_mark</Icon>
        </Button>}
    </div>
}