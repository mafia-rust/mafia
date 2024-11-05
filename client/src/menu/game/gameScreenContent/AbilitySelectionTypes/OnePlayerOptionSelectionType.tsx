import React from "react";
import Select, { SelectOptionsRecord } from "../../../../components/Select";
import { PlayerIndex } from "../../../../game/gameState.d";
import { useGameState } from "../../../../components/useHooks";
import StyledText from "../../../../components/StyledText";

export default function OnePlayerOptionSelectionType(props: {}){
    const options: SelectOptionsRecord<PlayerIndex> = {};

    const players = useGameState(
        gameState => gameState.players,
        ["gamePlayers"]
    )!;

    for(let playerIndex in players) {
        options[playerIndex] = [
            <StyledText noLinks={true}>{players[playerIndex].toString()}</StyledText>, 
            players[playerIndex].toString()
        ];
    }

    const [selectedPlayer, setSelectedPlayer] = React.useState<PlayerIndex | "none">("none");


    return <Select
        value={0}
        optionsSearch={options}
    />
}