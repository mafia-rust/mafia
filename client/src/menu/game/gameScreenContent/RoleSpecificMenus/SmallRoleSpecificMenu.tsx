import React from "react"
import GameState from "../../../../game/gameState.d"
import GAME_MANAGER from "../../../.."
import translate, { styleText } from "../../../../game/lang"

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
        let roleSpecificJSX = null;
        switch(this.state.gameState.roleState?.role){
            case "jailor":
                if(this.state.gameState.phase==="night")
                    roleSpecificJSX = styleText(""+this.state.gameState.roleState.executionsRemaining);
                else
                {
                    let jailedString = this.state.gameState.roleState.jailedTargetRef!=null?
                        this.state.gameState.players[this.state.gameState.roleState.jailedTargetRef].toString():
                        translate("none");
                    roleSpecificJSX = styleText(jailedString+" "+this.state.gameState.roleState.executionsRemaining);
                }
                break;
            case "medium":
                let seancedString = this.state.gameState.roleState.seancedTarget!=null?
                    this.state.gameState.players[this.state.gameState.roleState.seancedTarget].toString():
                    translate("none");
                roleSpecificJSX = styleText(seancedString+" "+this.state.gameState.roleState.seancnesRemaining);
                break;
            case "doctor":
                roleSpecificJSX = styleText(""+this.state.gameState.roleState.selfHealsRemaining);
                break;
            case "bodyguard":
                roleSpecificJSX = styleText(""+this.state.gameState.roleState.selfShieldsRemaining);
                break;
            case "vigilante":
                if(this.state.gameState.roleState.willSuicide)
                    roleSpecificJSX = styleText(translate("grave.killer.suicide"));
                else
                    roleSpecificJSX = styleText(""+this.state.gameState.roleState.bulletsRemaining);
                    break;
            case "veteran":
                roleSpecificJSX = styleText(""+this.state.gameState.roleState.alertsRemaining);
                break;
            case "janitor":
                roleSpecificJSX = styleText(""+this.state.gameState.roleState.cleansRemaining);
                break;
        }
        if(roleSpecificJSX!==null){
            return <div className="role-specific">{roleSpecificJSX}</div>
        }
        return null
    }
}