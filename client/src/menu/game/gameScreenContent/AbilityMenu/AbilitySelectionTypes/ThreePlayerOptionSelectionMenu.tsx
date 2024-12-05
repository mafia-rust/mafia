import React from "react";
import { PlayerIndex } from "../../../../../game/gameState.d";
import PlayerOptionDropdown from "../../../../../components/PlayerOptionDropdown";
import { AvailableThreePlayerOptionSelection, ThreePlayerOptionSelection } from "../../../../../game/abilityInput";

export default function ThreePlayerOptionSelectionMenu(props: Readonly<{
    availableSelection: AvailableThreePlayerOptionSelection
    selection: ThreePlayerOptionSelection,
    onChoose: (player: ThreePlayerOptionSelection) => void
}>){

    const handleSelection = (player: PlayerIndex | null, index: number) => {
        const newSelection: ThreePlayerOptionSelection = [props.selection[0], props.selection[1], props.selection[2]];
        newSelection[index] = player;
        props.onChoose(newSelection);
    }

    return <div>
        <PlayerOptionDropdown
            value={props.selection[0]}
            onChange={(p)=>handleSelection(p,0)}
            choosablePlayers={props.availableSelection.availableFirstPlayers.filter((p)=>p!==null) as number[]}
            canChooseNone={props.availableSelection.availableFirstPlayers.includes(null)}
        />
        <PlayerOptionDropdown
            value={props.selection[1]}
            onChange={(p)=>handleSelection(p,1)}
            choosablePlayers={props.availableSelection.availableSecondPlayers.filter((p)=>p!==null) as number[]}
            canChooseNone={props.availableSelection.availableSecondPlayers.includes(null)}
        />
        <PlayerOptionDropdown
            value={props.selection[2]}
            onChange={(p)=>handleSelection(p,2)}
            choosablePlayers={props.availableSelection.availableThirdPlayers.filter((p)=>p!==null) as number[]}
            canChooseNone={props.availableSelection.availableThirdPlayers.includes(null)}
        />
    </div>
}