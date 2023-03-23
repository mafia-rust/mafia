import React from "react";
import { translate } from "../game/lang";
import gameManager from "../index";

export class WikiMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            gameState : gameManager.gameState,
            roles: [], //List of roles to display
        };
        this.listener = ()=>{
            this.setState({
                gameState: gameManager.gameState,
            })
        };  
    }
    componentDidMount() {
        gameManager.addStateListener(this.listener);
    }
    componentWillUnmount() {
        gameManager.removeStateListener(this.listener);
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