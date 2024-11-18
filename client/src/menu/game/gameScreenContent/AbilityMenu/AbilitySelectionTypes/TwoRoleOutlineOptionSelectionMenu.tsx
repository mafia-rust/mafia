import { ReactElement } from "react"
import React from "react"
import "./twoRoleOutlineOptionSelectionMenu.css"
import { Role } from "../../../../../game/roleState.d"
import { RoleList, translateRoleOutline } from "../../../../../game/roleListState.d"
import StyledText from "../../../../../components/StyledText"
import translate from "../../../../../game/lang"
import { useGameState } from "../../../../../components/useHooks"
import { Button } from "../../../../../components/Button"
import { TwoRoleOutlineOptionSelection } from "../../../../../game/abilityInput"

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

export default function TwoRoleOutlineOptionSelectionMenu(props: {
    chosenOutlines?: TwoRoleOutlineOptionSelection,
    previouslyGivenResults?: [number, AuditorResult][],
    onChoose: (chosenOutlines: TwoRoleOutlineOptionSelection)=>void
}): ReactElement {
    const roleList = useGameState(
        (gameState)=>{
            return gameState.roleList;
        },
        ["roleList"]
    )!;

    const previouslyGivenResults = props.previouslyGivenResults ?? [];
    const chosenOutlines = props.chosenOutlines ?? [null, null];
    
    const buttons: AuditorButtons = [];
    for(let i = 0; i < roleList.length; i++){
        const found = previouslyGivenResults.find(result=>result[0] === i);
        if(found !== undefined){
            buttons.push({type:"used" as "used", result: found[1]});
        }else{
            buttons.push({type:"notUsed"  as "notUsed", chosen: chosenOutlines.includes(i)});
        }
    }
    
    return <div className="two-role-outline-option-input">
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
                onChoose={props.onChoose}
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
    chosenOutlines: [number | null, number | null],
    onChoose: (chosenOutlines: [number | null, number | null])=>void
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
                        props.onChoose(newChosenOutlines as [number | null, number | null]);
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