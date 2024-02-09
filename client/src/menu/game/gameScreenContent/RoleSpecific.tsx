import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import { ContentMenu, ContentTab } from "../GameScreen";
import GameState from "../../../game/gameState.d";
import LargeDoomsayerMenu from "./RoleSpecificMenus/LargeDoomsayerMenu";
import LargeAmnesiacMenu from "./RoleSpecificMenus/LargeAmnesiacMenu";
import LargeConsortMenu from "./RoleSpecificMenus/LargeConsortMenu";
import LargeForgerMenu from "./RoleSpecificMenus/LargeForgerMenu";
import LargeJournalistMenu from "./RoleSpecificMenus/LargeJournalistMenu";

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
                return <LargeAmnesiacMenu/>;
            case "journalist":
                return <LargeJournalistMenu/>;
            case "consort":
                return <LargeConsortMenu/>;
            case "forger":
                return <LargeForgerMenu/>;
        }
    }
    render(){return(<div className="role-specific-colors">
        <ContentTab close={ContentMenu.RoleSpecificMenu} helpMenu={null}>
            {translate("role."+this.state.gameState.roleState?.role+".name")}
        </ContentTab>
        <div>
            {this.renderRoleSpecificMenu()}
        </div>
    </div>)}
}