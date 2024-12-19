import { ReactElement } from "react";
import Counter from "../../../../../components/Counter";
import React from "react";
import StyledText from "../../../../../components/StyledText";
import translate from "../../../../../game/lang";
import { PhaseType } from "../../../../../game/gameState.d";

export default function SmallPuppeteerMenu(props: {marionettesRemaining: number, phase: PhaseType}): ReactElement {
    return <>
        <Counter
            max={3}
            current={props.marionettesRemaining}
        >
            <StyledText>{translate("role.puppeteer.smallRoleMenu.marionettesRemaining", props.marionettesRemaining)}</StyledText>
        </Counter>
    </>
}