import React from "react"
import GameState from "../../../../game/gameState.d"
import GAME_MANAGER from "../../../.."
import translate from "../../../../game/lang"
import StyledText from "../../../../components/StyledText"
import "./smallRoleSpecificMenu.css"
import SmallOjoMenu from "./SmallOjoMenu"
import RoleDropdown from "../../../../components/RoleDropdown"
import ROLES from "../../../../resources/roles.json"
import { getRolesComplement } from "../../../../game/roleListState.d"
import { Role } from "../../../../game/roleState.d"



type SmallRoleSpecificMenuProps = {
}
type SmallRoleSpecificMenuState = {
    gameState: GameState
}
export default class SmallRoleSpecificMenu extends React.Component<SmallRoleSpecificMenuProps, SmallRoleSpecificMenuState> {
    listener: () => void;
    constructor(props: SmallRoleSpecificMenuProps) {
        super(props);

        if(GAME_MANAGER.state.stateType === "game")
            this.state = {
                gameState : GAME_MANAGER.state,
            };
        this.listener = ()=>{
            if(GAME_MANAGER.state.stateType === "game")
                this.setState({
                    gameState: GAME_MANAGER.state
                });
        };  
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    render(): JSX.Element | null {
        const specific = this.renderSpecific();
        if(specific !== null)
            return <div className="small-role-specific-menu">
                {specific}
            </div>
        return null;
    }

    renderSpecific(): JSX.Element | null {

        if(this.state.gameState.clientState.type === "spectator")
            return null;


        switch(this.state.gameState.clientState.roleState?.type){
            case "jailor":
                if(this.state.gameState.phaseState.type==="night") {
                    return <StyledText>{translate("role.jailor.roleDataText.night", this.state.gameState.clientState.roleState.executionsRemaining)}</StyledText>;
                } else if (this.state.gameState.clientState.roleState.jailedTargetRef === null) {
                    return <StyledText>{translate("role.jailor.roleDataText.nobody", this.state.gameState.clientState.roleState.executionsRemaining)}</StyledText>;
                } else {
                    return <StyledText>{translate("role.jailor.roleDataText", 
                        this.state.gameState.players[this.state.gameState.clientState.roleState.jailedTargetRef].toString(), 
                        this.state.gameState.clientState.roleState.executionsRemaining
                    )}</StyledText>;;
                }
            case "medium":
                if (this.state.gameState.clientState.roleState.seancedTarget === null) {
                    return <StyledText>{translate("role.medium.roleDataText.nobody", this.state.gameState.clientState.roleState.seancesRemaining)}</StyledText>;
                } else {
                    return <StyledText>{translate("role.medium.roleDataText", 
                        this.state.gameState.players[this.state.gameState.clientState.roleState.seancedTarget].toString(),
                        this.state.gameState.clientState.roleState.seancesRemaining
                    )}</StyledText>;
                }
            case "doctor":
                return <StyledText>{translate("role.doctor.roleDataText", this.state.gameState.clientState.roleState.selfHealsRemaining)}</StyledText>;
            case "bodyguard":
                return <StyledText>{translate("role.bodyguard.roleDataText", this.state.gameState.clientState.roleState.selfShieldsRemaining)}</StyledText>;
            case "engineer":
                return <>
                    <div>
                        <StyledText>{translate("role.engineer.roleDataText." + this.state.gameState.clientState.roleState.trap.type)}</StyledText>
                    </div>
                    {
                        this.state.gameState.clientState.roleState.trap.type === "set" &&
                        this.state.gameState.phaseState.type === "night" &&
                        <button className={this.state.gameState.clientState.roleState.trap.shouldUnset?"highlighted":""} onClick={()=>{
                            if(
                                this.state.gameState.clientState.type === "player" && 
                                this.state.gameState.clientState.roleState?.type === "engineer" && 
                                this.state.gameState.clientState.roleState.trap.type === "set"
                            )
                                GAME_MANAGER.sendSetEngineerShouldUnset(!this.state.gameState.clientState.roleState.trap.shouldUnset);
                        }}>{translate("role.engineer.roleDataText.unset")}</button>
                    }
                </>;
            case "vigilante":
                switch(this.state.gameState.clientState.roleState.state.type){
                    case "willSuicide":
                        return <StyledText>{translate("role.vigilante.roleDataText.suicide")}</StyledText>;
                    case "notLoaded":
                        return <StyledText>{translate("role.vigilante.roleDataText.notLoaded")}</StyledText>;
                    case "loaded":
                        return <StyledText>{translate("role.vigilante.roleDataText", this.state.gameState.clientState.roleState.state.bullets)}</StyledText>;
                    default:
                        return null
                }
            case "veteran":
                return <StyledText>{translate("role.veteran.roleDataText", this.state.gameState.clientState.roleState.alertsRemaining)}</StyledText>;
            case "mortician":
                return <StyledText>{translate("role.mortician.roleDataText", (3-this.state.gameState.clientState.roleState.crematedPlayers.length))}</StyledText>;
            case "death":
                return <StyledText>{translate("role.death.roleDataText", this.state.gameState.clientState.roleState.souls)}</StyledText>;
            case "ojo":
                if(this.state.gameState.phaseState.type === "night" && this.state.gameState.clientState.myIndex!==null && this.state.gameState.players[this.state.gameState.clientState.myIndex].alive)
                    return <SmallOjoMenu action={this.state.gameState.clientState.roleState.chosenAction}/>;
                return null;
            case "wildcard":
            case "trueWildcard":
                return <><StyledText>{translate("role.wildcard.smallRoleMenu")}</StyledText><div><RoleDropdown 
                    value={this.state.gameState.clientState.roleState.role ?? "wildcard"}
                    disabledRoles={this.state.gameState.excludedRoles} 
                    onChange={(rle)=>{
                        GAME_MANAGER.sendSetWildcardRoleOutline(rle);
                    }}
                /></div></>;
            case "mafiaWildcard":
                const all_choosable_mafia: Role[] = Object.keys(ROLES).filter((rle)=>
                    ROLES[rle as keyof typeof ROLES].faction === "mafia" &&
                    rle !== "godfather" &&
                    rle !== "mafioso" &&
                    !this.state.gameState.excludedRoles.includes(rle as Role)
                ).map((r)=>r as Role);

                return <><StyledText>{translate("role.mafiaWildcard.smallRoleMenu")}</StyledText><div><RoleDropdown 
                    value={this.state.gameState.clientState.roleState.role ?? "mafiaWildcard"} 
                    disabledRoles={getRolesComplement(all_choosable_mafia)}
                    onChange={(rle)=>{
                        GAME_MANAGER.sendSetWildcardRoleOutline(rle);
                    }}
                /></div></>;
            case "martyr":
                if (this.state.gameState.clientState.roleState.state.type === "stillPlaying") {
                    return <StyledText>{translate("role.martyr.roleDataText", this.state.gameState.clientState.roleState.state.bullets)}</StyledText>;
                } else {
                    return null;
                }
            default:
                return null;
        }
    }
}