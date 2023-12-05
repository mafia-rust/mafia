import React from "react";
import GAME_MANAGER from "../../index";
import "../../index.css";
import { StateListener } from "../../game/gameManager.d";
import translate from "../../game/lang";
import { RoleOutline } from "../../game/roleListState.d";
import RolePicker from "../../components/RolePicker";

type RolePaneState = {
    roleList: RoleOutline[],
    host: boolean
}

export default class LobbyRolePane extends React.Component<{}, RolePaneState> {
    listener: StateListener;

    constructor(props: {}){
        super(props);

        this.state = {
            roleList: GAME_MANAGER.gameState.roleList,
            host: GAME_MANAGER.gameState.host
        }

        this.listener = () => {
            this.setState({
                roleList: GAME_MANAGER.gameState.roleList,
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

    onChangeRolePicker(index: number, value: RoleOutline){
        let roleList = [...this.state.roleList];
        roleList[index] = value;

        this.setState({
            roleList: roleList
        })

        GAME_MANAGER.sendSetRoleOutlinePacket(index, value);
    }

    render(){return(<section>
        <h2>{translate("menu.lobby.roleList")}</h2>
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
