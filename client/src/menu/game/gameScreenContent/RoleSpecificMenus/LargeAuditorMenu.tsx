import { ReactElement, useEffect, useState } from "react"
import { Role } from "../../../../game/roleState.d"
import React from "react"
import { RoleList, translateRoleOutline } from "../../../../game/roleListState.d"
import StyledText from "../../../../components/StyledText"
import GAME_MANAGER from "../../../.."
import "./largeAuditorMenu.css"
import translate from "../../../../game/lang"

export type AuditorResult = {
    type: "two",
    roles: [Role, Role]
} | {
    type: "one",
    role: Role
}

export default function LargeAuditorMenu(props: {}): ReactElement {

    const [buttons, setButtons] = useState<
        ({type:"notUsed", chosen: boolean} | {type:"used", result:AuditorResult})[]
    >([])

    useEffect(()=>{
        const listener = ()=>{
            if(
                GAME_MANAGER.state.stateType !== "game" ||
                GAME_MANAGER.state.clientState.type !== "player" ||
                GAME_MANAGER.state.clientState.roleState?.type !== "auditor"
            ){
                return;
            }
    
            let new_buttons = [];
            for(let i = 0; i < GAME_MANAGER.state.roleList.length; i++){
                let found = GAME_MANAGER.state.clientState.roleState.previouslyGivenResults.find(result=>result[0] === i);
                if(found){
                    new_buttons.push({type:"used" as "used", result: found[1]});
                }else{
                    new_buttons.push({type:"notUsed"  as "notUsed", chosen: GAME_MANAGER.state.clientState.roleState.nightSelection.chosenOutline === i});
                }
            }
            setButtons(new_buttons);
        };

        listener();
        GAME_MANAGER.addStateListener(listener);
        return ()=>GAME_MANAGER.removeStateListener(listener);
    }, [setButtons])

    if(
        GAME_MANAGER.state.stateType !== "game" ||
        GAME_MANAGER.state.clientState.type !== "player" ||
        GAME_MANAGER.state.clientState.roleState?.type !== "auditor"
    )
        return <></>
    
    
    return <div className="large-auditor-menu">
        <div className="grid">
            <RoleListDisplay
                roleList={GAME_MANAGER.state.roleList}
                strikenOutlineIndexs={
                    GAME_MANAGER.state.clientState.roleState.previouslyGivenResults.map(result=>{return result[0] as number})
                }
            />
            <ChooseButtons
                buttons={buttons}
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
                        GAME_MANAGER.sendRoleActionChoice({
                            type: "auditor",
                            chosenOutline: index
                        })
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