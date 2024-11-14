import { ReactElement } from "react";
import { AvailableOnePlayerOptionSelection, AvailableTwoRoleOptionSelection, AvailableTwoRoleOutlineOptionSelection, OnePlayerOptionSelection, TwoRoleOptionSelection, TwoRoleOutlineOptionSelection } from "../../../../game/abilityInput";
import React from "react";
import { usePlayerState } from "../../../../components/useHooks";
import { Button } from "../../../../components/Button";
import OnePlayerOptionSelectionMenu from "./AbilitySelectionTypes/OnePlayerOptionSelectionMenu";
import TwoPlayerOptionSelectionType from "./AbilitySelectionTypes/TwoPlayerOptionSelectionMenu";
import TwoRoleOutlineOptionInputMenu from "./AbilitySelectionTypes/TwoRoleOutlineOptionMenu";
import GAME_MANAGER from "../../../..";


type GenericAbilityID = number;
export type AvailableGenericAbilitySelection = {
    input: Partial<Record<GenericAbilityID, AvailableGenericAbilitySelectionType>>,
}
export type AvailableGenericAbilitySelectionType = {
    type: "unitSelection",
} | {
    type: "onePlayerOptionSelection",
    selection: AvailableOnePlayerOptionSelection,
} | {
    type: "twoRoleOptionSelection",
    selection: AvailableTwoRoleOptionSelection,
} | {
    type: "twoRoleOutlineOptionSelection",
    selection: AvailableTwoRoleOutlineOptionSelection,
}


export type GenericAbilitySelection = {
    id: GenericAbilityID,
    selectionType: GenericAbilitySelectionType
}
export type GenericAbilitySelectionType = {
    type: "unitSelection",
} | {
    type: "onePlayerOptionSelection",
    selection: OnePlayerOptionSelection
} | {
    type: "twoRoleOptionSelection",
    selection: TwoRoleOptionSelection
} | {
    type: "twoRoleOutlineOptionSelection",
    selection: TwoRoleOutlineOptionSelection
}

export default function GenericAbilityMenu(): ReactElement {

    const availableGenericAbilitySelection = usePlayerState(
        playerState => playerState.availableGenericAbilitySelection,
        ["availableGenericAbilitySelection"]
    )!;

    return <>
        Generic MENU
        {JSON.stringify(availableGenericAbilitySelection)}
        {Object.keys(availableGenericAbilitySelection.input).map((id: string) => {
            const selectionType = availableGenericAbilitySelection.input[id as any];
            if(selectionType === undefined) {return null}

            switch(selectionType.type) {
                case "unitSelection":
                    return <Button>UNIT SELECT</Button>
                case "onePlayerOptionSelection":
                    return <OnePlayerOptionSelectionMenu
                        availablePlayers={selectionType.selection}
                        onChoose={(player) => {
                            GAME_MANAGER.sendAbilityInput({
                                type: "genericAbility",
                                selection: {
                                    id: Number(id),
                                    selectionType: {
                                        type: "onePlayerOptionSelection",
                                        selection: player
                                    }
                                }
                            });
                        }}
                    />
                case "twoRoleOptionSelection":
                    return <TwoPlayerOptionSelectionType/>
                case "twoRoleOutlineOptionSelection":
                    return <TwoRoleOutlineOptionInputMenu
                        onChoose={(selection) => {
                            GAME_MANAGER.sendAbilityInput({
                                type: "genericAbility",
                                selection: {
                                    id: Number(id),
                                    selectionType: {
                                        type: "twoRoleOutlineOptionSelection",
                                        selection: selection
                                    }
                                }
                            });
                        }}
                    />
                default:
                    return null;
            }
        })
    }</>
}