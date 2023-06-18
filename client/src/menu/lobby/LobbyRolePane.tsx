import React from "react";
import GAME_MANAGER from "../../index";
import { RoleListEntry } from "../../game/gameState.d";
import "../../index.css";
import RolePicker from "../RolePicker";
import { StateListener } from "../../game/gameManager.d";
import translate from "../../game/lang";

interface RolePaneState {
    roleList: RoleListEntry[]
}

export default class LobbyRolePane extends React.Component<{}, RolePaneState> {
    listener: StateListener;

    constructor(props: {}){
        super(props);

        this.state = {
            roleList: GAME_MANAGER.gameState.roleList
        }

        this.listener = () => {
            this.setState({
                roleList: GAME_MANAGER.gameState.roleList
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
            <h2>{translate("menu.lobby.roleList")}</h2>
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
