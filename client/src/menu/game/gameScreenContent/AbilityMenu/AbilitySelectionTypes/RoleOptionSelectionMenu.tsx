import { ReactElement } from "react";
import { Role } from "../../../../../game/roleState.d";
import RoleDropdown from "../../../../../components/RoleDropdown";
import React from "react";

export default function RoleOptionSelectionMenu(props: Readonly<{
    selection: Role | null,
    onChoose: (role: Role | null)=>void,
}>): ReactElement {
    return <div>
        <RoleDropdown
            value={props.selection}
            onChange={props.onChoose}
            canChooseNone={true}
        />
    </div>;
}