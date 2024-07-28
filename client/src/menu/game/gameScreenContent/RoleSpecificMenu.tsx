import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import { ContentMenu, ContentTab } from "../GameScreen";
import GameState from "../../../game/gameState.d";
import RoleSpecificSection from "../../../components/RoleSpecific";

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
        if(this.state.gameState.clientState.type !== "player") return null;
        return <RoleSpecificSection/>
    }
    render(){
        if(this.state.gameState.clientState.type === "player")
            return(
                <div className="role-specific-colors">
                    <ContentTab close={ContentMenu.RoleSpecificMenu} helpMenu={null}>
                        {translate("role."+this.state.gameState.clientState.roleState?.type+".name")}
                    </ContentTab>
                    <div>
                        {this.renderRoleSpecificMenu()}
                    </div>
                </div>
            )
    }
}