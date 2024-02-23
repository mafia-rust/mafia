import React, { ReactElement, useEffect, useState } from "react";
import GAME_MANAGER from "../../index";
import "../../index.css";
import { StateListener } from "../../game/gameManager.d";
import { RoleOutline } from "../../game/roleListState.d";
import { OutlineListSelector } from "../../components/RolePicker";

export default function LobbyRolePane(): ReactElement {

    const [roleList, setRoleList] = useState(
        GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game" ? GAME_MANAGER.state.roleList : []
    );
    const [host, setHost] = useState(
        GAME_MANAGER.getMyHost() ?? false
    );

    useEffect(() => {
        const listener: StateListener = (type) => {
            if(GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game"){
                switch (type) {
                    case "roleList":
                        setRoleList([...GAME_MANAGER.state.roleList]);
                        break;
                    case "roleOutline":
                        setRoleList([...GAME_MANAGER.state.roleList]);
                        break;
                    case "playersHost":
                        setHost(GAME_MANAGER.getMyHost() ?? false);
                        break;
                }
            }
        }

        if(GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game"){
            setRoleList([...GAME_MANAGER.state.roleList]);
            setHost(GAME_MANAGER.getMyHost() ?? false);
        }

        GAME_MANAGER.addStateListener(listener);
        return ()=>{GAME_MANAGER.removeStateListener(listener);}
    }, [setRoleList, setHost]);



    let onChangeRolePicker = (value: RoleOutline, index: number) => {
        let newRoleList = [...roleList];
        newRoleList[index] = value;
        setRoleList(newRoleList);
        GAME_MANAGER.sendSetRoleOutlinePacket(index, value);
    }

    return <OutlineListSelector
        disabled={!host}
        roleList={roleList}
        onChangeRolePicker={onChangeRolePicker}
        onAddNewOutline={undefined}
        onRemoveOutline={undefined}
    />
}
