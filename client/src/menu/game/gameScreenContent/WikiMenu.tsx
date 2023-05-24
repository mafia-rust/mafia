import React from "react";
import translate, { styleText } from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import GameState, { RoleListEntry } from "../../../game/gameState.d";
import GameScreen, { ContentMenus } from "../GameScreen";
import RolePicker from "../../RolePicker";
import ROLES from "../../../resources/roles.json";

interface WikiMenuProps {
    roleListEntry: RoleListEntry | null,
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
            roleListEntry: (this.props.roleListEntry!==null ? this.props.roleListEntry : {type: "any"}),
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

            let defenseString = "";
            switch(ROLES[role.role as keyof typeof ROLES].defense){
                case 0:
                    defenseString = translate("none");
                    break;
                case 1:
                    defenseString = translate("basic");
                    break;
                case 2:
                    defenseString = translate("powerful");
                    break;
                case 3:
                    defenseString = translate("invincible");
                    break;
            }

            return <div>
                <div>
                    <div>{styleText(translate("role."+role.role+".name"))}</div>
                    <br/>
                    {styleText(translate("menu.wiki.abilities"))}
                    {this.renderRoleText(translate("role."+role.role+".abilities"))}
                    <br/>
                    {styleText(translate("menu.wiki.attributes"))}
                    {this.renderRoleText(translate("role."+role.role+".attributes"))}
                    <br/>
                    {styleText(translate("menu.wiki.maxCount", ROLES[role.role as keyof typeof ROLES].maxCount))}<br/>
                    {styleText(translate("menu.wiki.suspicious", ROLES[role.role as keyof typeof ROLES].suspicious?"suspicious":"innocent"))}<br/>
                    {styleText(translate("menu.wiki.defense", defenseString))}<br/>
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
            out.push(<li key={i}>{split[i]}</li>);
        }
        return <div>{out}</div>
    }
    render(){return(<div style={{height: "100%", overflowX:"hidden"}}>
        <button onClick={()=>{GameScreen.instance.closeMenu(ContentMenus.WikiMenu)}}>{translate("menu.wiki.title")}</button>
        <br/>
        <RolePicker roleListEntry={this.state.roleListEntry} onChange={(value)=>{this.onChangeRolePicker(value);}}/>
        <br/>
        {this.renderRole(this.state.roleListEntry)}
        <br/>
        TODO priorties list of ALL ROLES with collapsable sections
        TODO list of all night message strings
        <br/>
    </div>)}
}