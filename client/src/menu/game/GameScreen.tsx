import React, { ReactElement } from "react";
import HeaderMenu from "./HeaderMenu";
import GraveyardMenu from "./gameScreenContent/GraveyardMenu";
import ChatMenu from "./gameScreenContent/ChatMenu";
import PlayerListMenu from "./gameScreenContent/PlayerListMenu";
import WillMenu from "./gameScreenContent/WillMenu";
import GAME_MANAGER, { modulus } from "../..";
import WikiMenu from "./gameScreenContent/WikiMenu";
import "../../index.css";
import "./gameScreen.css";
import RoleSpecificMenu from "./gameScreenContent/RoleSpecific";
import Anchor from "../Anchor";
import StyledText from "../../components/StyledText";
import { Role } from "../../game/roleState.d";
import ROLES from "../../resources/roles.json";
import { StateEventType } from "../../game/gameManager.d";
import { WikiArticleLink } from "../../components/WikiArticleLink";
import Icon from "../../components/Icon";
import { Button } from "../../components/Button";
import translate from "../../game/lang";

export enum ContentMenu {
    ChatMenu = "ChatMenu",
    GraveyardMenu = "GraveyardMenu",
    PlayerListMenu = "PlayerListMenu",
    WillMenu = "WillMenu",
    WikiMenu = "WikiMenu",
    RoleSpecificMenu = "RoleSpecificMenu"
}

type GameScreenProps = {
    contentMenus: ContentMenu[],
    maxContent?: number | undefined
}
type GameScreenState = {
    maxContent: number,

    role: Role,

    chatMenuNotification: boolean,

    chatMenu: boolean,
    graveyardMenu: boolean,
    playerListMenu: boolean,
    willMenu: boolean,
    wikiMenu: boolean,
    roleSpecificMenu: boolean,
}

export default class GameScreen extends React.Component<GameScreenProps, GameScreenState> {
    static createDefault(): JSX.Element{
        if (Anchor.isMobile()) {
            return <GameScreen contentMenus={[
                ContentMenu.ChatMenu,
                ContentMenu.PlayerListMenu
            ]} maxContent={2}/>
        } else {
            return <GameScreen contentMenus={[
                ContentMenu.ChatMenu,
                ContentMenu.PlayerListMenu,
                ContentMenu.WillMenu,
                ContentMenu.GraveyardMenu
            ]}/>
        }
    }
    static instance: GameScreen;
    listener: (type: StateEventType | undefined) => void;
    swipeEventListener: (right: boolean) => void;

