import React from "react";
import GAME_MANAGER from "../../index";
import "../../index.css";
import { StateListener } from "../../game/gameManager.d";
import translate from "../../game/lang";
import { RoleOutline } from "../../game/roleListState.d";
import RolePicker from "../../components/RolePicker";
import ROLE_LIST_PRESETS from "./../../resources/roleListPresets.json";

type RolePaneState = {
    roleList: RoleOutline[],
    host: boolean,
    selectedRoleListPreset: string,
}

export default class LobbyRolePane extends React.Component<{}, RolePaneState> {
    listener: StateListener;

    constructor(props: {}){
        super(props);

        if(GAME_MANAGER.state.stateType === "lobby")
            this.state = {
                roleList: GAME_MANAGER.state.roleList,
                selectedRoleListPreset: Object.keys(ROLE_LIST_PRESETS)[0],
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
            roleList: roleList
        })

        GAME_MANAGER.sendSetRoleOutlinePacket(index, value);
    }
    handleRoleListPreset(){

        let roleLists = ROLE_LIST_PRESETS[this.state.selectedRoleListPreset as keyof typeof ROLE_LIST_PRESETS] as RoleOutline[][];
        if (GAME_MANAGER.state.stateType === "lobby"){
            let playerCount = GAME_MANAGER.state.players.size;

            for(; playerCount > 0; playerCount--){
                if(roleLists[playerCount] !== undefined && roleLists[playerCount] !== null){
                    break;
                }
            }

            if(roleLists[playerCount] === undefined || roleLists[playerCount] === null){
                let roleList: RoleOutline[] = [];
                for(let i = 0; i < GAME_MANAGER.state.players.size; i++){
                    roleList.push({type:"any"})
                }
                GAME_MANAGER.sendSetRoleListPacket(roleList);
            }
            else
                GAME_MANAGER.sendSetRoleListPacket(roleLists[GAME_MANAGER.state.players.size]);
        }
        
    }

    render(){return(<section className="graveyard-menu-colors">
        <h2>{translate("menu.lobby.roleList")}</h2>
        <div>
            <select
                onChange={(e)=>this.setState({selectedRoleListPreset: e.target.options[e.target.selectedIndex].value})}
                disabled={!this.state.host}
            >
                {
                    Object.keys(ROLE_LIST_PRESETS).map((value, i)=>{
                        return <option key={i} value={value}>{translate("menu.roleLists."+value)}</option>
                    })
                }
            </select>
            <button 
                onClick={(e)=>this.handleRoleListPreset()}
                disabled={!this.state.host}
            >{translate("menu.roleLists.set")}</button>

        </div>
        {this.state.roleList.map((_, index) => {
            return <RolePicker
                disabled={!this.state.host}
                roleOutline={this.state.roleList[index]}
                onChange={(value: RoleOutline) => {this.onChangeRolePicker(index, value);}}
                key={index}
            />
        })}
    </section>)}
}
