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
export default class SmallRoleSpecifcMenu extends React.Component<SmallRoleSpecificMenuProps, SmallRoleSpecificMenuState> {
    listener: () => void;
    constructor(props: SmallRoleSpecificMenuProps) {
        super(props);

        this.state = {
            gameState : GAME_MANAGER.gameState,
        };
        this.listener = ()=>{
            this.setState({
                gameState: GAME_MANAGER.gameState
            })
        };  
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    render(){
        let roleSpecificText = null;
        switch(this.state.gameState.roleState?.role){
            case "jailor":
                if(this.state.gameState.phase==="night")
                    roleSpecificText = ""+this.state.gameState.roleState.executionsRemaining;
                else
                {
                    let jailedString = this.state.gameState.roleState.jailedTargetRef!=null?
                        this.state.gameState.players[this.state.gameState.roleState.jailedTargetRef].toString():
                        translate("none");
                    roleSpecificText = jailedString+" "+this.state.gameState.roleState.executionsRemaining;
                }
                break;
            case "medium":
                let seancedString = this.state.gameState.roleState.seancedTarget!=null?
                    this.state.gameState.players[this.state.gameState.roleState.seancedTarget].toString():
                    translate("none");
                roleSpecificText = seancedString+" "+this.state.gameState.roleState.seancnesRemaining;
                break;
            case "doctor":
                roleSpecificText = ""+this.state.gameState.roleState.selfHealsRemaining;
                break;
            case "bodyguard":
                roleSpecificText = ""+this.state.gameState.roleState.selfShieldsRemaining;
                break;
            case "vigilante":
                if(this.state.gameState.roleState.willSuicide)
                    roleSpecificText = translate("grave.killer.suicide");
                else
                    roleSpecificText = ""+this.state.gameState.roleState.bulletsRemaining;
                    break;
            case "veteran":
                roleSpecificText = ""+this.state.gameState.roleState.alertsRemaining;
                break;
            case "janitor":
                roleSpecificText = ""+this.state.gameState.roleState.cleansRemaining;
                break;
        }
        if(roleSpecificText!==null){
            return <div className="role-specific">
                <StyledText>
                    {roleSpecificText}
                </StyledText>
            </div>
        }
        return null
    }
}