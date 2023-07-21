import React from "react"
import GameState from "../../../../game/gameState.d"
import GAME_MANAGER from "../../../.."
import { StateEventType } from "../../../../game/gameManager.d"
import RolePicker from "../../../../components/RolePicker"
import { RoleListEntry } from "../../../../game/roleListState.d"


type LargeAmnesiacMenuProps = {
}
type LargeAmnesiacMenuState = {
    gameState: GameState,
    localRoleListEntry: RoleListEntry
}
export default class LargeAmnesiacMenu extends React.Component<LargeAmnesiacMenuProps, LargeAmnesiacMenuState> {
    listener: (type?: StateEventType) => void;
    constructor(props: LargeAmnesiacMenuState) {
        super(props);

        let defaultRole: RoleListEntry;
        if(
            GAME_MANAGER.gameState.roleState?.role === "amnesiac" && 
            GAME_MANAGER.gameState.roleState.roleListEntry!==undefined &&
            GAME_MANAGER.gameState.roleState.roleListEntry!==null
        ){
            defaultRole = GAME_MANAGER.gameState.roleState.roleListEntry;
        }else{
            defaultRole = {type: "exact", role:"amnesiac"};
        }
        

        this.state = {
            gameState : GAME_MANAGER.gameState,
            localRoleListEntry: defaultRole
        };
        this.listener = (type)=>{
            this.setState({
                gameState: GAME_MANAGER.gameState
            });
            if(type==="yourRoleState" && GAME_MANAGER.gameState.roleState?.role === "amnesiac"){
                this.setState({
                    localRoleListEntry: GAME_MANAGER.gameState.roleState.roleListEntry
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

    sendAndSetRole(roleListEntry: RoleListEntry){
        this.setState({
            localRoleListEntry: roleListEntry
        });
        GAME_MANAGER.sendSetAmnesiacRoleListEntry(roleListEntry);
    }
    render(){
        return <div>
            <RolePicker roleListEntry={this.state.localRoleListEntry} onChange={(rle)=>{this.sendAndSetRole(rle)}}/>
        </div>
    }
}