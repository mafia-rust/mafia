import { ReactElement } from "react"
import { Role } from "../../../../game/roleState.d"
import React from "react"
import RoleDropdown from "../../../../components/RoleDropdown"
import GAME_MANAGER from "../../../.."
import translate from "../../../../game/lang"
import StyledText from "../../../../components/StyledText"

export type OjoAction = {
    type: "none"
} | {
    type: "kill",
    role: Role
} | {
    type: "see",
    role: Role
}

export default function SmallOjoMenu(props: {action: OjoAction}): ReactElement {

    const sendAction = (action: OjoAction) => {
        GAME_MANAGER.sendSetOjoAction(action);
    }

    return <>
        <StyledText>{translate("role.ojo.smallRoleMenu")}</StyledText>
        <div>
            <ActionTypeDropdown action={props.action} onChange={(a)=>{sendAction(a)}}/>
            {props.action.type === "none" ? null : <RoleDropdown value={props.action.role} onChange={(role)=>{
                if(props.action.type === "none") return;
                sendAction({...props.action, role: role})
            }}/>}
        </div>
    </>
}

function ActionTypeDropdown(props: {
    action: OjoAction,
    onChange: (action: OjoAction) => void
}): ReactElement {
    return <select
        value={props.action.type}
        onChange={(e)=>{
            if(e.target.value === "none"){
                props.onChange({type: "none"});
                return;
            }

            switch(props.action.type){
                case "none":
                    props.onChange({type: e.target.value as "kill" | "see", role: "wildCard"})
                    break;
                case "see":
                case "kill":
                    props.onChange({type: e.target.value as "kill" | "see", role: props.action.role})
                    break;
            }
        }}>
            <option value="none">{translate("none")}</option>
            <option value="see">{translate("see")}</option>
            <option value="kill">{translate("kill")}</option>
    </select>
}