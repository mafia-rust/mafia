import React from "react";
import GAME_MANAGER from "../../index";
import ROLES from "../../resources/roles.json"
import "../../index.css";


export default class LobbyRolePane extends React.Component {
    constructor(props){
        super(props);

        this.state = {
            gameState: GAME_MANAGER.gameState,
            roleList: [],
            rolePickers: []
        }

        this.listener = (type)=>{
            if(type==="RoleList"){
                
                //create rolePickers
                let newRolePickers = [];
                for(let i = 0; i < GAME_MANAGER.gameState.roleList.length; i++){

                    let roleListEntry = GAME_MANAGER.gameState.roleList[i];
                    let newAlignment = undefined;
                    let newFaction = undefined;
                    //if the faction and the role exist then use the role
                    if(roleListEntry.Exact){
                        newFaction = roleListEntry.Exact;
                        newAlignment = roleListEntry.Exact;
                    }
                    //if no role exists then use the faction
                    else if(roleListEntry.Faction){
                        newFaction = roleListEntry.Faction;
                        newAlignment = "Random";

                    } else if (roleListEntry === "Any") {
                        newFaction = "Any";
                        newAlignment = "Random";
                    } else if (roleListEntry.FactionAlignment) {
                        newFaction = roleListEntry.FactionAlignment;
                        newAlignment = roleListEntry.FactionAlignment;
                    } else if (roleListEntry.Faction) {
                        newFaction = roleListEntry.Faction;
                        newAlignment = "Random";
                    } else {
                        console.log("ERROR roleListEntry is not valid");
                    }

                    console.log(newFaction +" -- "+ newAlignment);

                    let newRolePicker = <RolePicker
                        key={i}
                        index={i}
                        faction={newFaction}
                        alignment={newAlignment}
                        onChange={
                            (index, value)=>{this.onChangeRolePicker(index, value)}
                        }
                    />;

                    newRolePickers.push(newRolePicker);
                }

                this.setState({
                    rolePickers: newRolePickers,
                    roleList: [...GAME_MANAGER.gameState.roleList],
                    gameState: GAME_MANAGER.gameState
                });

            }else{
                //create rolePickers
                let newRolePickers = this.state.rolePickers;
                if(this.state.gameState.players.length < this.state.rolePickers.length){
                    newRolePickers = []  
                }

                for(let i = 0; i < this.state.gameState.players.length; i++){
                    if(i >= this.state.rolePickers.length){
                        newRolePickers.push(<RolePicker 
                            key={i}
                            index={i}
                            onChange={
                                (index, value)=>{this.onChangeRolePicker(index, value)}
                            }
                        />);
                    }
                }
                //create roleList

                let newRoleList = this.state.roleList;
                if(this.state.gameState.players.length < this.state.roleList.length){
                    newRoleList = []  
                }

                for(let i = 0; i < this.state.gameState.players.length; i++){
                    if(i >= this.state.roleList.length)
                        newRoleList.push(createRoleListEntry_Any());
                }

                ////
                this.setState({
                    rolePickers: newRolePickers,
                    roleList: newRoleList,
                    gameState: GAME_MANAGER.gameState
                });
            }
        };  
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    render(){return(<div>
        Role List
        {this.renderList()}
    </div>);}

    renderList(){
        return<div>
            {this.state.rolePickers}
        </div>
    }

    onChangeRolePicker(index, value){
        let newList = this.state.roleList;
        newList[index] = value;
        this.setState({
            roleList: newList
        });
        GAME_MANAGER.roleList_button(this.state.roleList);
    }
}

///props.faction
///props.alignment
///props.index
///props.onChange
class RolePicker extends React.Component {
    constructor(props) {
        super(props);

        console.log("Role picker" + props.faction + " -- " + props.alignment);

        this.state = {
            faction: props.faction!==undefined?props.faction:"Any",
            alignment: props.alignment!==undefined?props.alignment:"Random",
            index: props.index,
            onChange: props.onChange,
        };
    }

    allFactions() {
        let factions = [];
        for (let role in ROLES) {
            if (!factions.includes(ROLES[role].faction)) {
                factions.push(ROLES[role].faction);
            }
        }
        return factions;
    }

    allAlignments(faction) {
        let alignments = [];
        let roles = [];

        for (let role in ROLES) {
            if (ROLES[role].faction !== faction) continue;

            if (!alignments.includes(ROLES[role].alignment)) {
                alignments.push(ROLES[role].alignment);
            }
            if (!roles.includes(role)) {
                roles.push(role);
            }
        }

        return alignments.concat(roles);
    }
    render(){return(<div className="role-picker-container">
        <select
            className="dropdown"
            defaultValue={this.state.faction}
            onChange={(e) => {
                if (e.target.value === "Any") {
                    this.props.onChange(this.state.index, createRoleListEntry_Any());
                } else {
                    this.props.onChange(
                        this.state.index,
                        createRoleListEntry_Faction(e.target.value)
                    );
                }
                this.setState({ faction: e.target.value });
            }}
        >
            {this.allFactions().map((faction) => {
                return <option key={faction}>{faction}</option>;
            }).concat([<option key={"Any"}>Any</option>])}
        </select>

        {this.state.faction !== "Any" && (<select
            className="dropdown"
            defaultValue={this.state.alignment}
            onChange={(e) => {
                if (Object.keys(ROLES).includes(e.target.value)) {
                    this.props.onChange(
                        this.state.index,
                        createRoleListEntry_Exact(e.target.value)
                    );
                } else if ("Random" === e.target.value) {
                    this.props.onChange(
                        this.state.index,
                        createRoleListEntry_Faction(this.state.faction)
                    );
                } else {
                    this.props.onChange(
                        this.state.index,
                        createRoleListEntry_FactionAlignment(
                            this.state.faction,
                            e.target.value
                        )
                    );
                }
                this.setState({ alignment: e.target.value });
            }}
        >
            {this.allAlignments(this.state.faction).map((alignment) => {
                return <option key={alignment}>{alignment}</option>;
            }).concat([<option key={"Random"}>Random</option>])}
        </select>)}
    </div>);}
}

function createRoleListEntry_Exact(role){
    return {
        "Exact":role
    };
}
function createRoleListEntry_FactionAlignment(faction, alignment){
    return {
        "FactionAlignment":faction+alignment
    };
}
function createRoleListEntry_Faction(faction){
    return {
        "Faction":faction
    };
}
function createRoleListEntry_Any(){
    return "Any";
}


  