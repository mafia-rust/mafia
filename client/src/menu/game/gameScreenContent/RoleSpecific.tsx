import React from "react";
import translate, { styleText } from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import GameScreen, { ContentMenus } from "../GameScreen";
import "./graveyardMenu.css";
import GameState from "../../../game/gameState.d";

type RoleSpecifcMenuProps = {
}
type RoleSpecifcMenuState = {
    gameState: GameState,
}

export default class RoleSpecifcMenu extends React.Component<RoleSpecifcMenuProps, RoleSpecifcMenuState> {
    listener: () => void;
    constructor(props: RoleSpecifcMenuProps) {
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

    render(){return(<div>
        <button onClick={()=>{GameScreen.instance.closeMenu(ContentMenus.RoleSpecificMenu)}}>
            {styleText(translate("role."+this.state.gameState.role+".name"))}
        </button>
        <div>
            TODO actual menu and everything
        </div>
    </div>)}
}