import React from "react"
import GameState from "../../../../game/gameState.d"
import GAME_MANAGER from "../../../.."
import translate, { styleText } from "../../../../game/lang"

type LargeDoomsayerMenuProps = {
}
type LargeDoomsayerMenuState = {
    gameState: GameState
}
export default class LargeDoomsayerMenu extends React.Component<LargeDoomsayerMenuProps, LargeDoomsayerMenuState> {
    listener: () => void;
    constructor(props: LargeDoomsayerMenuState) {
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
        return <div>TODO doomsayer menu</div>
    }
}