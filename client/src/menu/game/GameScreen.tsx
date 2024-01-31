import React, { ReactElement } from "react";
import HeaderMenu from "./HeaderMenu";
import GraveyardMenu from "./gameScreenContent/GraveyardMenu";
import ChatMenu from "./gameScreenContent/ChatMenu";
import PlayerListMenu from "./gameScreenContent/PlayerListMenu";
import WillMenu from "./gameScreenContent/WillMenu";
import GAME_MANAGER, { modulus } from "../..";
import GameState from "../../game/gameState.d";
import WikiMenu from "./gameScreenContent/WikiMenu";
import "../../index.css";
import "./gameScreen.css";
import RoleSpecificMenu from "./gameScreenContent/RoleSpecific";
import Anchor from "../Anchor";
import StyledText from "../../components/StyledText";
import { Role } from "../../game/roleState.d";
import ROLES from "../../resources/roles.json";
import { StateEventType } from "../../game/gameManager.d";

export enum ContentMenus {
    ChatMenu = "ChatMenu",
    GraveyardMenu = "GraveyardMenu",
    PlayerListMenu = "PlayerListMenu",
    WillMenu = "WillMenu",
    WikiMenu = "WikiMenu",
    RoleSpecificMenu = "RoleSpecificMenu"
}

type GameScreenProps = {
    contentMenus: ContentMenus[],
    maxContent?: number | undefined
}
type GameScreenState = {
    gameState: GameState,
    maxContent: number,

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
                ContentMenus.ChatMenu,
            ]} maxContent={2}/>
        } else {
            return <GameScreen contentMenus={[
                ContentMenus.ChatMenu,
                // ContentMenus.GraveyardMenu,
                ContentMenus.PlayerListMenu,
                // ContentMenus.WikiMenu,
                ContentMenus.WillMenu
            ]}/>
        }
    }
    static instance: GameScreen;
    listener: (type: StateEventType | undefined) => void;
    swipeEventListener: (right: boolean) => void;

    constructor(props: GameScreenProps) {
        super(props);
        GameScreen.instance = this;

        if(GAME_MANAGER.state.stateType === "game")
            this.state = {
                maxContent: props.maxContent?props.maxContent:Infinity,
                gameState: GAME_MANAGER.state,

                chatMenuNotification: false,

                chatMenu: props.contentMenus.includes(ContentMenus.ChatMenu),
                graveyardMenu: props.contentMenus.includes(ContentMenus.GraveyardMenu),
                playerListMenu: props.contentMenus.includes(ContentMenus.PlayerListMenu),
                willMenu: props.contentMenus.includes(ContentMenus.WillMenu),
                wikiMenu: props.contentMenus.includes(ContentMenus.WikiMenu),
                roleSpecificMenu : props.contentMenus.includes(ContentMenus.RoleSpecificMenu)
            }
        

        this.listener = (type)=>{
            if(GAME_MANAGER.state.stateType === "game"){
                this.setState({
                    gameState: GAME_MANAGER.state,
                });
                if(type === "addChatMessages" && !GameScreen.instance.menusOpen().includes(ContentMenus.ChatMenu)){
                    this.setState({
                        chatMenuNotification: true,
                    });
                }
            }
        }
        this.swipeEventListener = (right)=>{

            let allowedToOpenRoleSpecific = 
                ROLES[this.state.gameState.roleState?.role as Role] !== undefined && 
                ROLES[this.state.gameState.roleState?.role as Role].largeRoleSpecificMenu

            //close this menu and open the next one
            let menusOpen = this.menusOpen();
            let lastOpenMenu = menusOpen[menusOpen.length - 1];

            let indexOfLastOpenMenu = this.menus().indexOf(lastOpenMenu);

            let nextIndex = modulus(indexOfLastOpenMenu + (right?-1:1), this.menus().length);
            if(nextIndex === this.menus().indexOf(ContentMenus.RoleSpecificMenu) && !allowedToOpenRoleSpecific){
                nextIndex = modulus(nextIndex + (right?-1:1), this.menus().length);
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
    closeMenu(menu: ContentMenus) {
        switch(menu) {
            case ContentMenus.ChatMenu:
                this.setState({chatMenu: false});
                break;
            case ContentMenus.PlayerListMenu:
                this.setState({playerListMenu: false});
                break;
            case ContentMenus.WillMenu:
                this.setState({willMenu: false});
                break;
            case ContentMenus.RoleSpecificMenu:
                this.setState({roleSpecificMenu: false});
                break;
            case ContentMenus.GraveyardMenu:
                this.setState({graveyardMenu: false});
                break;
            case ContentMenus.WikiMenu:
                this.setState({wikiMenu: false});
                break;
        }
    }
    openMenu(menu: ContentMenus) {
        let menusOpen = this.menusOpen();
        if(menusOpen.length + 1 > this.state.maxContent && menusOpen.length > 0){
            this.closeMenu(menusOpen[0]);
        }

        switch(menu) {
            case ContentMenus.ChatMenu:
                this.setState({
                    chatMenu: true,
                    chatMenuNotification: false
                });

                break;
            case ContentMenus.PlayerListMenu:
                this.setState({playerListMenu: true});
                break;
            case ContentMenus.WillMenu:
                this.setState({willMenu: true});
                break;
            case ContentMenus.GraveyardMenu:
                this.setState({graveyardMenu: true});
                break;
            case ContentMenus.RoleSpecificMenu:
                this.setState({roleSpecificMenu: true});
                break;
            case ContentMenus.WikiMenu:
                this.setState({wikiMenu: true});
                break;
        }
    }
    closeOrOpenMenu(menu: ContentMenus){
        if(this.menusOpen().includes(menu)){
            this.closeMenu(menu);
        }else{
            this.openMenu(menu);
        }
    }
    menusOpen(): ContentMenus[] {
        let out = [];
        if(this.state.chatMenu){
            out.push(ContentMenus.ChatMenu);
        }
        if(this.state.playerListMenu) {
            out.push(ContentMenus.PlayerListMenu);
        }
        if(this.state.willMenu) {
            out.push(ContentMenus.WillMenu);
        }
        if(this.state.roleSpecificMenu) {
            out.push(ContentMenus.RoleSpecificMenu);
        }
        if(this.state.graveyardMenu) {
            out.push(ContentMenus.GraveyardMenu);
        }
        if(this.state.wikiMenu) {
            out.push(ContentMenus.WikiMenu);
        }
        return out;
    }
    menus(): ContentMenus[] {
        let out = [];
        out.push(ContentMenus.ChatMenu);
        out.push(ContentMenus.PlayerListMenu);
        out.push(ContentMenus.WillMenu);
        out.push(ContentMenus.RoleSpecificMenu);
        out.push(ContentMenus.GraveyardMenu);
        out.push(ContentMenus.WikiMenu);
        return out;
    }

    render() {
        if(GAME_MANAGER.state.stateType !== "game"){
            return;
        }

        return (
            <div className="game-screen">
                <div className="header">
                    <HeaderMenu phase={GAME_MANAGER.state.phase} chatMenuNotification={this.state.chatMenuNotification}/>
                </div>
                <div className="content">
                    {this.state.chatMenu?<ChatMenu/>:null}
                    {this.state.playerListMenu?<PlayerListMenu/>:null}
                    {this.state.willMenu?<WillMenu/>:null}
                    {this.state.roleSpecificMenu?<RoleSpecificMenu/>:null}
                    {this.state.graveyardMenu?<GraveyardMenu/>:null}
                    {this.state.wikiMenu?<WikiMenu/>:null}
                </div>
            </div>
        );
    }
}

export function ContentTab(props: { close: ContentMenus | false, children: string }): ReactElement {
    return <div className="content-tab">
        <div>
            <StyledText>
                {props.children}
            </StyledText>
        </div>

        {props.close && <button 
            className="material-icons-round close" 
            onClick={()=>GameScreen.instance.closeMenu(props.close as ContentMenus)}
        >
            close
        </button>}
    </div>
}