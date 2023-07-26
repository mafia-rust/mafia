
import React from "react";
import GAME_MANAGER from "../../index";
import { RoleOutline, translateRoleOutline } from "../../game/roleListState.d";
import "../../index.css";
import { StateListener } from "../../game/gameManager.d";
import translate from "../../game/lang";
import RolePicker from "../../components/RolePicker";
import StyledText from "../../components/StyledText";

interface ExcludedRolesState {
    excludedRoles: RoleOutline[],
    roleOutline: RoleOutline,
    host: boolean
}

export default class LobbyExcludedRoles extends React.Component<{}, ExcludedRolesState> {
    listener: StateListener;

    constructor(props: {}){
        super(props);

        this.state = {
            excludedRoles: GAME_MANAGER.gameState.excludedRoles,
            roleOutline: {type:"any"},
            host: GAME_MANAGER.gameState.host
        }

        this.listener = () => {
            this.setState({
                excludedRoles: GAME_MANAGER.gameState.excludedRoles,
                host: GAME_MANAGER.gameState.host
            });
        };
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    includeRole(role: RoleOutline){
        let roles = [...this.state.excludedRoles];
        roles = roles.filter((value)=>value !== role);
        GAME_MANAGER.sendExcludedRolesPacket(roles);
    }
    excludeRole(){
        let roles = [...this.state.excludedRoles];
        roles.push(this.state.roleOutline);
        GAME_MANAGER.sendExcludedRolesPacket(roles);
    }

    

    render(){return(<section className="excluded-roles">
        <header>
            <h2>{translate("menu.lobby.excludedRoles")}</h2>
        </header>
        <div>
            <RolePicker
                disabled={!this.state.host}
                roleOutline={this.state.roleOutline}
                onChange={(value: RoleOutline) => {
                    this.setState({
                        roleOutline: value
                    })
                }}
            />
            <button 
                disabled={!this.state.host}
                onClick={()=>{this.excludeRole()}}
            >{translate("menu.excludedRoles.exclude")}</button>
        </div>
        <div>
            {this.state.excludedRoles.map((value, i)=>{
                return <button key={i} 
                    disabled={!this.state.host}
                    onClick={()=>{this.includeRole(value)}}
                >
                    <StyledText noLinks={!this.state.host}>
                        {translateRoleOutline(value) ?? ""}
                    </StyledText>
                </button>
            })}
        </div>
    </section>)}
}
