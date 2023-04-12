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
            faction: props.faction !== undefined ? props.faction : "Any",
            alignment: props.alignment !== undefined ? props.alignment : "Random",
            role: "Any",
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
            //if (!roles.includes(role)) {
            //    roles.push(role);
            //}
        }

        return alignments.concat(roles);
    }

    allRoles(faction, alignment) {
        let roles = [];

        for (let role in ROLES) {
            if (ROLES[role].faction !== faction) continue;
            if (ROLES[role].alignment !== alignment && alignment !== "Random") continue;

            roles.push(role);
        }

        return roles;
    }
    render() {
        const { faction, alignment, role } = this.state;
        const factions = this.allFactions();
        const alignments = this.allAlignments(faction);
      
        const isAlignmentVisible = faction !== "Any";
        const isRoleVisible = alignment !== "Random";
      
        return (
          <div>
            <div className="role-picker-container">
              <select
                className="dropdown"
                value={faction}
                onChange={(e) => {
                  const newFaction = e.target.value;
                  const newAlignment = newFaction !== "Any" ? "Random" : "Any";
                  const newRole = "Random"; // Reset role dropdown to "Random" when faction or alignment changes
      
                  this.setState(
                    { faction: newFaction, alignment: newAlignment, role: newRole },
                    () => this.state.onChange(this.state.index, this.state)
                  );
                }}
              >
                <option value="Any">Any</option>
                {factions.map((faction, i) => (
                  <option key={i} value={faction}>
                    {faction}
                  </option>
                ))}
              </select>
      
              {isAlignmentVisible && (
                <select
                  className="dropdown"
                  value={alignment}
                  onChange={(e) => {
                    const newAlignment = e.target.value;
                    const newRole = "Random"; // Reset role dropdown to "Random" when alignment changes
      
                    this.setState(
                      { alignment: newAlignment, role: newRole },
                      () => this.state.onChange(this.state.index, this.state)
                    );
                  }}
                >
                  <option value="Random">Random</option>
                  {alignments.map((alignment, i) => (
                    <option key={i} value={alignment}>
                      {alignment}
                    </option>
                  ))}
                </select>
              )}
      
              {isRoleVisible && (
                <select
                  className="dropdown"
                  value={role}
                  onChange={(e) => {
                    const newRole = e.target.value;
      
                    this.setState({ role: newRole }, () =>
                      this.state.onChange(this.state.index, this.state)
                    );
                  }}
                >
                  <option value="Random">Random</option>
                  {this.allRoles(faction, alignment).map((role, i) => (
                    <option key={i} value={role}>
                      {role}
                    </option>
                  ))}
                </select>
              )}
            </div>
      
            <div>{this.state.rolePickers}</div>
          </div>
        );
      }
                      
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