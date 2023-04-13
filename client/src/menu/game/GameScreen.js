import React from "react";
import "../../index.css";
import * as LoadingScreen from "../LoadingScreen";
import PhaseRowMenu from "./PhaseRowMenu";
import GraveyardMenu from "./GraveyardMenu";
import ChatMenu from "./ChatMenu";
import PlayerListMenu from "./PlayerListMenu";
import WillMenu from "./WillMenu";
import "./gameScreen.css";

export default class GameScreen extends React.Component {
    static instance;

    constructor(props) {
        super(props);
        this.state = {
            header: LoadingScreen.create(),
            content: [LoadingScreen.create()],
        };
    }

    componentDidMount() {
        GameScreen.instance = this;
        this.setState({
            header: <PhaseRowMenu/>,
            content: [
                <GraveyardMenu/>,
                <ChatMenu/>,
                <PlayerListMenu/>,
                <WillMenu/>
            ],
        });
    }

    componentWillUnmount() {
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

    renderHeader(header) {
        return (
            <div className="game-screen-header">
                {header}
            </div>
        );
    }

    renderContent(content) {
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
