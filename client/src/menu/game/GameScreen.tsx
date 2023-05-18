import React from "react";
import * as LoadingScreen from "../LoadingScreen";
import PhaseRowMenu from "./PhaseRowMenu";
import GraveyardMenu from "./gameScreenContent/GraveyardMenu";
import ChatMenu from "./gameScreenContent/ChatMenu";
import PlayerListMenu from "./gameScreenContent/PlayerListMenu";
import WillMenu from "./gameScreenContent/WillMenu";
import GAME_MANAGER from "../..";
import GameState from "../../game/gameState.d";
import WikiMenu from "./gameScreenContent/WikiMenu";

import "../../index.css";
import "./gameScreen.css";

export enum ContentMenus {
    GraveyardMenu = "GraveyardMenu",
    PlayerListMenu = "PlayerListMenu",
    WillMenu = "WillMenu",
    WikiMenu = "WikiMenu",
}

type GameScreenProps = {
    
}
type GameScreenState = {
    gameState: GameState,
    header: JSX.Element,
    content: JSX.Element[],
}

export default class GameScreen extends React.Component<GameScreenProps, GameScreenState> {
    static instance: GameScreen;
    listener: () => void;

    constructor(props: GameScreenProps) {
        super(props);
        GameScreen.instance = this;
        this.state = {
            header: LoadingScreen.create(),
            content: [LoadingScreen.create()],
            gameState: GAME_MANAGER.gameState,
        };

        this.listener = ()=>{
            this.setState({
                gameState: GAME_MANAGER.gameState,
            })
        }
    }
    
    openMenu(menu: ContentMenus) {
        switch(menu) {
            case ContentMenus.GraveyardMenu:
                this.state.content.push(<GraveyardMenu/>);
                this.setState({
                    content: this.state.content,
                });
                GAME_MANAGER.graveyardMenuOpen = true;
                break;
            case ContentMenus.PlayerListMenu:
                this.state.content.push(<PlayerListMenu/>);
                this.setState({
                    content: this.state.content,
                });
                GAME_MANAGER.playerListMenuOpen = true;
                break;
            case ContentMenus.WillMenu:
                this.state.content.push(<WillMenu/>);
                this.setState({
                    content: this.state.content,
                });
                GAME_MANAGER.willMenuOpen = true;
                break;
            case ContentMenus.WikiMenu:
                this.state.content.push(<WikiMenu role={{type: "any"}}/>);
                this.setState({
                    content: this.state.content,
                });
                GAME_MANAGER.wikiMenuOpen = true;
                break;
        }
        GAME_MANAGER.invokeStateListeners();
    }
    closeMenu(menu: ContentMenus) {
        for(let i = 0; i < this.state.content.length; i++) {
            if(this.state.content[i].type.name === menu.toString()) {
                this.state.content.splice(i, 1);
                this.setState({
                    content: this.state.content,
                });
                switch(menu) {
                    case ContentMenus.GraveyardMenu:
                        GAME_MANAGER.graveyardMenuOpen = false;
                        break;
                    case ContentMenus.PlayerListMenu:
                        GAME_MANAGER.playerListMenuOpen = false;
                        break;
                    case ContentMenus.WillMenu:
                        GAME_MANAGER.willMenuOpen = false;
                        break;
                    case ContentMenus.WikiMenu:
                        GAME_MANAGER.wikiMenuOpen = false;
                        break;
                }
                break;
            }
        }
        GAME_MANAGER.invokeStateListeners();
    }

    componentDidMount() {
        GameScreen.instance = this;
        this.setState({
            header: <PhaseRowMenu 
                phase={this.state.gameState.phase}
            />,
            content: [
                <GraveyardMenu/>,
                <ChatMenu/>,
                <PlayerListMenu/>,
                <WillMenu/>
            ],
        });
        GAME_MANAGER.addStateListener(this.listener);
    }

    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    render() {
        return (
            <div className="game-screen">
                {this.renderHeader(this.state.header)}
                {this.renderContent(this.state.content)}
            </div>
        );
    }

    renderHeader(header: JSX.Element) {
        return (
            <div className="header">
                {header}
            </div>
        );
    }

    renderContent(content: JSX.Element[]) {
        return (
            <div className="content">
                {content.map((panel, index) => {
                    return (
                        <div key={index}>
                            {panel}
                        </div>
                    );
                })}
            </div>
        );
    }
}
