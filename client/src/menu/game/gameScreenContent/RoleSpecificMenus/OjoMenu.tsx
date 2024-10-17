import { ReactElement, useEffect, useState } from "react"
import { Role, RoleState } from "../../../../game/roleState.d"
import React from "react"
import RoleDropdown from "../../../../components/RoleDropdown"
import GAME_MANAGER from "../../../.."
import translate from "../../../../game/lang"
import StyledText from "../../../../components/StyledText"
import { useGameState } from "../../../../components/useHooks"
import { AuditorResult } from "./LargeAuditorMenu"
import { RoleList, translateRoleOutline } from "../../../../game/roleListState.d"


export default function OjoMenu(
    props: {
        roleState: RoleState & {type: "ojo"}
    }
): ReactElement | null {

    const sendRoleChosen = (roleChosen: Role | null) => {
        GAME_MANAGER.sendSetRoleChosen(roleChosen);
    }

    const dayNumber = useGameState(
        state=>state.dayNumber,
        ["phase"]
    )!;

    const roleList = useGameState(
        state=>state.roleList,
        ["roleList"]
    )!;

    const [buttons, setButtons] = useState<
        ({type:"notUsed", chosen: boolean} | {type:"used", result: AuditorResult})[]
    >([]);

    useEffect(()=>{
        setButtons(roleList.map((entry, index)=>{
            let found = props.roleState.previouslyGivenResults.find(result=>result[0] === index);
            if(found){
                return {type:"used" as "used", result: found[1]};
            }else{
                return {type:"notUsed"  as "notUsed", chosen: props.roleState.chosenOutline === index};
            }
        }));
    }, [props.roleState.previouslyGivenResults, props.roleState.chosenOutline, roleList]);


    return <>
        <div className="large-auditor-menu">
            {translate("role.ojo.audit")}
            <div className="grid">
                <RoleListDisplay
                    roleList={roleList}
                    strikenOutlineIndexs={
                        props.roleState.previouslyGivenResults.map(result=>{return result[0] as number})
                    }
                />
                <ChooseButtons
                    buttons={buttons}
                />
            </div>
        </div>
        {(dayNumber > 1) && <div>
            {translate("role.ojo.attack")}
            <RoleDropdown
                value={props.roleState.roleChosen}
                onChange={(roleOption)=>{
                    sendRoleChosen(roleOption)
                }}
                canChooseNone={true}
            />
        </div>}
    </>
}
function RoleListDisplay(props: {
    roleList: RoleList,
    strikenOutlineIndexs: number[],
}): ReactElement {
    return <>
        {props.roleList.map((entry, index)=>{
            return <button 
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
            </button>
        })}
    </>
}
function ChooseButtons(props: {
    //true means its selected, false means its not selected
    buttons: ({type:"notUsed", chosen:boolean} | {type:"used", result:AuditorResult})[]
}): ReactElement {
    return<> {
        props.buttons.map((button, index)=>{
            if(button.type === "notUsed"){
                return <button 
                    className={"choose-button" + (button.chosen ? " highlighted" : "")}
                    key={index}
                    onClick={()=>{
                        GAME_MANAGER.sendSetAuditorChosenOutline(index)
                    }}
                >
                    <StyledText>
                        {translate("choose")}
                    </StyledText>
                </button>
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