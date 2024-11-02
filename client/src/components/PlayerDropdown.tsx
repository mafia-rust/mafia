import { ReactElement } from "react"
import React from "react"
import translate from "../game/lang"
import { PlayerIndex } from "../game/gameState.d"
import { useGameState } from "./useHooks"
import StyledText from "./StyledText"
import Select from "./Select"

export default function PlayerDropdown(props: {
    value: PlayerIndex | null,
    onChange: (player: PlayerIndex | null) => void,
    choosablePlayers?: PlayerIndex[],
    canChooseNone?: boolean
}): ReactElement {

    const SENTINEL_VALUE_FOR_NONE = -1;

    let players = useGameState(
        gameState => gameState.players.map(player => [player.index, player.toString()] as [PlayerIndex, string]),
        ["gamePlayers"]
    )!;

    const optionMap: Record<PlayerIndex, React.ReactNode> = {};

    if(props.canChooseNone === true){
        optionMap[-1] = <StyledText noLinks={true}>{translate("none")}</StyledText>;
    }

    for (const [index, name] of players) {
        if(
            props.choosablePlayers === undefined ||
            props.choosablePlayers.includes(index)
        ){
            optionMap[index] = <StyledText noLinks={true}>{name.toString()}</StyledText>;
        }
    }

    return <Select
        value={props.value ?? SENTINEL_VALUE_FOR_NONE}
        options={optionMap}
        onChange={value => props.onChange(value === SENTINEL_VALUE_FOR_NONE ? null : value)}
    />
}