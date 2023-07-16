import React from "react";
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
import RoleSpecifcMenu from "./gameScreenContent/RoleSpecific";
import Anchor from "../Anchor";

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
                ContentMenus.GraveyardMenu,
                ContentMenus.PlayerListMenu,
                ContentMenus.WikiMenu,
                ContentMenus.WillMenu
            ]} maxContent={false}/>
        }
    }
    static instance: GameScreen;
    listener: () => void;

    constructor(props: GameScreenProps) {
        super(props);
        GameScreen.instance = this;
        this.state = {
            maxContent: props.maxContent?props.contentMenus.length:Infinity,
            gameState: GAME_MANAGER.gameState,

            graveyardMenu: props.contentMenus.includes(ContentMenus.GraveyardMenu),
            playerListMenu: props.contentMenus.includes(ContentMenus.PlayerListMenu),
            willMenu: props.contentMenus.includes(ContentMenus.WillMenu),
            wikiMenu: props.contentMenus.includes(ContentMenus.WikiMenu),
            roleSpecificMenu : props.contentMenus.includes(ContentMenus.RoleSpecificMenu)
        };

        this.listener = ()=>{
            this.setState({
                gameState: GAME_MANAGER.gameState,
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
        switch(menu) {
            case ContentMenus.GraveyardMenu:
                this.setState({graveyardMenu: !this.menusOpen().includes(menu)});
                break;
            case ContentMenus.PlayerListMenu:
                this.setState({playerListMenu: !this.menusOpen().includes(menu)});
                break;
            case ContentMenus.WillMenu:
                this.setState({willMenu: !this.menusOpen().includes(menu)});
                break;
            case ContentMenus.WikiMenu:
                this.setState({wikiMenu: !this.menusOpen().includes(menu)});
                break;
            case ContentMenus.RoleSpecificMenu:
                this.setState({roleSpecificMenu: !this.menusOpen().includes(menu)});
                break;
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
        return (
            <div className="game-screen">
                <button className="leave-button" onClick={() => GAME_MANAGER.leaveGame()}>
                    {translate("menu.button.leave")}
                </button>
                <div className="header">
                    <HeaderMenu phase={GAME_MANAGER.gameState.phase}/>
                </div>
                <div className="content">
                    {this.state.graveyardMenu?<GraveyardMenu/>:null}
                    <ChatMenu/>
                    {this.state.playerListMenu?<PlayerListMenu/>:null}
                    {this.state.willMenu?<WillMenu/>:null}
                    {this.state.roleSpecificMenu?<RoleSpecifcMenu/>:null}
                    {this.state.wikiMenu?<WikiMenu/>:null}
                </div>
            </div>
        );
    }
}
