import React from "react";
import { PlayerIndex } from "../../../../game/gameState.d";
import { usePlayerState } from "../../../../components/useHooks";
import PlayerOptionDropdown from "../../../../components/PlayerOptionDropdown";
import GAME_MANAGER from "../../../..";
import SelectionInformation from "../../../../components/SelectionInformation";

export default function TwoPlayerOptionSelectionType(props: {}){

    const selection = usePlayerState(
        playerState => playerState.targets,
        ["yourSelection"]
    )!;

    const handleSelectionFirst = (player: PlayerIndex | null) => {
        if (selection.length === 0 || selection.length === 1){
            GAME_MANAGER.sendTargetPacket(player===null ? [] : [player]);
        }else if(player !== null){
            GAME_MANAGER.sendTargetPacket([player, selection[1]]);
        }else{
            GAME_MANAGER.sendTargetPacket([selection[1]]);
        }
    }
    const handleSelectionSecond = (player: PlayerIndex | null) => {
        if ((selection.length === 1 || selection.length === 2) && player !== null){
            GAME_MANAGER.sendTargetPacket([selection[0], player]);
        }else if(selection.length === 1 && player === null){
            GAME_MANAGER.sendTargetPacket([selection[0]]);
        }else{
            GAME_MANAGER.sendTargetPacket([]);
        }
    }

    return <div>
        <SelectionInformation/>
        <PlayerOptionDropdown
            value={selection[0]===undefined ? null : selection[0]}
            onChange={handleSelectionFirst}
            canChooseNone={true}
        />
        {
            selection.length >= 1 && 
            <PlayerOptionDropdown
                value={selection[1]===undefined ? null : selection[1]}
                onChange={handleSelectionSecond}
                canChooseNone={true}
            />
        }
    </div>
}