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
import RoleSpecificMenu from "./gameScreenContent/RoleSpecificMenu";
import { addSwipeEventListener, MobileContext, removeSwipeEventListener } from "../Anchor";
import StyledText from "../../components/StyledText";
import { WikiArticleLink } from "../../components/WikiArticleLink";
import Icon from "../../components/Icon";
import { Button } from "../../components/Button";
import translate from "../../game/lang";
import { roleSpecificMenuType } from "../Settings";
import { useGameState, usePlayerState } from "../../components/useHooks";

export enum ContentMenu {
    ChatMenu = "ChatMenu",
    GraveyardMenu = "GraveyardMenu",
    PlayerListMenu = "PlayerListMenu",
    WillMenu = "WillMenu",
    WikiMenu = "WikiMenu",
    RoleSpecificMenu = "RoleSpecificMenu"
}

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
            if(menusOpen.length + 1 > maxContent && menusOpen.length > 0){
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
    const roleState = usePlayerState(
        playerState => playerState.roleState,
        ["yourRoleState"]
    )!;
    const mobile = useContext(MobileContext)!;

    const menuController = useMenuController(
        mobile ? 2 : Infinity, 
        {
            ChatMenu: true,
            PlayerListMenu: true,
            WillMenu: !mobile,
            GraveyardMenu: !mobile,
            WikiMenu: false,
            RoleSpecificMenu: !mobile && roleSpecificMenuType(roleState.type) === "standalone"
        },
        () => MENU_CONTROLLER_HOLDER.controller!,
        menuController => MENU_CONTROLLER_HOLDER.controller = menuController
    );
    
    usePlayerState(
        playerState => {
            if (
                roleSpecificMenuType(playerState.roleState.type) !== "standalone" 
                && menuController.menuOpen(ContentMenu.RoleSpecificMenu)
            ) {
                menuController.closeMenu(ContentMenu.RoleSpecificMenu)
            }
        },
        ["yourRoleState"]
    );

    const chatMenuNotification = useGameState(
        () => !menuController.menusOpen().includes(ContentMenu.ChatMenu),
        ["addChatMessages"]
    )!;

    useEffect(() => {
        const swipeEventListener = (right: boolean) => {
            const allowedToOpenRoleSpecific = roleSpecificMenuType(roleState.type) === "standalone"
    
            //close this menu and open the next one
            const menusOpen = menuController.menusOpen();
            const lastOpenMenu = menusOpen[menusOpen.length - 1];

            const ALL_MENUS: Readonly<ContentMenu[]> = [
                ContentMenu.ChatMenu,
                ContentMenu.PlayerListMenu,
                ContentMenu.WillMenu,
                ContentMenu.RoleSpecificMenu,
                ContentMenu.GraveyardMenu,
                ContentMenu.WikiMenu
            ];
    
            const indexOfLastOpenMenu = ALL_MENUS.indexOf(lastOpenMenu);
    
            let nextIndex = modulus(
                indexOfLastOpenMenu + (right?-1:1), 
                ALL_MENUS.length
            );
    
            if(
                (nextIndex === ALL_MENUS.indexOf(ContentMenu.RoleSpecificMenu) && !allowedToOpenRoleSpecific) ||
                (menuController.menusOpen().includes(ALL_MENUS[nextIndex]))
            ){
                nextIndex = modulus(
                    nextIndex + (right?-1:1),
                    ALL_MENUS.length
                );
            }
            
            menuController.closeMenu(lastOpenMenu);
            menuController.openMenu(ALL_MENUS[nextIndex]);
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
                {menuController.menuOpen(ContentMenu.RoleSpecificMenu) && <RoleSpecificMenu/>}
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