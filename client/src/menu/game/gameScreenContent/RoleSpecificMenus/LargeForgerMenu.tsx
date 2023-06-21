import React from "react"
import GameState from "../../../../game/gameState.d"
import GAME_MANAGER from "../../../.."
import translate, { styleText } from "../../../../game/lang"

type LargeForgerMenuProps = {
}
type LargeForgerMenuState = {
    gameState: GameState
}
export default class LargeForgerMenu extends React.Component<LargeForgerMenuProps, LargeForgerMenuState> {
    listener: () => void;
    constructor(props: LargeForgerMenuState) {
        super(props);

        this.state = {
            gameState : GAME_MANAGER.gameState,
        };
        this.listener = ()=>{
            this.setState({
                gameState: GAME_MANAGER.gameState
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