import React from "react";
import "../../index.css";
import * as LoadingScreen from "../LoadingScreen";
import PhaseRowMenu from "./PhaseRowMenu";
import GraveyardMenu from "./gameScreenContent/GraveyardMenu";
import ChatMenu from "./gameScreenContent/ChatMenu";
import PlayerListMenu from "./gameScreenContent/PlayerListMenu";
import WillMenu from "./gameScreenContent/WillMenu";
import "./gameScreen.css";
import GAME_MANAGER from "../..";
import GameState from "../../game/gameState.d";


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

    openOrCloseMenu(menu: JSX.Element) {
        for(let i = 0; i < this.state.content.length; i++) {
            if(this.state.content[i].type === menu.type) {
                this.state.content.splice(i, 1);
                this.setState({
                    content: this.state.content,
                });
                return;
            }
        }
        this.state.content.push(menu);
        this.setState({
            content: [...this.state.content],
        });
    }
            

    componentDidMount() {
        GameScreen.instance = this;
        this.setState({
            header: <PhaseRowMenu phase={this.state.gameState.phase}/>,
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
        //GameScreen.instance = undefined;
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
            <div className="game-screen-header">
                {header}
            </div>
        );
    }

    renderContent(content: JSX.Element[]) {
        return (
            <div className="game-screen-content">
                {content.map((panel, index) => {
                    return (
                        <div
                            key={index}
                            className="game-screen-panel"
                            style={{
                                gridColumn: index + 1,
                                gridRow: 1,
                            }}
                        >
                            {panel}
                        </div>
                    );
                })}
            </div>
        );
    }
}
