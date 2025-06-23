import { ReactElement } from "react"
import React from "react"
import translate from "../game/lang"
import StyledText from "./StyledText"
import Select, { SelectOptionsSearch } from "./Select"
import { useContextGameState } from "../stateContext/useHooks"
import { PlayerIndex } from "../stateContext/stateType/otherState"

/// A dropdown menu for selecting a player.
/// canChooseNone defaults to false.
export default function PlayerOptionDropdown(props: {
    value: PlayerIndex | null,
    onChange: (player: PlayerIndex | null) => void,
    choosablePlayers?: PlayerIndex[],
    canChooseNone?: boolean
}): ReactElement {
    const players = useContextGameState()!
        .players
        .map(player => [player.index, player.toString()] as [PlayerIndex, string]);

    const optionMap: SelectOptionsSearch<PlayerIndex | "none"> = new Map();

    if(props.canChooseNone === true){
        optionMap.set("none", [<StyledText noLinks={true}>{translate("none")}</StyledText>, translate("none")]);
    }

    for (const [index, name] of players) {
        if(
            props.choosablePlayers === undefined ||
            props.choosablePlayers.includes(index)
        ){
            optionMap.set(index, [<StyledText noLinks={true}>{name.toString()}</StyledText>, name]);
        }
    }

    return <Select
        value={(props.value===null ?"none":props.value) as PlayerIndex | "none"}
        optionsSearch={optionMap}
        onChange={value => props.onChange(value === "none" ? null : value)}
    />
}