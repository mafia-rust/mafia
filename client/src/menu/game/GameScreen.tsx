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
import Anchor, { AnchorContext } from "../Anchor";
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

export interface ContentController {
    closeOrOpenMenu(menu: ContentMenu): void;
    closeMenu(menu: ContentMenu): void;
    openMenu(menu: ContentMenu, callback?: ()=>void): void;
    menusOpen(): ContentMenu[];
    menuOpen(menu: ContentMenu): boolean;
    canOpen(menu: ContentMenu): boolean;
}

export function useContentController<C extends Partial<Record<ContentMenu, boolean>>>(
    maxContent: number, 
    initial: C,
    getContentController: () => ContentController,
    setContentController: (contentController: ContentController | undefined) => void,
): ContentController {
    const [contentMenus, setContentMenus] = useState<C>(initial);

    const initializeContentController = useCallback(() => {
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

            const menusOpen = getContentController().menusOpen();
            if(menusOpen.length + 1 > maxContent && menusOpen.length > 0){
                const menuToClose = menusOpen[menusOpen.length - 1];
                newMenus[menuToClose] = false;
            }

            setContentMenus(newMenus);
        }

        setContentController({
            closeMenu(menu) {
                setContentMenu(menu, false)
            },
            closeOrOpenMenu(menu) {
                if (getContentController().menusOpen().includes(menu)) {
                    getContentController().closeMenu(menu)
                } else {
                    getContentController().openMenu(menu, () => {});
                }
            },
            openMenu(menu, callback) {
                setContentMenu(menu, true);
                
                // This isn't correct but it probably works.
                callback && callback();
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
    }, [contentMenus, getContentController, maxContent, setContentController]);

    // Initialize on component load so MenuButtons component doesn't freak out
    initializeContentController();
    useEffect(() => {
        initializeContentController();
        return () => setContentController(undefined);
    }, [initializeContentController, setContentController])

    return getContentController();
}

const CONTENT_CONTROLLER_HOLDER: { controller: ContentController | undefined } = {
    controller: undefined
}

const ContentControllerContext = createContext<ContentController | undefined>(undefined)
export { ContentControllerContext }

export default function GameScreen(): ReactElement {
    const roleState = usePlayerState(
        playerState => playerState.roleState,
        ["yourRoleState"]
    )!;
    const { mobile } = useContext(AnchorContext)!;

    const contentController = useContentController(
        mobile ? 2 : Infinity, 
        {
            ChatMenu: true,
            PlayerListMenu: true,
            WillMenu: !mobile,
            GraveyardMenu: !mobile,
            WikiMenu: false,
            RoleSpecificMenu: !mobile && roleSpecificMenuType(roleState.type) === "standalone"
        },
        () => CONTENT_CONTROLLER_HOLDER.controller!,
        contentController => CONTENT_CONTROLLER_HOLDER.controller = contentController
    );
    
    usePlayerState(
        playerState => {
            if (
                roleSpecificMenuType(playerState.roleState.type) !== "standalone" 
                && contentController.menuOpen(ContentMenu.RoleSpecificMenu)
            ) {
                contentController.closeMenu(ContentMenu.RoleSpecificMenu)
            }
        },
        ["yourRoleState"]
    );

    const chatMenuNotification = useGameState(
        () => !contentController.menusOpen().includes(ContentMenu.ChatMenu),
        ["addChatMessages"]
    )!;

    useEffect(() => {
        const swipeEventListener = (right: boolean) => {
            const allowedToOpenRoleSpecific = roleSpecificMenuType(roleState.type) === "standalone"
    
            //close this menu and open the next one
            const menusOpen = contentController.menusOpen();
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
                (contentController.menusOpen().includes(ALL_MENUS[nextIndex]))
            ){
                nextIndex = modulus(
                    nextIndex + (right?-1:1),
                    ALL_MENUS.length
                );
            }
            
            contentController.closeMenu(lastOpenMenu);
            contentController.openMenu(ALL_MENUS[nextIndex]);
        }

        Anchor.addSwipeEventListener(swipeEventListener);
        return () => Anchor.removeSwipeEventListener(swipeEventListener);
    })

    const allMenusClosed = contentController.menusOpen().length === 0;

    return <ContentControllerContext.Provider value={contentController}>
        <div className="game-screen">
            <div className="header">
                <HeaderMenu chatMenuNotification={chatMenuNotification}/>
            </div>
            <div className="content">
                {contentController.menuOpen(ContentMenu.ChatMenu) && <ChatMenu/>}
                {contentController.menuOpen(ContentMenu.PlayerListMenu) && <PlayerListMenu/>}
                {contentController.menuOpen(ContentMenu.WillMenu) && <WillMenu/>}
                {contentController.menuOpen(ContentMenu.RoleSpecificMenu) && <RoleSpecificMenu/>}
                {contentController.menuOpen(ContentMenu.GraveyardMenu) && <GraveyardMenu/>}
                {contentController.menuOpen(ContentMenu.WikiMenu) && <WikiMenu/>}
                {allMenusClosed && <div className="no-content">
                    {translate("menu.gameScreen.noContent")}
                </div>}
            </div>
        </div>
    </ContentControllerContext.Provider>
}

export function ContentTab(props: Readonly<{
    helpMenu: WikiArticleLink | null
    close: ContentMenu | false, 
    children: string 
}>): ReactElement {
    const contentController = useContext(ContentControllerContext)!;
    const spectator = useGameState(
        gameState => gameState.clientState.type === "spectator",
        ["gamePlayers"]
    )!;
    const { mobile } = useContext(AnchorContext)!;

    return <div className="content-tab">
        <div>
            <StyledText>
                {props.children}
            </StyledText>
        </div>

        {props.close && (!spectator || mobile) && <Button className="close"
            onClick={()=>contentController.closeMenu(props.close as ContentMenu)}
        >
            <Icon size="small">close</Icon>
        </Button>}
        {props.helpMenu && !spectator && <Button className="help"
            onClick={()=>{
                contentController.openMenu(ContentMenu.WikiMenu, ()=>{
                    props.helpMenu && GAME_MANAGER.setWikiArticle(props.helpMenu);
                });
            }}
        >
            <Icon size="small">question_mark</Icon>
        </Button>}
    </div>
}