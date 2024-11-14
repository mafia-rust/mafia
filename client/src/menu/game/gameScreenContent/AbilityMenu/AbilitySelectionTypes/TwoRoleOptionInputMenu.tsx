import { ReactElement } from "react";
import { TwoRoleOptionSelection } from "../../../../../game/abilityInput";
import React from "react";
import RoleDropdown from "../../../../../components/RoleDropdown";
import { getAllRoles } from "../../../../../game/roleListState.d";
import { Role } from "../../../../../game/roleState.d";

export default function TwoRoleOptionInputMenu(props: Readonly<{
    input: TwoRoleOptionSelection,
    disabledRoles?: Role[], 
    onChoose: (input: TwoRoleOptionSelection)=>void
}>): ReactElement {

    const handleOnChange = (index: 0 | 1, roleOption: Role | null) => {
        const newInput: TwoRoleOptionSelection = [props.input[0], props.input[1]];
        newInput[index] = roleOption;
        props.onChoose(newInput);
    }

    const enabledRoles = props.disabledRoles === undefined ? getAllRoles() :
        getAllRoles().filter(role=>!props.disabledRoles!.includes(role));

    return <div>
        <RoleDropdown
            enabledRoles={enabledRoles}
            value={props.input[0]}
            onChange={(roleOption)=>{handleOnChange(0, roleOption)}}
            canChooseNone={true}
        />
        <RoleDropdown
            enabledRoles={enabledRoles}
            value={props.input[1]}
            onChange={(roleOption)=>{handleOnChange(1, roleOption)}}
            canChooseNone={true}
        />
    </div>;
}