import { ReactElement } from "react";
import GAME_MANAGER from "../../../../..";
import { usePlayerState } from "../../../../../components/useHooks";
import { RoleState } from "../../../../../game/roleState.d";
import Counter from "../../../../../components/Counter";
import React from "react";
import { getAllRoles } from "../../../../../game/roleListState.d";
import StyledText from "../../../../../components/StyledText";
import translate from "../../../../../game/lang";
import TwoRoleOptionSelectionMenu from "../AbilitySelectionTypes/TwoRoleOptionSelectionMenu";



export default function StewardMenu(
    props: {
        roleState: RoleState & {type: "steward"}
    }
): ReactElement | null {

    const shouldDisplay = usePlayerState(
        (playerState, gameState) => gameState.phaseState.type === "night" && gameState.players[playerState.myIndex]?.alive,
        ["playerAlive", "yourPlayerIndex", "phase", "gamePlayers"]
    )!;

    if (!shouldDisplay) {
        return null;
    }

    const availableRoles = getAllRoles()
    .filter(role=>
        !(role === "steward" && props.roleState.stewardProtectsRemaining === 0) &&
        !(props.roleState.previousRoleChosen?.includes(role))
    );

    const availableSelection = {
        availableRoles: availableRoles,
        canChooseDuplicates: false
    };

    return <>
        <Counter max={1} current={props.roleState.stewardProtectsRemaining}><StyledText>{translate("role.steward.roleDataText", props.roleState.stewardProtectsRemaining)}</StyledText></Counter>
        <TwoRoleOptionSelectionMenu 
            input={props.roleState.roleChosen}
            availableSelection={availableSelection}
            onChoose={(input)=>{
                GAME_MANAGER.sendAbilityInput({
                    type: "steward",
                    selection: input
                });
            }}
        />

    </>
}