    constructor(props: GameScreenProps) {
        super(props);
        GameScreen.instance = this;

        if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player")
            this.state = {
                maxContent: props.maxContent?props.maxContent:Infinity,

                role: GAME_MANAGER.state.clientState.roleState?.type as Role,

                chatMenuNotification: false,

                chatMenu: props.contentMenus.includes(ContentMenu.ChatMenu),
                graveyardMenu: props.contentMenus.includes(ContentMenu.GraveyardMenu),
                playerListMenu: props.contentMenus.includes(ContentMenu.PlayerListMenu),
                willMenu: props.contentMenus.includes(ContentMenu.WillMenu),
                wikiMenu: props.contentMenus.includes(ContentMenu.WikiMenu),
                roleSpecificMenu : ROLES[GAME_MANAGER.state.clientState.roleState?.type as Role] !== undefined && 
                    ROLES[GAME_MANAGER.state.clientState.roleState?.type as Role].largeRoleSpecificMenu,
            }
        

        this.listener = (type)=>{
            if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player"){
                if(type === "yourRoleState"){
                    this.setState({
                        role: GAME_MANAGER.state.clientState.roleState?.type as Role,
                    });
                }
                if(type === "addChatMessages" && !GameScreen.instance.menusOpen().includes(ContentMenu.ChatMenu)){
                    this.setState({
                        chatMenuNotification: true,
                    });
                }
            }
        }
        this.swipeEventListener = (right)=>{

            let allowedToOpenRoleSpecific = 
                ROLES[this.state.role as Role] !== undefined && 
                ROLES[this.state.role as Role].largeRoleSpecificMenu

            //close this menu and open the next one
            let menusOpen = this.menusOpen();
            let lastOpenMenu = menusOpen[menusOpen.length - 1];

            let indexOfLastOpenMenu = this.menus().indexOf(lastOpenMenu);

            let nextIndex = modulus(
                indexOfLastOpenMenu + (right?-1:1), 
                this.menus().length
            );

            if(
                (nextIndex === this.menus().indexOf(ContentMenu.RoleSpecificMenu) && !allowedToOpenRoleSpecific) ||
                (this.menusOpen().includes(this.menus()[nextIndex]))
            ){
                nextIndex = modulus(
                    nextIndex + (right?-1:1),
                    this.menus().length
                );
            }
            
            this.closeMenu(lastOpenMenu);
            this.openMenu(this.menus()[nextIndex]);

        }

    }
    componentDidMount() {
        GameScreen.instance = this;
        GAME_MANAGER.addStateListener(this.listener);
        Anchor.addSwipeEventListener(this.swipeEventListener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
        Anchor.removeSwipeEventListener(this.swipeEventListener);
    }
    closeMenu(menu: ContentMenu) {
        switch(menu) {
            case ContentMenu.ChatMenu:
                this.setState({chatMenu: false});
                break;
            case ContentMenu.PlayerListMenu:
                this.setState({playerListMenu: false});
                break;
            case ContentMenu.WillMenu:
                this.setState({willMenu: false});
                break;
            case ContentMenu.RoleSpecificMenu:
                this.setState({roleSpecificMenu: false});
                break;
            case ContentMenu.GraveyardMenu:
                this.setState({graveyardMenu: false});
                break;
            case ContentMenu.WikiMenu:
                this.setState({wikiMenu: false});
                break;
        }
    }
    openMenu(menu: ContentMenu, callback: ()=>void = ()=>{}) {
        let menusOpen = this.menusOpen();
        if(menusOpen.length + 1 > this.state.maxContent && menusOpen.length > 0){
            this.closeMenu(menusOpen[menusOpen.length - 1]);
        }

        switch(menu) {
            case ContentMenu.ChatMenu:
                this.setState({
                    chatMenu: true,
                    chatMenuNotification: false
                }, callback);

                break;
            case ContentMenu.PlayerListMenu:
                this.setState({playerListMenu: true}, callback);
                break;
            case ContentMenu.WillMenu:
                this.setState({willMenu: true}, callback);
                break;
            case ContentMenu.GraveyardMenu:
                this.setState({graveyardMenu: true}, callback);
                break;
            case ContentMenu.RoleSpecificMenu:
                if(ROLES[this.state.role as Role] !== undefined && ROLES[this.state.role as Role].largeRoleSpecificMenu)
                    this.setState({roleSpecificMenu: true}, callback);
                break;
            case ContentMenu.WikiMenu:
                this.setState({wikiMenu: true}, callback);
                break;
        }
    }
    closeOrOpenMenu(menu: ContentMenu){
        if(this.menusOpen().includes(menu)){
            this.closeMenu(menu);
        }else{
            this.openMenu(menu);
        }
    }
    menusOpen(): ContentMenu[] {
        let out = [];
        if(this.state.chatMenu){
            out.push(ContentMenu.ChatMenu);
        }
        if(this.state.playerListMenu) {
            out.push(ContentMenu.PlayerListMenu);
        }
        if(this.state.willMenu) {
            out.push(ContentMenu.WillMenu);
        }
        if(this.state.roleSpecificMenu) {
            out.push(ContentMenu.RoleSpecificMenu);
        }
        if(this.state.graveyardMenu) {
            out.push(ContentMenu.GraveyardMenu);
        }
        if(this.state.wikiMenu) {
            out.push(ContentMenu.WikiMenu);
        }
        return out;
    }
    menus(): ContentMenu[] {
        return [
            ContentMenu.ChatMenu,
            ContentMenu.PlayerListMenu,
            ContentMenu.WillMenu,
            ContentMenu.RoleSpecificMenu,
            ContentMenu.GraveyardMenu,
            ContentMenu.WikiMenu
        ];
    }

    render() {
        if(GAME_MANAGER.state.stateType !== "game"){
            return;
        }

        const allMenusClosed = !this.state.chatMenu && !this.state.playerListMenu && !this.state.willMenu && !this.state.roleSpecificMenu && !this.state.graveyardMenu && !this.state.wikiMenu;

        return (
            <div className="game-screen">
                <div className="header">
                    <HeaderMenu phase={GAME_MANAGER.state.phaseState.type} chatMenuNotification={this.state.chatMenuNotification}/>
                </div>
                <div className="content">
                    {this.state.chatMenu && <ChatMenu/>}
                    {this.state.playerListMenu && <PlayerListMenu/>}
                    {this.state.willMenu && <WillMenu/>}
                    {this.state.roleSpecificMenu && <RoleSpecificMenu/>}
                    {this.state.graveyardMenu && <GraveyardMenu/>}
                    {this.state.wikiMenu && <WikiMenu/>}
                    {allMenusClosed && <div className="no-content">
                        {translate("menu.gameScreen.noContent")}
                    </div>}
                </div>
            </div>
        );
    }
}

export function ContentTab(props: {
    helpMenu: WikiArticleLink | null
    close: ContentMenu | false, 
    children: string 
}): ReactElement {

    return <div className="content-tab">
        <div>
            <StyledText>
                {props.children}
            </StyledText>
        </div>

        {props.close && <Button className="close"
            onClick={()=>GameScreen.instance.closeMenu(props.close as ContentMenu)}
        >
            <Icon size="small">close</Icon>
        </Button>}
        {props.helpMenu ? <Button className="help"
            onClick={()=>{
                GameScreen.instance.openMenu(ContentMenu.WikiMenu, ()=>{
                    props.helpMenu && GAME_MANAGER.setWikiArticle(props.helpMenu);
                });
            }}
        >
            <Icon size="small">question_mark</Icon>
        </Button>:null}
    </div>
}