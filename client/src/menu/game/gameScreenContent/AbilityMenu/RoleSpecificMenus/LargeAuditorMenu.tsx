import { ReactElement } from "react"
import React from "react"
import "./largeAuditorMenu.css"
import GAME_MANAGER from "../../../../.."
import { Role } from "../../../../../game/roleState.d"
import { RoleList, translateRoleOutline } from "../../../../../game/roleListState.d"
import StyledText from "../../../../../components/StyledText"
import translate from "../../../../../game/lang"
import { useGameState, usePlayerState } from "../../../../../components/useHooks"
import { Button } from "../../../../../components/Button"

export type AuditorResult = {
    type: "two",
    roles: [Role, Role]
} | {
    type: "one",
    role: Role
}
type AuditorButtons = ({
    type:"notUsed",
    chosen: boolean
} | {
    type:"used",
    result: AuditorResult
})[]

export default function LargeAuditorMenu(props: {}): ReactElement {
    const previouslyGivenResults = usePlayerState(
        (playerState, gameState)=>{
            if(playerState.roleState?.type === "auditor"){
                return playerState.roleState.previouslyGivenResults;
            }
            return [];
        },
        ["yourRoleState"]
    )!;
    const chosenOutlines = usePlayerState(
        (playerState, gameState)=>{
            if(playerState.roleState?.type === "auditor"){
                return playerState.roleState.chosenOutline;
            }
            return null;
        },
        ["yourRoleState"]
    )!;
    const roleList = useGameState(
        (gameState)=>{
            return gameState.roleList;
        },
        ["roleList"]
    )!;


    
    const buttons: AuditorButtons = [];
    for(let i = 0; i < roleList.length; i++){
        const found = previouslyGivenResults.find(result=>result[0] === i);
        if(found !== undefined){
            buttons.push({type:"used" as "used", result: found[1]});
        }else{
            buttons.push({type:"notUsed"  as "notUsed", chosen: chosenOutlines.includes(i)});
        }
    }
    
    
    return <div className="large-auditor-menu">
        <div className="grid">
            <RoleListDisplay
                roleList={roleList}
                strikenOutlineIndexs={
                    previouslyGivenResults.map(result=>{return result[0] as number})
                }
            />
            <ChooseButtons
                buttons={buttons}
                chosenOutlines={chosenOutlines}
            />
        </div>
    </div>
}
function RoleListDisplay(props: {
    roleList: RoleList,
    strikenOutlineIndexs: number[],
}): ReactElement {
    return <>
        {props.roleList.map((entry, index)=>{
            return <Button 
                className="role-list-button"
                style={{ gridRow: index + 1 }} 
                key={index}
            >
                {
                    props.strikenOutlineIndexs.includes(index) ? 
                    <s><StyledText>
                        {translateRoleOutline(entry) ?? ""}
                    </StyledText></s> : 
                    <StyledText>
                        {translateRoleOutline(entry) ?? ""}
                    </StyledText>
                }
            </Button>
        })}
    </>
}
function ChooseButtons(props: Readonly<{
    //true means its selected, false means its not selected
    buttons: AuditorButtons,
    chosenOutlines: [number | null, number | null]
}>): ReactElement {
    return<> {
        props.buttons.map((button, index)=>{
            if(button.type === "notUsed"){
                return <Button 
                    className={"choose-button" + (button.chosen ? " highlighted" : "")}
                    key={index}
                    onClick={()=>{
                        let newChosenOutlines = [...props.chosenOutlines];
                        newChosenOutlines = newChosenOutlines.filter((outline)=>outline !== null);

                        if(newChosenOutlines.includes(index)){
                            newChosenOutlines = newChosenOutlines.filter((outline)=>outline !== index);
                        }else{
                            newChosenOutlines.unshift(index);
                        }

                        //use for loops instead of while loops to prevent infinite loops and tomfoolery
                        for(let i = 0; newChosenOutlines.length > 2 && i < 100; i++){
                            newChosenOutlines.pop();
                        }
                        for(let i = 0; newChosenOutlines.length < 2 && i < 100; i++){
                            newChosenOutlines.push(null);
                        }

                        const input: AbilityInput = {
                            type: "auditor",
                            input: newChosenOutlines as [number | null, number | null]
                        };
                        GAME_MANAGER.sendAbilityInput(input);
                    }}
                >
                    <StyledText>
                        {translate("choose")}
                    </StyledText>
                </Button>
            }else{
                if(button.result.type === "one"){
                    return <label
                        className="choose-button"
                        key={index}
                    >
                        <StyledText>
                            {translate("role."+button.result.role+".name")}
                        </StyledText>
                    </label>
                }else{
                    return <label
                        className="choose-button"
                        key={index}
                    >
                        <StyledText>
                            {translate("role."+button.result.roles[0]+".name")} {translate("role."+button.result.roles[1]+".name")}
                        </StyledText>
                    </label>
                }
            }
        })
    }</>
}