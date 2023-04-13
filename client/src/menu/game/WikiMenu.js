import React from "react";
import translate from "../../game/lang";
import GAME_MANAGER from "../../index";

export default class WikiMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            gameState : GAME_MANAGER.gameState,
            roles: [], //List of roles to display
        };
        this.listener = ()=>{
            this.setState({
                gameState: GAME_MANAGER.gameState,
            })
        };  
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }
    renderRole(role, index){
        return <div key={index}>
            <button>{translate("role."+role+".name")}</button>
        </div>
    }
    renderRoleExtra(){

    }
    renderInvestigativeResults(){
        return <div>
            {this.state.gameState.investigatorResults.map((result, index)=>{
                return <div key={index}>
                    {result.map((role, index2)=>{
                        return <div key={index2} style={{display:"flex"}}>
                            <button>{translate("role."+role+".name")}</button>
                        </div>
                    }, this)}
                </div>
            }, this)}
        </div>
    }
    render(){return(<div style={{height: "100%", overflowX:"hidden"}}>
        
    </div>)}
}