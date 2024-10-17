import { ReactElement } from "react";
import StyledText from "../../../../components/StyledText";
import React from "react";
import translate from "../../../../game/lang";
import GAME_MANAGER from "../../../..";
import { Role, roleJsonData } from "../../../../game/roleState.d";
import { usePlayerState } from "../../../../components/useHooks";
import Icon from "../../../../components/Icon";
import { Button } from "../../../../components/Button";
import Counter from "../../../../components/Counter";

export default function CounterfeiterMenu (props: {}): ReactElement {
    
    const action = usePlayerState<"forge"|"noForge">(
        playerState => playerState.roleState.type === "counterfeiter" ? playerState.roleState.action : "noForge",
        ["yourRoleState"]
    )!;
    const forgesRemaining = usePlayerState<number>(
        playerState => playerState.roleState.type === "counterfeiter" ? playerState.roleState.forgesRemaining : 0,
        ["yourRoleState"]
    )!;
    const savedRole = usePlayerState<Role>(
        playerState => playerState.roleState.type === "counterfeiter" ? playerState.roleState.fakeRole : "jester",
        ["yourRoleState"]
    )!;
    const savedWill = usePlayerState<string>(
        playerState => playerState.roleState.type === "counterfeiter" ? playerState.roleState.fakeWill : "",
        ["yourRoleState"]
    )!;

    const [localRole, setLocalRole] = React.useState<Role>(savedRole);
    const [localWill, setLocalWill] = React.useState<string>(savedWill=== "" ? "ROLE\nNight 1: \nNight 2:" : savedWill);


    const handleSave = ()=>{
        let role = localRole;
        let will = localWill;

        if(will === ""){
            will = "ROLE\nNight 1: \nNight 2:";
            setLocalWill(will);
        }


        GAME_MANAGER.sendSetForgerWill(role, will);
    }
    const handleSend = () => {
        GAME_MANAGER.sendSendMessagePacket('\n' + savedWill);
    }

    let forgerRoleOptions: JSX.Element[] = [];
    for(let role of Object.keys(roleJsonData()) as Role[]){
        forgerRoleOptions.push(
            <option key={role} value={role}>{translate("role."+role+".name")}</option>
        );
    }

    return <div className="large-forger-menu">
        <div>
            <select
                value={localRole} 
                onChange={(e)=>{
                    setLocalRole(e.target.value as Role);
                }}>
                {forgerRoleOptions}
            </select>
            <div>
                <Button
                    highlighted={localWill !== savedWill || localRole !== savedRole}
                    onClick={() => {
                        handleSave();
                        return true;
                    }}
                    pressedChildren={() => <Icon>done</Icon>}
                >
                    <Icon>save</Icon>
                </Button>
                <Button
                    onClick={() => {
                        handleSend()
                        return true;
                    }}
                    pressedChildren={() => <Icon>done</Icon>}
                >
                    <Icon>send</Icon>
                </Button>
            </div>
        </div>
        <textarea
            value={localWill}
            onChange={(e) => {
                setLocalWill(e.target.value);
            }}
            onKeyDown={(e) => {
                if (e.ctrlKey) {
                    if (e.key === 's') {
                        e.preventDefault();
                        handleSave();
                    } else if (e.key === "Enter") {
                        handleSave();
                    }
                }
            }}>
        </textarea>
        <div>
            <StyledText>
                {translate("role.counterfeiter.roleDataText")}
            </StyledText>
            <select
                value={action}
                onChange={e => {
                    GAME_MANAGER.sendSetCounterfeiterAction(e.target.value as "forge" | "noForge");
                }}
                >
                <option value={"noForge"} key={"noForge"}>{translate("noForge")}</option>
                {
                    forgesRemaining > 0 &&
                    <option value={"forge"} key={"forge"}>{translate("forge")}</option>
                }
            </select>
            <Counter max={3} current={forgesRemaining}>{translate("role.forger.menu.forgesRemaining", forgesRemaining)}</Counter>
        </div>
    </div>;
}