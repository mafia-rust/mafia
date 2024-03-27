import React from "react"
import GameState from "../../../../game/gameState.d"
import GAME_MANAGER from "../../../.."
import { StateEventType } from "../../../../game/gameManager.d"
import RoleOutlineDropdown from "../../../../components/OutlineSelector"
import { RoleOutline } from "../../../../game/roleListState.d"


type LargeAmnesiacMenuProps = {
}
type LargeAmnesiacMenuState = {
    gameState: GameState,
    localRoleOutline: RoleOutline
}
export default class LargeAmnesiacMenu extends React.Component<LargeAmnesiacMenuProps, LargeAmnesiacMenuState> {
    listener: (type?: StateEventType) => void;
    constructor(props: LargeAmnesiacMenuState) {
        super(props);

        let defaultRole: RoleOutline;
        if(
            GAME_MANAGER.state.stateType === "game" &&
            GAME_MANAGER.state.clientState.type === "player" &&
            GAME_MANAGER.state.clientState.roleState?.role === "amnesiac" && 
            GAME_MANAGER.state.clientState.roleState.roleOutline!==undefined &&
            GAME_MANAGER.state.clientState.roleState.roleOutline!==null
        ){
            defaultRole = GAME_MANAGER.state.clientState.roleState.roleOutline;
        }else{
            defaultRole = {type: "any"};
        }
        
        if(GAME_MANAGER.state.stateType === "game")
            this.state = {
                gameState : GAME_MANAGER.state,
                localRoleOutline: defaultRole
            };

        this.listener = (type)=>{
            if(GAME_MANAGER.state.stateType === "game")
                this.setState({
                    gameState: GAME_MANAGER.state
                });
            if(
                GAME_MANAGER.state.stateType === "game" &&
                GAME_MANAGER.state.clientState.type === "player" &&
                type==="yourRoleState" && GAME_MANAGER.state.clientState.roleState?.role === "amnesiac"
            ){
                this.setState({
                    localRoleOutline: GAME_MANAGER.state.clientState.roleState.roleOutline
                });
            }
        };  
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    sendAndSetRole(roleOutline: RoleOutline){
        this.setState({
            localRoleOutline: roleOutline
        });
        GAME_MANAGER.sendSetAmnesiacRoleOutline(roleOutline);
    }
    render(){
        return <div>
            <RoleOutlineDropdown roleOutline={this.state.localRoleOutline} onChange={(rle)=>{this.sendAndSetRole(rle)}}/>
        </div>
    }
}