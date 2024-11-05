import { ReactElement } from "react"
import React from "react"
import translate from "../game/lang"
import { PlayerIndex } from "../game/gameState.d"
import { useGameState } from "./useHooks"
import StyledText from "./StyledText"
import Select, { SelectOptionsRecord } from "./Select"

export default function PlayerOptionDropdown(props: {
    value: PlayerIndex | null,
    onChange: (player: PlayerIndex | null) => void,
    choosablePlayers?: PlayerIndex[],
    canChooseNone?: boolean
}): ReactElement {

    let players = useGameState(
        gameState => gameState.players.map(player => [player.index, player.toString()] as [PlayerIndex, string]),
        ["gamePlayers"]
    )!;

    const optionMap: SelectOptionsRecord<PlayerIndex | "none"> = {};

    if(props.canChooseNone === true){
        optionMap["none"] = [
            <StyledText noLinks={true}>{translate("none")}</StyledText>,
            translate("none")
        ];
    }

    for (const [index, name] of players) {
        if(
            props.choosablePlayers === undefined ||
            props.choosablePlayers.includes(index)
        ){
            optionMap[index] = [
                <StyledText noLinks={true}>{name}</StyledText>,
                name
            ];
        }
    }

    return <Select
        value={props.value===null ? "none" : props.value}
        optionsSearch={optionMap}
        onChange={value => props.onChange(value === "none" ? null : value)}
    />
}