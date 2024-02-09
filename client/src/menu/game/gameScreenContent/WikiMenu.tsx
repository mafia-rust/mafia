import React from "react";
import GAME_MANAGER from "../../../index";
import { ContentMenu, ContentTab } from "../GameScreen";
import "./wikiMenu.css"
import GameState from "../../../game/gameState.d";
import translate from "../../../game/lang";
import WikiSearch from "../../../components/WikiSearch";
import { getRolesFromRoleListRemoveExclusionsAddConversions, getRolesComplement } from "../../../game/roleListState.d";

type WikiMenuProps = {
}
type WikiMenuState = {
    gameState: GameState,
}

export default class WikiMenu extends React.Component<WikiMenuProps, WikiMenuState> {
    listener: () => void;
    
    constructor(props : WikiMenuProps) {
        super(props);

        if(GAME_MANAGER.state.stateType === "game")
            this.state = {
                gameState : GAME_MANAGER.state,
            };
        this.listener = ()=>{
            if(GAME_MANAGER.state.stateType === "game")
                this.setState({
                    gameState: GAME_MANAGER.state,
                });
        };  
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    render(){return(<div className="wiki-menu wiki-menu-colors">
        <ContentTab close={ContentMenu.WikiMenu} helpMenu={null}>{translate("menu.wiki.title")}</ContentTab>
        
        <div className="wiki-menu-search">
            <WikiSearch excludedRoles={
                GAME_MANAGER.state.stateType === "lobby" || GAME_MANAGER.state.stateType === "game" ?
                getRolesComplement(getRolesFromRoleListRemoveExclusionsAddConversions(GAME_MANAGER.state.roleList, GAME_MANAGER.state.excludedRoles)) : []
            }/>
        </div>
    </div>)}
}