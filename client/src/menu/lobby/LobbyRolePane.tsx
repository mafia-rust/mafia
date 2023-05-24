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
            if(this.state.roleList.length > GAME_MANAGER.gameState.players.length){
                this.setState({
                    roleList: [...GAME_MANAGER.gameState.roleList].slice(
                        0, GAME_MANAGER.gameState.players.length
                    )
                }, ()=>{
                    GAME_MANAGER.sendSetRoleListPacket(this.state.roleList);
                });
            }
            else if(
                type === "roleList" || 
                this.state.roleList.length !== GAME_MANAGER.gameState.roleList.length ||
                this.state.roleList.length !== GAME_MANAGER.gameState.players.length
                ){
                this.setState({
                    roleList: [...GAME_MANAGER.gameState.roleList]
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
