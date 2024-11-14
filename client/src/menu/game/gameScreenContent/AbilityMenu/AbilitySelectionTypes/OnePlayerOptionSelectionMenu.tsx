import React from "react";
import { PlayerIndex } from "../../../../../game/gameState.d";
import { useGameState, usePlayerState } from "../../../../../components/useHooks";
import PlayerOptionDropdown from "../../../../../components/PlayerOptionDropdown";
import SelectionInformation from "../SelectionInformation";


export default function OnePlayerOptionSelectionMenu(props: Readonly<{
    availablePlayers?: (PlayerIndex | null)[],
    onChoose: (player: PlayerIndex | null) => void
}>){

    const selection = usePlayerState(
        playerState => playerState.targets,
        ["yourSelection"]
    )!;

    const handleSelection = (player: PlayerIndex | null) => {
        props.onChoose(player);
    }

    const playerCount = useGameState(
        gameState => gameState.players.length,
        ["gamePlayers"]
    )!;

    let canChooseNone = props.availablePlayers === undefined || props.availablePlayers.includes(null);
    let availablePlayers = props.availablePlayers?.filter(player => player !== null) as PlayerIndex[] | undefined;
    if(availablePlayers === undefined){
        availablePlayers = Array.from({length: playerCount}, (_, i) => i as PlayerIndex);
    }


    return <div>
        <SelectionInformation/>
        <PlayerOptionDropdown
            value={selection[0]===undefined ? null : selection[0]}
            onChange={handleSelection}
            canChooseNone={canChooseNone}
            choosablePlayers={availablePlayers}
        />
    </div>
}