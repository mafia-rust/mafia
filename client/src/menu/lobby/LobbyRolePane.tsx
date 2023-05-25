import React from "react";
import GAME_MANAGER from "../../index";
import { RoleListEntry } from "../../game/gameState.d";
import "../../index.css";
import RolePicker from "../RolePicker";
import { StateEventType } from "../../game/gameManager.d";

interface RolePaneState {
    roleList: RoleListEntry[]
}

export default class LobbyRolePane extends React.Component<any, RolePaneState> {
    listener: (type: StateEventType) => void;

    constructor(props: any){
        super(props);

        this.state = {
            roleList: [...GAME_MANAGER.gameState.roleList]
        }

        this.listener = (type) => {
            console.log();
            let roleList: RoleListEntry[] = [];
            for(let i = 0; i < GAME_MANAGER.gameState.players.length; i++){
                if(i < GAME_MANAGER.gameState.roleList.length){
                    roleList.push(GAME_MANAGER.gameState.roleList[i]);
                }else
                    roleList.push({type:"any"});
            }
            this.setState({
                roleList: roleList
            });
        };
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    onChangeRolePicker(index: number, value: RoleListEntry){
        let roleList = [...this.state.roleList];
        roleList[index] = value;

        this.setState({
            roleList: roleList
        })

        GAME_MANAGER.sendSetRoleListPacket(roleList);
    }

    render(){return(<section>
        <header>
            <h2>Role list:</h2>
            <div>
                {/* TODO, role list presets */}
            </div>
        </header>
        <div> {
            this.state.roleList.map((_, index) => {
                return <RolePicker
                    roleListEntry={this.state.roleList[index]}
                    onChange={(value: RoleListEntry) => {this.onChangeRolePicker(index, value);}}
                    key={index}
                />
            })
        } </div>
    </section>)}
}
