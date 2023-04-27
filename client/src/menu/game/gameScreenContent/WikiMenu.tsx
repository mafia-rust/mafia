import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import GameState, { RoleListEntry } from "../../../game/gameState.d";
import GameScreen, { ContentMenus } from "../GameScreen";
import RolePicker from "../../RolePicker";


interface WikiMenuProps {
    role: RoleListEntry,
}
interface WikiMenuState {
    gameState: GameState,
    roleListEntry: RoleListEntry,
}


export default class WikiMenu extends React.Component<WikiMenuProps, WikiMenuState> {
    listener: () => void;
    
    constructor(props : WikiMenuProps) {
        super(props);

        this.state = {
            gameState : GAME_MANAGER.gameState,
            roleListEntry: {type: "any"},
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

    onChangeRolePicker(value: RoleListEntry){
        this.setState({
            roleListEntry: value,
        });        
    }


    renderRole(role: RoleListEntry){
        if (role.type === "exact"){
            return <div>
                <div>
                    {translate("role."+role.role+".name")}<br/><br/>
                    {translate("role."+role.role+".description")}<br/><br/>
                    {translate("role."+role.role+".advancedDescription")}<br/><br/>
                </div>
            </div>
        } else {
            return <div>
                {translate("menu.wiki.noRole")}
            </div>
        }
    }
    renderInvestigativeResults(){
        return <div>
            {this.state.gameState.investigatorResults.map((result, index)=>{
                //for every investigative result
                //TODO this flex box isnt working
                return <div key={index} style={{display:"flex"}}>
                    {result.map((role: string, index2: React.Key | null | undefined)=>{
                        //for every role in invest result
                        return <div key={index2}>
                            <button>{translate("role."+role+".name")}</button>
                        </div>
                    }, this)}
                </div>

            }, this)}
        </div>
    }
    render(){return(<div style={{height: "100%", overflowX:"hidden"}}>
        <button onClick={()=>{GameScreen.instance.closeMenu(ContentMenus.WikiMenu)}}>{translate("menu.wiki.title")}</button>
        <br/>
        <RolePicker roleListEntry={this.state.roleListEntry} onChange={(value)=>{this.onChangeRolePicker(value);}}/>
        <br/>
        {this.renderRole(this.state.roleListEntry)}
        <br/>
        {translate("menu.wiki.investigatorResults")}
        {this.renderInvestigativeResults()}
    </div>)}
}