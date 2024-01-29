import React from "react";
import GAME_MANAGER from "../../index";
import "../../index.css";
import { StateListener } from "../../game/gameManager.d";
import translate from "../../game/lang";
import { RoleOutline } from "../../game/roleListState.d";
import RoleOutlineSelector from "../../components/RolePicker";
// import ROLE_LIST_PRESETS from "./../../resources/roleListPresets.json";

type RolePaneState = {
    roleList: RoleOutline[],
    host: boolean,
}

export default class LobbyRolePane extends React.Component<{}, RolePaneState> {
    listener: StateListener;

    constructor(props: {}){
        super(props);

        if(GAME_MANAGER.state.stateType === "lobby")
            this.state = {
                roleList: GAME_MANAGER.state.roleList,
                host: GAME_MANAGER.getMyHost() ?? false
            }

        this.listener = () => {
            if(GAME_MANAGER.state.stateType === "lobby"){
                
                this.setState({
                    host: GAME_MANAGER.getMyHost() ?? false
                });

                if(GAME_MANAGER.state.roleList !== this.state.roleList){
                    this.setState({
                        roleList: GAME_MANAGER.state.roleList,
                    });

                }
            }
                
        };
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    onChangeRolePicker(index: number, value: RoleOutline){
        let roleList = [...this.state.roleList];
        roleList[index] = value;

        this.setState({
            roleList
        })

        GAME_MANAGER.sendSetRoleOutlinePacket(index, value);
    }
    
    render(){
        return(<section className="graveyard-menu-colors">
        <h2>{translate("menu.lobby.roleList")}</h2>
        <button
            onClick={()=>{
                GAME_MANAGER.sendSimplifyRoleListPacket();
            }}>
            {translate("simplify")}
        </button>
        {this.state.roleList.map((outline, index) => {
            return <RoleOutlineSelector
                disabled={!this.state.host}
                roleOutline={outline}
                onChange={(value: RoleOutline) => {this.onChangeRolePicker(index, value);}}
                key={index}
            />
        })}
    </section>)}
}
