import React from "react";
import { PlayerIndex } from "../../../../../game/gameState.d";
import { usePlayerState } from "../../../../../components/useHooks";
import PlayerOptionDropdown from "../../../../../components/PlayerOptionDropdown";
import GAME_MANAGER from "../../../../..";
import SelectionInformation from "../SelectionInformation";

export default function OnePlayerOptionSelectionType(props: {}){

    const selection = usePlayerState(
        playerState => playerState.targets,
        ["yourSelection"]
    )!;

    const handleSelection = (player: PlayerIndex | null) => {
        GAME_MANAGER.sendTargetPacket(player===null ? [] : [player]);
    }

    return <div>
        <SelectionInformation/>
        <PlayerOptionDropdown
            value={selection[0]===undefined ? null : selection[0]}
            onChange={handleSelection}
            canChooseNone={true}
        />
    </div>
}