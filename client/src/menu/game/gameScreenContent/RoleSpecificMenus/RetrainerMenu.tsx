import { ReactElement, useState } from "react"
import RoleDropdown from "../../../../components/RoleDropdown";
import StyledText from "../../../../components/StyledText";
import translate from "../../../../game/lang";
import React from "react";
import { Role } from "../../../../game/roleState.d";
import ROLES from "../../../../resources/roles.json";
import GAME_MANAGER from "../../../..";
import { Button } from "../../../../components/Button";
import Counter from "../../../../components/Counter";
import { useGameState, usePlayerState } from "../../../../components/useHooks";
import { PhaseType } from "../../../../game/gameState.d";

export default function RetrainerMenu(): ReactElement {

    const [role, setRole] = useState<Role>("mafiaSupportWildcard");
    const backupName = usePlayerState<string | null>(
        (playerState, gameState) => {
            if(playerState.roleState.type === "retrainer" && playerState.roleState.backup!==null){
                return gameState.players[playerState.roleState.backup].toString();
            }
            return null
        },
        ["yourRoleState", "gamePlayers"]
    )
    const phase = useGameState<PhaseType>(
        (gameState) => gameState.phaseState.type,
        ["phase"]
    );
    const alive = usePlayerState<boolean>(
        (playerState, gameState) => {
            return gameState.players[playerState.myIndex].alive
        },
        ["playerAlive", "gamePlayers", "yourPlayerIndex"]
    )

    const retrainsRemaining = usePlayerState<number>(
        (playerState, gameState)=>{
            if(playerState.roleState.type==="retrainer"){
                return playerState.roleState.retrainsRemaining;
            }else{
                return 0;
            }
        },
        ["yourRoleState"]
    );

    const allChoosableMafia : Role[] = Object.keys(ROLES).filter((rle)=>
        rle === "mafiaSupportWildcard" ||
        (
            ROLES[rle as keyof typeof ROLES].roleSet === "mafiaSupport" &&
            GAME_MANAGER.state.stateType === "game" &&
            GAME_MANAGER.state.enabledRoles.includes(rle as Role)
        )
    ).map((r)=>r as Role);

    let canRetrain = alive && ((retrainsRemaining??0) > 0) && phase !== "night" && phase !== "briefing" && backupName !== null;

    return <>
        {canRetrain && <>
            <Button
                onClick={()=>{
                    GAME_MANAGER.sendRoleActionChoice({
                        type: "retrainer",
                        action: {
                            type: "retrain",
                            role: role
                        }
                    });
                }}
            >
                <StyledText>{translate("retrain")}{backupName?(" "+backupName):""}</StyledText>
            </Button>
            <RoleDropdown
                value={role ?? "mafiaSupportWildcard"} 
                enabledRoles={allChoosableMafia}
                onChange={(rle)=>{
                    setRole(rle);
                }}
            />
        </>}
        {!canRetrain && <StyledText>{translate("role.retrainer.cannotRetrain")}</StyledText>}  
        <Counter max={2} current={retrainsRemaining??0}>{translate("role.retrainer.retrainsRemaining", retrainsRemaining??0)}</Counter>
    </>
}