
import React from "react";
import GAME_MANAGER from "../../index";
import { RoleOutline } from "../../game/roleListState.d";
import "../../index.css";
import { StateListener } from "../../game/gameManager.d";
import { Role } from "../../game/roleState.d";
import DisabledRoleSelector from "../../components/DisabledRoleSelector";

type ExcludedRolesState = {
    excludedRoles: Role[],
    roleOutline: RoleOutline,
    host: boolean
}


export default class LobbyExcludedRoles extends React.Component<{}, ExcludedRolesState> {
    listener: StateListener;

    constructor(props: {}){
        super(props);

        if(GAME_MANAGER.state.stateType === "lobby")
            this.state = {
                excludedRoles: GAME_MANAGER.state.excludedRoles,
                roleOutline: {type:"any"},
                host: GAME_MANAGER.getMyHost() ?? false
            }

        this.listener = (type) => {
            if(GAME_MANAGER.state.stateType === "lobby")
                this.setState({
                    excludedRoles: GAME_MANAGER.state.excludedRoles,
                    host: GAME_MANAGER.getMyHost() ?? false
                });
        };
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }
    

    render(){return(
        <DisabledRoleSelector
            disabledRoles={this.state.excludedRoles}
            onDisableRoles={(roles)=>{GAME_MANAGER.sendExcludedRolesPacket([...this.state.excludedRoles, ...roles])}}
            onEnableRoles={(roles)=>{GAME_MANAGER.sendExcludedRolesPacket(this.state.excludedRoles.filter((role)=>!roles.includes(role)))}}
            onIncludeAll={()=>{GAME_MANAGER.sendExcludedRolesPacket([])}}
            disabled={!this.state.host}
        />
    )}
}
