import { ReactElement } from "react"
import React from "react"
import translate from "../game/lang"
import { PlayerIndex } from "../game/gameState.d"
import { useGameState } from "./useHooks"

export default function PlayerDropdown(props: {
    value: PlayerIndex | null,
    onChange: (player: PlayerIndex | null) => void,
    choosablePlayers?: PlayerIndex[],
    canChooseNone?: boolean
}): ReactElement {

    let players = useGameState(
        gameState => gameState.players.map(player => [player.index, player.toString()] as [PlayerIndex, string]),
        ["gamePlayers"]
    )!;

    let options = players.filter((player)=>
            props.choosablePlayers === undefined ||
            props.choosablePlayers.includes(player[0])
        )
        .map((player) => {
            return <option value={player[0]} key={player[0]}>{player[1]}</option>
        }
    );
    if(props.canChooseNone === true){
        options.unshift(<option value={"none"} key={"none"}>{translate("none")}</option>);
    }

    return <select
        value={props.value===null?"none":props.value}
        onChange={e => props.onChange(
            e.target.value==="none"?null:Number.parseInt(e.target.value)
        )}
    >
        {options}
    </select>
}