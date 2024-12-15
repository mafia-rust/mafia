import { ReactElement } from "react";
import GAME_MANAGER from "../../../../..";
import Counter from "../../../../../components/Counter";
import React from "react";
import StyledText from "../../../../../components/StyledText";
import translate from "../../../../../game/lang";
import { PhaseType } from "../../../../../game/gameState.d";

export type RecruiterAction = "recruit" | "kill"

export default function RecruiterMenu(props: {action: RecruiterAction, remaining: number, phase: PhaseType, dayNumber: number}): ReactElement {
    return <>
        <Counter
            max={Math.max(5, props.remaining)}
            current={props.remaining}
        >
            <StyledText>{translate("role.recruiter.smallRoleMenu.recruitsRemaining", props.remaining)}</StyledText>
        </Counter>
    </>
}