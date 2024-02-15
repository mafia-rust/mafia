import React from "react"
import GameState from "../../../../game/gameState.d"
import GAME_MANAGER from "../../../.."
import translate from "../../../../game/lang"
import StyledText from "../../../../components/StyledText"
import RoleOutlineDropdown from "../../../../components/RolePicker"

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
        switch(this.state.gameState.roleState?.role){
            case "jailor":
                if(this.state.gameState.phase==="night") {
                    return <StyledText>{translate("role.jailor.roleDataText.night", this.state.gameState.roleState.executionsRemaining)}</StyledText>;
                } else if (this.state.gameState.roleState.jailedTargetRef === null) {
                    return <StyledText>{translate("role.jailor.roleDataText.nobody", this.state.gameState.roleState.executionsRemaining)}</StyledText>;
                } else {
                    return <StyledText>{translate("role.jailor.roleDataText", 
                        this.state.gameState.players[this.state.gameState.roleState.jailedTargetRef].toString(), 
                        this.state.gameState.roleState.executionsRemaining
                    )}</StyledText>;;
                }
            case "medium":
                if (this.state.gameState.roleState.seancedTarget === null) {
                    return <StyledText>{translate("role.medium.roleDataText.nobody", this.state.gameState.roleState.seancesRemaining)}</StyledText>;
                } else {
                    return <StyledText>{translate("role.medium.roleDataText", 
                        this.state.gameState.players[this.state.gameState.roleState.seancedTarget].toString(),
                        this.state.gameState.roleState.seancesRemaining
                    )}</StyledText>;
                }
            case "doctor":
                return <StyledText>{translate("role.doctor.roleDataText", this.state.gameState.roleState.selfHealsRemaining)}</StyledText>;
            case "bodyguard":
                return <StyledText>{translate("role.bodyguard.roleDataText", this.state.gameState.roleState.selfShieldsRemaining)}</StyledText>;
            case "vigilante":
                if(this.state.gameState.roleState.willSuicide)
                    return <StyledText>{translate("role.vigilante.roleDataText.suicide")}</StyledText>;
                else
                    return <StyledText>{translate("role.vigilante.roleDataText", this.state.gameState.roleState.bulletsRemaining)}</StyledText>;
            case "veteran":
                return <StyledText>{translate("role.veteran.roleDataText", this.state.gameState.roleState.alertsRemaining)}</StyledText>;
            case "janitor":
                return <StyledText>{translate("role.janitor.roleDataText", this.state.gameState.roleState.cleansRemaining)}</StyledText>;
            case "death":
                return <StyledText>{translate("role.death.roleDataText", this.state.gameState.roleState.souls)}</StyledText>;
            case "amnesiac":
                return <RoleOutlineDropdown 
                    roleOutline={this.state.gameState.roleState.roleOutline ?? {type: "any"}} 
                    onChange={(rle)=>{
                        GAME_MANAGER.sendSetAmnesiacRoleOutline(rle);
                    }}
                />;
            case "martyr":
                if (this.state.gameState.roleState.state.type === "stillPlaying") {
                    return <StyledText>{translate("role.martyr.roleDataText", this.state.gameState.roleState.state.bullets)}</StyledText>;
                } else {
                    return null;
                }
            default:
                return null;
        }
    }
}