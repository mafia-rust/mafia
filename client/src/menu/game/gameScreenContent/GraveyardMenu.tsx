import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import { ContentMenu, ContentTab } from "../GameScreen";
import "./graveyardMenu.css";
import { Player } from "../../../game/gameState.d";
import { RoleList, translateRoleOutline } from "../../../game/roleListState.d";
import StyledText from "../../../components/StyledText";
import GraveComponent from "../../../components/grave";
import { Grave } from "../../../game/graveState";
import { StateListener } from "../../../game/gameManager.d";
import { Role } from "../../../game/roleState.d";
import Icon from "../../../components/Icon";
import { EnabledRolesDisplay } from "../../../components/gameModeSettings/EnabledRoleSelector";

type GraveyardMenuProps = {
}
type GraveyardMenuState = {
    graves: Grave[],
    players: Player[],
    roleList: RoleList,
    enabledRoles: Role[],
    extendedGraveIndex: number | null,
    strickenRoleListIndex: number[]
}

export default class GraveyardMenu extends React.Component<GraveyardMenuProps, GraveyardMenuState> {
    listener: StateListener;
    constructor(props: GraveyardMenuProps) {
        super(props);

        if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player")
            this.state = {
                graves: GAME_MANAGER.state.graves,
                players: GAME_MANAGER.state.players,
                roleList: GAME_MANAGER.state.roleList,
                enabledRoles: GAME_MANAGER.state.enabledRoles,
                extendedGraveIndex: null,
                strickenRoleListIndex: GAME_MANAGER.state.clientState.crossedOutOutlines
            };
        this.listener = (type)=>{
            if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player"){
                switch (type) {
                    case "addGrave":
                        this.setState({graves: GAME_MANAGER.state.graves})
                    break;
                    case "gamePlayers":
                        this.setState({players: GAME_MANAGER.state.players})
                    break;
                    case "roleList":
                        this.setState({roleList: GAME_MANAGER.state.roleList})
                    break;
                    case "enabledRoles":
                        this.setState({enabledRoles: GAME_MANAGER.state.enabledRoles})
                    break;
                    case "yourCrossedOutOutlines":
                        this.setState({strickenRoleListIndex: GAME_MANAGER.state.clientState.crossedOutOutlines})
                    break;
                }
            }
        };  
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    renderGraves(){
        return <>
            {this.state.graves.map((grave, graveIndex)=>{
                return this.renderGrave(grave, graveIndex);
            }, this)}
        </>
    }
    renderGrave(grave: Grave, graveIndex: number){
        let graveRoleString: string;
        if (grave.information.type === "normal") {
            graveRoleString = translate(`role.${grave.information.role}.name`);
        } else {
            graveRoleString = translate("obscured");
        }

        return(<button
            className="grave-list-button"
            style={{ gridRow: graveIndex + 1 }} 
            key={graveIndex}
            onClick={()=>{
                let extendedGraveIndex = this.state.extendedGraveIndex;
                if(extendedGraveIndex === graveIndex)
                    extendedGraveIndex = null;
                else
                    extendedGraveIndex = graveIndex;
                this.setState({extendedGraveIndex:extendedGraveIndex})
            }}
        >
            <span>
                {
                    this.state.extendedGraveIndex === graveIndex ? 
                        <Icon>menu</Icon> :
                        <Icon>menu_open</Icon>
                }
                <StyledText noLinks={true}>{this.state.players[grave.player]?.toString()}</StyledText>
                <StyledText noLinks={true}>
                    {` (${graveRoleString})`}
                </StyledText>
            </span>
        </button>);
    }
    renderGraveExtended(grave: Grave){
        return(<div className="grave-label">
            <button onClick={()=>this.setState({extendedGraveIndex:null})}>
                <Icon>close</Icon>
            </button>
            <GraveComponent grave={grave} playerNames={this.state.players.map(p => p.toString())}/>
        </div>);
    }

    renderRoleList(){return<>
        {this.state.roleList.map((entry, index)=>{
            return <button 
                className="role-list-button"
                style={{ gridRow: index + 1 }} 
                key={index}
                onClick={()=>{
                    let strickenRoleListIndex = this.state.strickenRoleListIndex;
                    if(strickenRoleListIndex.includes(index))
                        strickenRoleListIndex = strickenRoleListIndex.filter(x=>x!==index);
                    else
                        strickenRoleListIndex.push(index);

                    this.setState({strickenRoleListIndex:strickenRoleListIndex})
                    GAME_MANAGER.sendSaveCrossedOutOutlinesPacket(strickenRoleListIndex);
                }}
                onMouseDown={(e)=>{
                    // on right click, show a list of all roles that can be in this bucket
                    // if(e.button === 2){
                    //     e.preventDefault();
                    // }
                }}
            >
                {
                    this.state.strickenRoleListIndex.includes(index) ? 
                    <s><StyledText>
                        {translateRoleOutline(entry) ?? ""}
                    </StyledText></s> : 
                    <StyledText>
                        {translateRoleOutline(entry) ?? ""}
                    </StyledText>
                }
            </button>
        }, this)}
    </>}

    renderEnabledRoles(){
        return<details className="graveyard-menu-enabledRoles">
            <summary>
                {translate("menu.enabledRoles.enabledRoles")}
            </summary>
            <EnabledRolesDisplay enabledRoles={this.state.enabledRoles}/>
        </details>
    }


    render(){return(<div className="graveyard-menu graveyard-menu-colors">
        <ContentTab close={ContentMenu.GraveyardMenu} helpMenu={"standard/graveyard"}>{translate("menu.graveyard.title")}</ContentTab>
            
        <div className="grid">
            {this.renderRoleList()}
            {this.renderGraves()}
        </div>
        {this.state.extendedGraveIndex!==null?this.renderGraveExtended(this.state.graves[this.state.extendedGraveIndex]):null}
        {this.renderEnabledRoles()}
    </div>)}
}