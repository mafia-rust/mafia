import React from "react";
import { PlayerIndex } from "../../../../../game/gameState.d";
import PlayerOptionDropdown from "../../../../../components/PlayerOptionDropdown";
import { AvailableTwoPlayerOptionSelection, TwoPlayerOptionSelection } from "../../../../../game/abilityInput";

export default function TwoPlayerOptionSelectionMenu(props: Readonly<{
    availableSelection: AvailableTwoPlayerOptionSelection
    selection: TwoPlayerOptionSelection,
    onChoose: (player: TwoPlayerOptionSelection) => void
}>){

    const handleSelectionFirst = (player: PlayerIndex | null) => {
        const newSelection: TwoPlayerOptionSelection = [props.selection[0], props.selection[1]];
        newSelection[0] = player;
        props.onChoose(newSelection);
    }
    
    const handleSelectionSecond = (player: PlayerIndex | null) => {
        const newSelection: TwoPlayerOptionSelection = [props.selection[0], props.selection[1]];
        newSelection[1] = player;
        props.onChoose(newSelection);
    }

    return <div>
        <PlayerOptionDropdown
            value={props.selection[0]}
            onChange={handleSelectionFirst}
            canChooseNone={true}
        />
        <PlayerOptionDropdown
            value={props.selection[1]}
            onChange={handleSelectionSecond}
            canChooseNone={true}
        />
    </div>
}