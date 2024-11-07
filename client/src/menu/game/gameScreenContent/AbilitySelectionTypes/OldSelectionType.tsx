import React from "react";
import { ReactElement } from "react";
import { useGameState } from "../../../../components/useHooks";
import { Player, PlayerIndex } from "../../../../game/gameState.d";

export default function OldSelectionType(): ReactElement {
    const players = useGameState(
        gameState => gameState.players,
        ["gamePlayers"]
    )!;

    return <div>
        {players.map(player => <PlayerCard key={player.index} player={player.index}/>)}
    </div>
}

function PlayerCard(props: Readonly<{
    player: PlayerIndex
}>): ReactElement {
    return <div>

    </div>
}