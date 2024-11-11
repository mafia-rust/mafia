import { KiraInput } from "../menu/game/gameScreenContent/AbilityMenu/RoleSpecificMenus/KiraMenu";

export type BooleanInput = boolean | null;
export type OnePlayerOptionInput = number | null;
export type TwoRoleOutlineOptionInput = [number | null, number | null];


export type AbilityInput = {
    type: "auditor",
    input: TwoRoleOutlineOptionInput
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
}