import { ReactElement, useMemo, useState } from "react";
import StyledText from "../../../../components/StyledText";
import PlayerOptionDropdown from "../../../../components/PlayerOptionDropdown";
import React from "react";
import translate from "../../../../game/lang";
import GAME_MANAGER from "../../../..";
import { useGameState, usePlayerState } from "../../../../components/useHooks";
import { Button } from "../../../../components/Button";

export default function SyndicateGunItemMenu(props: Readonly<{}>): ReactElement {
    const myIndex = usePlayerState(
        playerState => playerState.myIndex,
        ["yourPlayerIndex"]
    )!;
    const shooter = usePlayerState(
        (state) => state.syndicateGunItemData.shooter,
        ["yourSyndicateGunItemData"]
    )!;
    const target = usePlayerState(
        (state) => state.syndicateGunItemData.target,
        ["yourSyndicateGunItemData"]
    )!;
    const players = useGameState(
        (state)=>state.players,
        ["gamePlayers", "playerAlive"]
    )!;

    const phaseState = useGameState(
        gameState => gameState.phaseState,
        ["phase", "playerOnTrial"]
    )!;
    const dayNumber = useGameState(
        gameState => gameState.dayNumber,
        ["phase"]
    )!;
    const insiderGroups = usePlayerState(
        playerState => playerState.insiderGroups,
        ["yourInsiderGroups"]
    )!;
    const fellowInsiders = usePlayerState(
        playerState => playerState.fellowInsiders,
        ["yourFellowInsiders"]
    )!;


    const canShootPlayers = useMemo(
        () => players
            .filter((player)=>player.alive)
            .filter((player)=>player.index !== shooter)
            .filter((player)=>player.index !== myIndex)
            .filter((player)=>!fellowInsiders.includes(player.index))
            .map((player)=>player.index),
        [players, shooter, myIndex, fellowInsiders]
    );
    

    const canGivePlayers = useMemo(
        () => players
            .filter((player)=>myIndex === shooter)
            .filter((player)=>player.alive)
            .filter((player)=>fellowInsiders.includes(player.index))
            .filter((player)=>player.index !== shooter)
            .map((player)=>player.index),
        [players, shooter, myIndex, fellowInsiders]
    );


    const [giveToPlayer, setGiveToPlayer] = React.useState<number | null>(null);
    const [open, setOpen] = useState<boolean>(true);

    if(
        phaseState.type !== "night" ||
        dayNumber === 1 ||
        !insiderGroups.includes("mafia")
    )
        return <></>;
    else
        return (<details className="role-specific-colors small-role-specific-menu" open={open}>
            <summary
                onClick={(e)=>{
                    e.preventDefault();
                    setOpen(!open);
                }}
            >{translate("syndicateGunItem")}</summary>
            <div>
                <StyledText>{translate("syndicateGunItem.description")}</StyledText>
                <div style={{
                    display:"flex",
                    flexDirection:"column",
                    alignItems:"center"
                }}>
                    {shooter===myIndex?<div>
                        {translate("kill")}
                        <PlayerOptionDropdown 
                            value={target===undefined?null:target}
                            onChange={(player)=>{
                                const input = {
                                    type: "syndicateGunItemShoot" as const,
                                    input: player
                                }
                                GAME_MANAGER.sendAbilityInput(input)
                            }}
                            choosablePlayers={canShootPlayers}
                            canChooseNone={true}
                        />
                    </div>:null}
                    {canGivePlayers.length !== 0 ? <div>
                        <Button
                            onClick={()=>{
                                const input = {
                                    type: "syndicateGunItemGive" as const,
                                    input: giveToPlayer
                                }
                                GAME_MANAGER.sendAbilityInput(input)
                            }}
                        >{translate("syndicateGunItem.give")}</Button>
                        <PlayerOptionDropdown 
                            value={giveToPlayer}
                            onChange={(player)=>{setGiveToPlayer(player)}}
                            choosablePlayers={canGivePlayers}
                            canChooseNone={true}
                        />
                    </div>:null}
                </div>
            </div>
        </details>);
}