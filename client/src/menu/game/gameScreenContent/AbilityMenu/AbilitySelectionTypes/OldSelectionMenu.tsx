import React, { useState } from "react";
import { ReactElement } from "react";
import { usePlayerState } from "../../../../../components/useHooks";
import { PlayerIndex } from "../../../../../game/gameState.d";
import translate, { translateAny } from "../../../../../game/lang";
import { Button } from "../../../../../components/Button";
import GAME_MANAGER from "../../../../..";
import { RoleState } from "../../../../../game/roleState.d";
import PlayerNamePlate from "../../../../../components/PlayerNamePlate";
import "./oldSelectionMenu.css";
import SelectionInformation from "../SelectionInformation";
import RoleSpecificSection from "../RoleSpecific";

export default function OldSelectionType(): ReactElement {
    const useablePlayers = usePlayerState(
        (playerState, gameState) => gameState.players
            .filter((player) => player.buttons.target || player.buttons.dayTarget)
            .map((player) => player.index),
        ["yourButtons"]
    )!;

    const roleState = usePlayerState(
        playerState => playerState.roleState,
        ["yourRoleState"]
    )!;
    
    const [roleSpecificOpen, setRoleSpecificOpen] = useState<boolean>(true);
            
    return <details className="role-specific-colors small-role-specific-menu" open={roleSpecificOpen}>
        <summary
            onClick={(e)=>{
                e.preventDefault();
                setRoleSpecificOpen(!roleSpecificOpen);
            }}
        >
            {translate("role."+roleState?.type+".name")}
        </summary>
        <RoleSpecificSection/>
        <SelectionInformation />
        {useablePlayers.length !== 0 && 
            <div className="old-selection-type">
                {useablePlayers.map(idx => <PlayerCard key={idx} playerIndex={idx}/>)}
            </div>
        }
    </details>
}

function useSelectedPlayers(): PlayerIndex[] {
    return usePlayerState(
        (playerState, gameState) => gameState.phaseState.type === "night" ? playerState.targets : [],
        ["phase", "yourSelection"],
        []
    )!;
}

function useDayTargetedPlayers(): PlayerIndex[] {
    const roleState = usePlayerState(
        playerState => playerState.roleState,
        ["yourRoleState"],
    );

    switch (roleState?.type){
        case "jailor":
        case "kidnapper":
            if (roleState.jailedTargetRef !== null) return [roleState.jailedTargetRef]
            break;
        case "medium":
            if (roleState.seancedTarget !== null) return [roleState.seancedTarget]
            break;
        case "reporter":
            if (roleState.interviewedTarget !== null) return [roleState.interviewedTarget]
            break;
    }

    return []
}

function PlayerCard(props: Readonly<{
    playerIndex: PlayerIndex
}>): ReactElement{
    const chosenPlayers = useSelectedPlayers()
        .concat(useDayTargetedPlayers())

    return <div 
        className={`player-card ${chosenPlayers.includes(props.playerIndex) ? "highlighted" : ""}`}
        key={props.playerIndex}
    >
        <PlayerNamePlate playerIndex={props.playerIndex}/>
        <PlayerButtons playerIndex={props.playerIndex}/>
    </div>
}

function PlayerButtons(props: Readonly<{
    playerIndex: PlayerIndex
}>): ReactElement {

    const buttons = usePlayerState(
        (playerState, gameState) => gameState.players[props.playerIndex].buttons,
        ["yourButtons"]
    )!;
    const roleState = usePlayerState(
        playerState => playerState.roleState,
        ["yourRoleState"]
    )!;

    return <div>
        {buttons.dayTarget && <DayTargetButton playerIndex={props.playerIndex} roleState={roleState}/>}
        <TargetButton playerIndex={props.playerIndex} roleState={roleState} buttons={buttons}/>
    </div>
}

function DayTargetButton(props: Readonly<{
    playerIndex: PlayerIndex,
    roleState: RoleState | undefined
}>): ReactElement {
    return <Button 
        highlighted={useDayTargetedPlayers().includes(props.playerIndex)} 
        onClick={()=>GAME_MANAGER.sendDayTargetPacket(props.playerIndex)}
    >
        {translateAny(["role."+props.roleState?.type+".dayTarget", "dayTarget"])}
    </Button>
}

function TargetButton(props: Readonly<{
    playerIndex: PlayerIndex,
    roleState: RoleState | undefined,
    buttons: {
        dayTarget: boolean,
        target: boolean,
        vote: boolean,
    }
}>): ReactElement | null {
    const targets = usePlayerState(
        playerState => playerState.targets,
        ["yourSelection"]
    )!;

    const selectedPlayers = useSelectedPlayers();

    if(props.buttons.target) {
        return <Button onClick={() => GAME_MANAGER.sendTargetPacket([...targets, props.playerIndex])}>
            {translateAny(["role."+props.roleState?.type+".target", "target"])}
        </Button>
    } else if (selectedPlayers.includes(props.playerIndex)) {
        let newTargets = [...targets];
        newTargets.splice(newTargets.indexOf(props.playerIndex), 1);
        return <Button highlighted={true} onClick={() => GAME_MANAGER.sendTargetPacket(newTargets)}>
            {translate("cancel")}
        </Button>
    } else {
        return null;
    }
}