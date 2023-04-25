import React from "react";
import GAME_MANAGER from "../../index";
import { RoleListEntry } from "../../game/gameState.d";
import "../../index.css";
import RolePicker from "../RolePicker";
import { StateEventType } from "../../game/net/gameManager.d";

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
            if (type === "roleList") {
                this.setState({
                    roleList: [...GAME_MANAGER.gameState.roleList]
                })
            }
        };  
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    render(){return(<section>
        <header>
            <h2>Role list:</h2>
            <div className="settings-controls">
                {/* TODO, role list presets */}
            </div>
        </header>
        <div className="input-column"> {
            this.state.roleList.map((_, index) => {
                return <RolePicker
                    roleListEntry={this.state.roleList[index]}
                    onChange={(value: RoleListEntry) => {this.onChangeRolePicker(index, value)}}
                    key={index}
                />
            })
        } </div>
    </section>)}

    onChangeRolePicker(index: number, value: RoleListEntry){
        let roleList = [...this.state.roleList];
        roleList[index] = value;

        this.setState({
            roleList: roleList
        })

        GAME_MANAGER.sendSetRoleListPacket(roleList);
    }
}
