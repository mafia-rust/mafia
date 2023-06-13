import React from "react";
import GAME_MANAGER from "../../../index";
import GameScreen, { ContentMenus } from "../GameScreen";
import WikiSearch from "../../WikiSearch";
import GameState from "../../../game/gameState.d";
import translate from "../../../game/lang";

interface WikiMenuProps {
}
interface WikiMenuState {
    gameState: GameState,
}


export default class WikiMenu extends React.Component<WikiMenuProps, WikiMenuState> {
    listener: () => void;
    
    constructor(props : WikiMenuProps) {
        super(props);

        this.state = {
            gameState : GAME_MANAGER.gameState,
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


    render(){return(<div>
        <button onClick={()=>{GameScreen.instance.closeMenu(ContentMenus.WikiMenu)}}>{translate("menu.wiki.title")}</button>
        <WikiSearch/>
        {/* <br/>
        <RolePicker roleListEntry={this.state.roleListEntry} onChange={(value)=>{this.onChangeRolePicker(value);}}/>
        <br/>
        {this.renderRole(this.state.roleListEntry)}
        <br/>
        TODO priorties list of ALL ROLES with collapsable sections
        TODO list of all night message strings
        <br/> */}
    </div>)}
}