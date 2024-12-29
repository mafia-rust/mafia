import React, { ReactElement, useCallback, useContext } from "react";
import { MODIFIERS, ModifierType } from "../../game/gameState.d";
import translate from "../../game/lang";
import StyledText from "../StyledText";
import { GameModeContext } from "./GameModesEditor";
import { Button } from "../Button";

export function EnabledModifiersSelector(props: Readonly<{
    disabled: boolean,
    enabledModifiers?: ModifierType[],
    onChange?: (modifiers: ModifierType[]) => void,
}>): ReactElement {
    let { enabledModifiers } = useContext(GameModeContext);
    enabledModifiers = props.enabledModifiers ?? enabledModifiers;

    return <div className="chat-menu-colors selector-section">
        <h2>{translate("modifiers")}</h2>
        <EnabledModifiersDisplay
            disabled={props.disabled}
            modifiable={!props.disabled}
            enabledModifiers={enabledModifiers}
            onEnableModifiers={(modifiers: ModifierType[]) => {
                if (props.onChange) {
                    props.onChange(enabledModifiers.concat(modifiers))
                }
            }}
            onDisableModifiers={(modifiers: ModifierType[]) => {
                if (props.onChange) {
                    props.onChange(enabledModifiers.filter(modifier => !modifiers.includes(modifier)))
                }
            }}
        />
    </div>
}

type EnabledModifiersDisplayProps = {
    enabledModifiers: ModifierType[],
} & (
    {
        modifiable: true,
        onDisableModifiers: (modifiers: ModifierType[]) => void,
        onEnableModifiers: (modifiers: ModifierType[]) => void,
        disabled?: boolean,
    } |
    {
        modifiable?: false,
    }
)

export function EnabledModifiersDisplay(props: EnabledModifiersDisplayProps): ReactElement {
    const isEnabled = useCallback((modifier: ModifierType) => props.enabledModifiers.includes(modifier), [props.enabledModifiers]);

    const modifierTextElement = (modifier: ModifierType) => {

        return <StyledText 
            noLinks={props.modifiable ?? false}
            className={!isEnabled(modifier) ? "keyword-disabled" : undefined}
        >
            {translate(modifier)}
        </StyledText>
    }

    return <div>
        {MODIFIERS.map((modifier, i) => 
            props.modifiable 
                ? <Button key={modifier}
                    disabled={props.disabled}
                    onClick={() => (!isEnabled(modifier) ? props.onEnableModifiers : props.onDisableModifiers)([modifier])}
                >
                    {modifierTextElement(modifier)}
                </Button> 
                : <div key={modifier} className={"placard" + (!isEnabled(modifier) ? " disabled" : "")}>
                    {modifierTextElement(modifier)}
                </div>
            
        )}
    </div>
}