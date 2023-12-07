import React from "react"
import GameState from "../../../../game/gameState.d"
import GAME_MANAGER from "../../../.."

type LargeForgerMenuProps = {
}
type LargeForgerMenuState = {
    gameState: GameState
}
export default class LargeForgerMenu extends React.Component<LargeForgerMenuProps, LargeForgerMenuState> {
    listener: () => void;
    constructor(props: LargeForgerMenuState) {
        super(props);

        if(GAME_MANAGER.state.stateType === "game")
            this.state = {
                gameState : GAME_MANAGER.state,
            };
        this.listener = ()=>{
            if(GAME_MANAGER.state.stateType === "game")
                this.setState({
                    gameState: GAME_MANAGER.state
                })
        };  
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    render(){
        return <div>TODO forger menu</div>
    }
}