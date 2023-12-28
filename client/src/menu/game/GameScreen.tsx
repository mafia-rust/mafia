import React, { ReactElement } from "react";
import HeaderMenu from "./HeaderMenu";
import GraveyardMenu from "./gameScreenContent/GraveyardMenu";
import ChatMenu from "./gameScreenContent/ChatMenu";
import PlayerListMenu from "./gameScreenContent/PlayerListMenu";
import WillMenu from "./gameScreenContent/WillMenu";
import GAME_MANAGER from "../..";
import GameState from "../../game/gameState.d";
import WikiMenu from "./gameScreenContent/WikiMenu";

import "../../index.css";
import "./gameScreen.css";
import translate from "../../game/lang";
import RoleSpecificMenu from "./gameScreenContent/RoleSpecific";
import Anchor from "../Anchor";
import StyledText from "../../components/StyledText";

export enum ContentMenus {
    GraveyardMenu = "GraveyardMenu",
    PlayerListMenu = "PlayerListMenu",
    WillMenu = "WillMenu",
    WikiMenu = "WikiMenu",
    RoleSpecificMenu = "RoleSpecificMenu"
}

type GameScreenProps = {
    contentMenus: ContentMenus[],
    maxContent: boolean
}
type GameScreenState = {
    gameState: GameState,
    maxContent: number,

    graveyardMenu: boolean,
    playerListMenu: boolean,
    willMenu: boolean,
    wikiMenu: boolean,
    roleSpecificMenu: boolean
}

export default class GameScreen extends React.Component<GameScreenProps, GameScreenState> {
    static createDefault(): JSX.Element{
        if (Anchor.isMobile()) {
            return <GameScreen contentMenus={[
                ContentMenus.PlayerListMenu,
            ]} maxContent={true}/>
        } else {
            return <GameScreen contentMenus={[
                // ContentMenus.GraveyardMenu,
                ContentMenus.PlayerListMenu,
                // ContentMenus.WikiMenu,
                ContentMenus.WillMenu
            ]} maxContent={false}/>
        }
    }
    static instance: GameScreen;
    listener: () => void;

    constructor(props: GameScreenProps) {
        super(props);
        GameScreen.instance = this;

        if(GAME_MANAGER.state.stateType === "game")
            this.state = {
                maxContent: props.maxContent?props.contentMenus.length:Infinity,
                gameState: GAME_MANAGER.state,

                graveyardMenu: props.contentMenus.includes(ContentMenus.GraveyardMenu),
                playerListMenu: props.contentMenus.includes(ContentMenus.PlayerListMenu),
                willMenu: props.contentMenus.includes(ContentMenus.WillMenu),
                wikiMenu: props.contentMenus.includes(ContentMenus.WikiMenu),
                roleSpecificMenu : props.contentMenus.includes(ContentMenus.RoleSpecificMenu)
            };

        this.listener = ()=>{
            if(GAME_MANAGER.state.stateType === "game")
                this.setState({
                    gameState: GAME_MANAGER.state,
                });
        }
    }
    componentDidMount() {
        GameScreen.instance = this;
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }
    closeMenu(menu: ContentMenus) {
        switch(menu) {
            case ContentMenus.GraveyardMenu:
                this.setState({graveyardMenu: false});
                break;
            case ContentMenus.PlayerListMenu:
                this.setState({playerListMenu: false});
                break;
            case ContentMenus.WillMenu:
                this.setState({willMenu: false});
                break;
            case ContentMenus.WikiMenu:
                this.setState({wikiMenu: false});
                break;
            case ContentMenus.RoleSpecificMenu:
                this.setState({roleSpecificMenu: false});
                break;
        }
    }
    openMenu(menu: ContentMenus) {
        let menusOpen = this.menusOpen();
        if(menusOpen.length + 1 > this.state.maxContent && menusOpen.length > 0){
            this.closeMenu(menusOpen[0]);
        }

        switch(menu) {
            case ContentMenus.GraveyardMenu:
                this.setState({graveyardMenu: true});
                break;
            case ContentMenus.PlayerListMenu:
                this.setState({playerListMenu: true});
                break;
            case ContentMenus.WillMenu:
                this.setState({willMenu: true});
                break;
            case ContentMenus.WikiMenu:
                this.setState({wikiMenu: true});
                break;
            case ContentMenus.RoleSpecificMenu:
                this.setState({roleSpecificMenu: true});
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
        if(this.state.graveyardMenu) {
            out.push(ContentMenus.GraveyardMenu);
        }
        if(this.state.playerListMenu) {
            out.push(ContentMenus.PlayerListMenu);
        }
        if(this.state.willMenu) {
            out.push(ContentMenus.WillMenu);
        }
        if(this.state.wikiMenu) {
            out.push(ContentMenus.WikiMenu);
        }
        if(this.state.roleSpecificMenu) {
            out.push(ContentMenus.RoleSpecificMenu);
        }
        return out;
    }

    render() {
        if(GAME_MANAGER.state.stateType !== "game"){
            return;
        }

        return (
            <div className="game-screen">
                <button className="material-icons-round leave-button" onClick={() => GAME_MANAGER.leaveGame()}>
                    close
                </button>
                <div className="header">
                    <HeaderMenu phase={GAME_MANAGER.state.phase}/>
                </div>
                <div className="content">
                    <ChatMenu/>
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
            aria-label={translate("menu.button.close")}
        >
            close
        </button>}
    </div>
}