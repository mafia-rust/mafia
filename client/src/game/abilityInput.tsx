import { KiraInput } from "../menu/game/gameScreenContent/AbilityMenu/RoleSpecificMenus/KiraMenu";
import { Role } from "./roleState.d";

export type BooleanInput = boolean | null;
export type OnePlayerOptionInput = number | null;
export type TwoRoleOptionInput = [Role | null, Role | null];
export type TwoRoleOutlineOptionInput = [number | null, number | null];
export type RoleOptionSelection = Role | null;


export type AbilityInput = {
    type: "disguiser",
    input: RoleOptionSelection
} | {
    type: "auditor",
    input: TwoRoleOutlineOptionInput
} | {
    type: "steward",
    input: TwoRoleOptionInput
} | {
    type: "ojoInvestigate",
    input: TwoRoleOutlineOptionInput
} | {
    type: "kira",
    input: KiraInput
} | {
    type: "forfeitVote"
    input: BooleanInput,
} | {
    type: "pitchforkVote"
    input: OnePlayerOptionInput,
} | {
    type: "hitOrderVote"
    input: OnePlayerOptionInput,
} | {
    type: "hitOrderMafioso",
} | {
    type: "syndicateGunItemShoot",
    input: OnePlayerOptionInput,
} | {
    type: "syndicateGunItemGive",
    input: OnePlayerOptionInput,
}