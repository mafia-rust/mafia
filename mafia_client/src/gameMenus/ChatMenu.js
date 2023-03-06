import React from "react";
import gameManager from "../index.js";

export class ChatMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            gameState : gameManager.gameState,
        };
        this.listener = ()=>{
            this.setState({
                gameState: gameManager.gameState
            });
        };
    }
    componentDidMount() {
        gameManager.addStateListener(this.listener);
    }
    componentWillUnmount() {
        gameManager.removeStateListener(this.listener);
    }
    renderChatMessage(msg, i) {
        return(<div key={i}>
            {JSON.stringify(msg)}
        </div>);
    }
    render(){return(<div>
        {this.state.gameState.chatMessages.map((msg, i)=>{
            return this.renderChatMessage(msg, i);
        }, this)}
    </div>)}
}