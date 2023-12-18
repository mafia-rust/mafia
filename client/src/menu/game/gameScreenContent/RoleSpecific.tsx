import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import { ContentMenus, ContentTab } from "../GameScreen";
import "./graveyardMenu.css";
import GameState from "../../../game/gameState.d";
import LargeDoomsayerMenu from "./RoleSpecificMenus/LargeDoomsayerMenu";
import LargeAmnesiacMenu from "./RoleSpecificMenus/LargeAmnesiacMenu";

type RoleSpecificMenuProps = {
}
type RoleSpecificMenuState = {
    gameState: GameState,
}

export default class RoleSpecificMenu extends React.Component<RoleSpecificMenuProps, RoleSpecificMenuState> {
    listener: () => void;
    constructor(props: RoleSpecificMenuProps) {
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

    
    renderRoleSpecificMenu(){
        switch(this.state.gameState.roleState?.role){
            case "doomsayer":
                return <LargeDoomsayerMenu/>;
            case "amnesiac":
                return <LargeAmnesiacMenu/>
        }
    }
    render(){return(<div>
        <ContentTab close={ContentMenus.RoleSpecificMenu}>
            {translate("role."+this.state.gameState.roleState?.role+".name")}
        </ContentTab>
        <div>
            {this.renderRoleSpecificMenu()}
        </div>
    </div>)}
}