import React from "react"
import GameState from "../../../../game/gameState.d"
import GAME_MANAGER from "../../../.."
import translate from "../../../../game/lang"
import StyledText from "../../../../components/StyledText"

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

    render(){
        const roleSpecificText = this.getRoleSpecificText();

        return roleSpecificText && <div className="role-specific">
            <StyledText>
                {roleSpecificText}
            </StyledText>
        </div>
    }

    getRoleSpecificText(): string | undefined {
        switch(this.state.gameState.roleState?.role){
            case "jailor":
                if(this.state.gameState.phase==="night") {
                    return translate("role.jailor.roleDataText.night", this.state.gameState.roleState.executionsRemaining);
                } else if (this.state.gameState.roleState.jailedTargetRef === null) {
                    return translate("role.jailor.roleDataText.nobody", this.state.gameState.roleState.executionsRemaining)
                } else {
                    return translate("role.jailor.roleDataText", 
                        this.state.gameState.players[this.state.gameState.roleState.jailedTargetRef].toString(), 
                        this.state.gameState.roleState.executionsRemaining
                    );
                }
            case "medium":
                if (this.state.gameState.roleState.seancedTarget === null) {
                    return translate("role.medium.roleDataText.nobody", this.state.gameState.roleState.seancesRemaining);
                } else {
                    return translate("role.medium.roleDataText", 
                        this.state.gameState.players[this.state.gameState.roleState.seancedTarget].toString(),
                        this.state.gameState.roleState.seancesRemaining
                    )
                }
            case "doctor":
                return translate("role.doctor.roleDataText", this.state.gameState.roleState.selfHealsRemaining)
            case "bodyguard":
                return translate("role.bodyguard.roleDataText", this.state.gameState.roleState.selfShieldsRemaining)
            case "vigilante":
                if(this.state.gameState.roleState.willSuicide)
                    return translate("role.vigilante.roleDataText.suicide");
                else
                    return translate("role.vigilante.roleDataText", this.state.gameState.roleState.bulletsRemaining);
            case "veteran":
                return translate("role.veteran.roleDataText", this.state.gameState.roleState.alertsRemaining);
            case "janitor":
                return translate("role.janitor.roleDataText", this.state.gameState.roleState.cleansRemaining);
            case "death":
                return translate("role.death.roleDataText", this.state.gameState.roleState.souls);
        }
    }
}