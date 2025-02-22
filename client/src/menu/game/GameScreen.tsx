import React, { createContext, ReactElement, useCallback, useContext, useEffect, useState } from "react";
import HeaderMenu, { MenuButtons } from "./HeaderMenu";
import GraveyardMenu from "./gameScreenContent/GraveyardMenu";
import ChatMenu from "./gameScreenContent/ChatMenu";
import PlayerListMenu from "./gameScreenContent/PlayerListMenu";
import WillMenu from "./gameScreenContent/WillMenu";
import GAME_MANAGER, { DEV_ENV, modulus, Theme } from "../..";
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
import { Panel, PanelGroup, PanelResizeHandle } from "react-resizable-panels";
import { loadSettingsParsed } from "../../game/localStorage";

export enum ContentMenu {
    ChatMenu = "ChatMenu",
    RoleSpecificMenu = "RoleSpecificMenu",
    WillMenu = "WillMenu",
    PlayerListMenu = "PlayerListMenu",
    GraveyardMenu = "GraveyardMenu",
    WikiMenu = "WikiMenu",
}

export const MENU_ELEMENTS = {
    [ContentMenu.ChatMenu]: ChatMenu,
    [ContentMenu.PlayerListMenu]: PlayerListMenu,
    [ContentMenu.RoleSpecificMenu]: AbilityMenu,
    [ContentMenu.WillMenu]: WillMenu,
    [ContentMenu.GraveyardMenu]: GraveyardMenu,
    [ContentMenu.WikiMenu]: WikiMenu
}

export const MENU_THEMES: Record<ContentMenu, Theme | null> = {
    [ContentMenu.ChatMenu]: "chat-menu-colors",
    [ContentMenu.PlayerListMenu]: "player-list-menu-colors",
    [ContentMenu.RoleSpecificMenu]: "role-specific-colors",
    [ContentMenu.WillMenu]: "will-menu-colors",
    [ContentMenu.GraveyardMenu]: "graveyard-menu-colors",
    [ContentMenu.WikiMenu]: "wiki-menu-colors"
}

export const MENU_TRANSLATION_KEYS: Record<ContentMenu, string> = {
    [ContentMenu.ChatMenu]: "menu.chat",
    [ContentMenu.PlayerListMenu]: "menu.playerList",
    [ContentMenu.RoleSpecificMenu]: "menu.ability",
    [ContentMenu.WillMenu]: "menu.will",
    [ContentMenu.GraveyardMenu]: "menu.gameMode",
    [ContentMenu.WikiMenu]: "menu.wiki"
}

const ALL_CONTENT_MENUS = Object.values(ContentMenu);

export interface MenuController {
    closeOrOpenMenu(menu: ContentMenu): void;
    closeMenu(menu: ContentMenu): void;
    openMenu(menu: ContentMenu, callback?: ()=>void): void;
    menusOpen(): ContentMenu[];
    menuOpen(menu: ContentMenu): boolean;
    canOpen(menu: ContentMenu): boolean;
    menus(): ContentMenu[]
    maxMenus: number
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

                if (GAME_MANAGER.state.stateType === "game" && menu === ContentMenu.ChatMenu){
                    GAME_MANAGER.state.missedChatMessages = false;
                }
                GAME_MANAGER.invokeStateListeners("closeGameMenu");
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

                if (GAME_MANAGER.state.stateType === "game" && menu === ContentMenu.ChatMenu){
                    GAME_MANAGER.state.missedChatMessages = false;
                }

                if (callback) {
                    setCallbacks(callbacks => callbacks.concat(callback))
                }

                GAME_MANAGER.invokeStateListeners("openGameMenu");
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
            },
            menus(): ContentMenu[] {
                return Object.keys(contentMenus).filter(menu => contentMenus[menu as ContentMenu] !== undefined) as ContentMenu[];
            },
            maxMenus: maxContent
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
    const { maxMenus, menuOrder } = loadSettingsParsed();

    const menusOpen: [ContentMenu, boolean | undefined][] = [
        [ContentMenu.WikiMenu, mobile ? undefined : false ],
        [ContentMenu.GraveyardMenu, maxMenus > 4 ],
        [ContentMenu.PlayerListMenu, maxMenus > 1 ],
        [ContentMenu.ChatMenu, true ],
        [ContentMenu.WillMenu, maxMenus > 3 ],
        [ContentMenu.RoleSpecificMenu, maxMenus > 2 ],
    ];

    menusOpen.sort((a, b) => menuOrder.indexOf(a[0]) - menuOrder.indexOf(b[0]))

    const menuController = useMenuController(
        maxMenus, 
        Object.fromEntries(menusOpen),
        () => MENU_CONTROLLER_HOLDER.controller!,
        menuController => MENU_CONTROLLER_HOLDER.controller = menuController
    );

    const chatMenuNotification = useGameState(
        (game) => game.missedChatMessages && !menuController.menuOpen(ContentMenu.ChatMenu),
        ["addChatMessages", "openGameMenu", "closeGameMenu"]
    )!;
    

    useEffect(() => {
        const onBeforeUnload = (e: BeforeUnloadEvent) => {
            if (!DEV_ENV) e.preventDefault()
        };

        window.addEventListener("beforeunload", onBeforeUnload);
        return () => window.removeEventListener("beforeunload", onBeforeUnload);
    }, [])

    useEffect(() => {
        const swipeEventListener = (right: boolean) => {
            // Close the furthest right menu, open the next one to the left or right
            
            const menusOpen = menuController.menusOpen();
            if (menusOpen.length === 0) {
                return;
            }

            const allowedMenus = ALL_CONTENT_MENUS.filter(menu => { 
                return !menusOpen.includes(menu) && menuController.menus().includes(menu)
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

    return <MenuControllerContext.Provider value={menuController}>
        <div className="game-screen">
            <div className="header">
                <HeaderMenu chatMenuNotification={chatMenuNotification}/>
            </div>
            <GameScreenMenus />
            {mobile && <MenuButtons chatMenuNotification={chatMenuNotification}/>}
        </div>
    </MenuControllerContext.Provider>
}

export function GameScreenMenus(): ReactElement {
    const menuController = useContext(MenuControllerContext)!;
    const minSize = 10; // Percentage
    const mobile = useContext(MobileContext)!;

    // These don't add up to 100, but the panel group will fix it
    const defaultSizes = {
        [ContentMenu.ChatMenu]: 35,
        [ContentMenu.RoleSpecificMenu]: 15,
        [ContentMenu.WillMenu]: 15,
        [ContentMenu.PlayerListMenu]: 25,
        [ContentMenu.GraveyardMenu]: 10,
        [ContentMenu.WikiMenu]: 15,
    }

    return <PanelGroup direction="horizontal" className="content">
        {menuController.menusOpen().map((menu, index, menusOpen) => {
            const MenuElement = MENU_ELEMENTS[menu];
            return <>
                <Panel
                    className="panel"
                    minSize={minSize}
                    defaultSize={mobile===false?defaultSizes[menu]:undefined}
                    key={menu}
                >
                    <MenuElement />
                </Panel>
                {!mobile && menusOpen.some((_, i) => i > index) && <PanelResizeHandle key={index+".handle"} className="panel-handle"/>}
            </>
        })}
        {menuController.menusOpen().length === 0 && <Panel><div className="no-content">
            {translate("menu.gameScreen.noContent")}
        </div></Panel>}
    </PanelGroup>
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