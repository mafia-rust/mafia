import React, { ReactElement, useMemo } from "react";
import GAME_MANAGER from "../..";
import translate from "../../game/lang";
import StyledText from "../../components/StyledText";
import { useGameState } from "../../components/useHooks";
import GraveComponent from "../../components/grave";
import { FastForwardButton } from "../game/HeaderMenu";
import Counter from "../../components/Counter";



export default function PhaseStartedScreen(): ReactElement {
    const phaseState = useGameState(
        gameState => gameState.phaseState,
        ["phase", "playerOnTrial"]
    )!

    return <div className="phase-started">
        {(() => {
            switch (phaseState.type) {
                case "briefing":
                    return <div className="briefing-screen">
                        <StyledText>{translate("phase."+phaseState.type+".subtitle")}</StyledText>
                        <FastForwardButton />
                    </div>
                case "dusk":
                case "night":
                case "discussion":
                    return <StyledText>{translate("phase."+phaseState.type+".subtitle")}</StyledText>;
                case "nomination": {
                    const votesRequired = GAME_MANAGER.getVotesRequired()!;

                    return <div className="nomination-screen player-list-menu-colors">
                        <StyledText>{
                            (votesRequired === 1 ? translate("votesRequired.1") : translate("votesRequired", votesRequired)) 
                            + " " + translate("trialsRemaining", phaseState.trialsLeft)
                        }</StyledText>
                        <Counter max={votesRequired} current={votesRequired}>
                            {translate("menu.playerList.player.votes", votesRequired)}
                        </Counter>
                    </div>
                }
                case "testimony":
                case "judgement":
                case "finalWords":
                    return <StyledText>{
                        translate("phase."+phaseState.type+".subtitle", GAME_MANAGER.getPlayerNames()[phaseState.playerOnTrial].toString())
                    }</StyledText>
                case "obituary":
                    return <ObituaryScreen />
            }
        })()}
    </div>
}

function ObituaryScreen(): ReactElement {
    const graves = useGameState(
        gameState => gameState.graves,
        ["addGrave"]
    )!

    const dayNumber = useGameState(
        gameState => gameState.dayNumber,
        ["phase"]
    )!

    const playerNames = useGameState(
        gameState => gameState.players.map(player => player.toString()),
        ["gamePlayers"]
    )!

    const newGraves = useMemo(() => {
        return graves.filter(grave => grave.diedPhase === "night" && grave.dayNumber === dayNumber - 1);
    }, [graves, dayNumber])

    if(newGraves.length === 0)
        return (
            <StyledText>{translate("nobodyDiedLastNight")}</StyledText>
        );
    return <div className="obituary-screen graveyard-menu-colors">
        {newGraves.map(grave => 
            <GraveComponent 
                key={grave.player.toString()} 
                grave={grave} 
                playerNames={playerNames}
            />
        )}
    </div>;
}
