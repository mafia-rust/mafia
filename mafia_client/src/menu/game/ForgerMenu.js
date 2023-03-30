import React from "react";
import GAME_MANAGER from "../..";

export class ForgerMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            gameState : gameManager.gameState,
        };
        this.listener = ()=>{
            this.setState({
                gameState: gameManager.gameState,
            })
        };  
    }

    componentDidMount() {
        gameManager.addStateListener(this.listener);
    }

    componentWillUnmount() {
        gameManager.removeStateListener(this.listener);
    }

    render() {
        return(
            <div>
            </div>
        )
    }
}