import React, { Key } from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import GameState from "../../../game/gameState.d";


interface WikiMenuProps {
    role: string | null,
}
interface WikiMenuState {
    gameState: GameState,
    role: string | null,
}


export default class WikiMenu extends React.Component<WikiMenuProps, WikiMenuState> {
    listener: () => void;
    
    constructor(props : WikiMenuProps) {
        super(props);

        this.state = {
            gameState : GAME_MANAGER.gameState,
            role: props.role, //List of roles to display
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
    renderRole(role: string){
        return <div>
            <button>{translate("role."+role+".name")}</button>
        </div>
    }
    renderInvestigativeResults(){
        return <div>
            {this.state.gameState.investigatorResults.map((result, index)=>{
                //for every investigative result
                return <div key={index}>
                    {result.map((role: string, index2: React.Key | null | undefined)=>{
                        //for every role in invest result
                        return <div key={index2} style={{display:"flex"}}>
                            <button>{translate("role."+role+".name")}</button>
                        </div>
                    }, this)}
                </div>

            }, this)}
        </div>
    }
    render(){return(<div style={{height: "100%", overflowX:"hidden"}}>
        {translate("menu.wiki.title")}
        {/* TODO, rolepicker code here*/}
        {this.state.role?this.renderRole(this.state.role):null}
        {this.renderInvestigativeResults()}
        
    </div>)}
}