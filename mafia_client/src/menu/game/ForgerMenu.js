import React from "react";
import GAME_MANAGER from "../..";

export default class ForgerMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            gameState : GAME_MANAGER.gameState,
        };
        this.listener = ()=>{
            this.setState({
                gameState: GAME_MANAGER.gameState,
            })
        };  
    }

    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }

    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    render() {
        return(
            <div>
            </div>
        )
    }
}