import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import { ContentMenus, ContentTab } from "../GameScreen";
import "./graveyardMenu.css";
import GameState from "../../../game/gameState.d";
import LargeDoomsayerMenu from "./RoleSpecificMenus/LargeDoomsayerMenu";

type RoleSpecificMenuProps = {
}
type RoleSpecificMenuState = {
    gameState: GameState,
}

export default class RoleSpecificMenu extends React.Component<RoleSpecificMenuProps, RoleSpecificMenuState> {
    listener: () => void;
    constructor(props: RoleSpecificMenuProps) {
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

    
    renderRoleSpecificMenu(){
        switch(this.state.gameState.role){
            case "doomsayer":
                return <LargeDoomsayerMenu/>;
        }
    }
    render(){return(<div>
        <ContentTab close={ContentMenus.RoleSpecificMenu}>
            {translate("role."+this.state.gameState.role+".name")}
        </ContentTab>
        <div>
            {this.renderRoleSpecificMenu()}
        </div>
    </div>)}
}