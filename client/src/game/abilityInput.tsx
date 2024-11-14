import { GenericAbilitySelection } from "../menu/game/gameScreenContent/AbilityMenu/GenericAbilityMenu";
import { KiraInput } from "../menu/game/gameScreenContent/AbilityMenu/RoleSpecificMenus/KiraMenu";
import { Role } from "./roleState.d";

export type BooleanSelection = boolean | null;

export type OnePlayerOptionSelection = number | null;
export type AvailableOnePlayerOptionSelection = (number | null)[];

export type TwoRoleOptionSelection = [Role | null, Role | null];
export type AvailableTwoRoleOptionSelection = {
    availableRoles: (Role | null)[],
    canChooseDuplicates: boolean
};

export type TwoRoleOutlineOptionSelection = [number | null, number | null];
export type AvailableTwoRoleOutlineOptionSelection = (number | null)[];






export type AbilityInput = {
    type: "genericAbility",
    selection: GenericAbilitySelection
} | {
    type: "auditor",
    selection: TwoRoleOutlineOptionSelection
} | {
    type: "steward",
    selection: TwoRoleOptionSelection
} | {
    type: "ojoInvestigate",
    selection: TwoRoleOutlineOptionSelection
} | {
    type: "kira",
    selection: KiraInput
} | {
    type: "forfeitVote"
    selection: BooleanSelection,
} | {
    type: "pitchforkVote"
    selection: OnePlayerOptionSelection,
} | {
    type: "hitOrderVote"
    selection: OnePlayerOptionSelection,
} | {
    type: "hitOrderMafioso",
}