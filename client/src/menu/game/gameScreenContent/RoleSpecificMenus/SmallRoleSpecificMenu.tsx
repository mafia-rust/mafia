import React from "react"
import GameState from "../../../../game/gameState.d"
import GAME_MANAGER from "../../../.."
import translate from "../../../../game/lang"
import StyledText from "../../../../components/StyledText"
import RoleOutlineDropdown from "../../../../components/OutlineSelector"
import "./smallRoleSpecificMenu.css"

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


        switch(this.state.gameState.clientState.roleState?.role){
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
            case "janitor":
                return <StyledText>{translate("role.janitor.roleDataText", this.state.gameState.clientState.roleState.cleansRemaining)}</StyledText>;
            case "death":
                return <StyledText>{translate("role.death.roleDataText", this.state.gameState.clientState.roleState.souls)}</StyledText>;
            case "amnesiac":
                return <><StyledText>{translate("role.amnesiac.smallRoleMenu")}</StyledText><RoleOutlineDropdown 
                    roleOutline={this.state.gameState.clientState.roleState.roleOutline ?? {type: "any"}} 
                    onChange={(rle)=>{
                        GAME_MANAGER.sendSetAmnesiacRoleOutline(rle);
                    }}
                /></>;
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