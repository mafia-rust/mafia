import { ReactElement } from "react";
import { RoleState } from "../../../../../game/roleState.d";
import Counter from "../../../../../components/Counter";
import React from "react";
import StyledText from "../../../../../components/StyledText";
import translate from "../../../../../game/lang";


export default function StewardMenu(props: Readonly<{
        roleState: RoleState & {type: "steward"}
}>): ReactElement | null {
    return <Counter max={1} current={props.roleState.stewardProtectsRemaining}><StyledText>{translate("role.steward.roleDataText", props.roleState.stewardProtectsRemaining)}</StyledText></Counter>
}