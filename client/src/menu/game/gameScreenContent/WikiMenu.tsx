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
                    <div>{translate("role."+role.role+".name")}</div>
                    <br/>
                    {translate("menu.wiki.abilities")}
                    {this.renderRoleText(translate("role."+role.role+".abilities"))}
                    <br/>
                    {translate("menu.wiki.attributes")}
                    {this.renderRoleText(translate("role."+role.role+".attributes"))}
                </div>
            </div>
        } else {
            return <div>
                {translate("menu.wiki.noRole")}
            </div>
        }
    }
    renderRoleText(string: string): JSX.Element{
        let split = string.split("*");
        let out = [];
        for(let i = 1; i < split.length; i++){
            out.push(<li>{split[i]}</li>);
            // out.push(<br/>);
        }
        return <div>{out}</div>
    }
    render(){return(<div style={{height: "100%", overflowX:"hidden"}}>
        <button onClick={()=>{GameScreen.instance.closeMenu(ContentMenus.WikiMenu)}}>{translate("menu.wiki.title")}</button>
        <br/>
        <RolePicker roleListEntry={this.state.roleListEntry} onChange={(value)=>{this.onChangeRolePicker(value);}}/>
        <br/>
        {this.renderRole(this.state.roleListEntry)}
        TODO priorties list of ALL ROLES with collapsable sections
        TODO list of all night message strings
        <br/>
    </div>)}
}