import React from "react"
import GAME_MANAGER from "../../../.."
import { Role } from "../../../../game/roleState.d"
import translate from "../../../../game/lang"
import ROLES from "../../../../resources/roles.json";
import "./largeForgerMenu.css"

type LargeForgerMenuProps = {
}
type LargeForgerMenuState = {
    localRole: Role,
    savedRole: Role,
    localWill: string,
    savedWill: string,
    forgesRemaining: number,
}
export default class LargeForgerMenu extends React.Component<LargeForgerMenuProps, LargeForgerMenuState> {
    listener: () => void;
    constructor(props: LargeForgerMenuState) {
        super(props);

        if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.roleState?.role === "forger")
            this.state = {
                localRole: GAME_MANAGER.state.roleState?.fakeRole,
                savedRole: GAME_MANAGER.state.roleState?.fakeRole,
                localWill: GAME_MANAGER.state.roleState?.fakeWill,
                savedWill: GAME_MANAGER.state.roleState?.fakeWill,
                forgesRemaining: GAME_MANAGER.state.roleState?.forgesRemaining,
            };
        this.listener = ()=>{
            if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.roleState?.role === "forger"){
                this.setState({
                    savedWill: GAME_MANAGER.state.roleState.fakeWill,
                    savedRole: GAME_MANAGER.state.roleState.fakeRole,
                    forgesRemaining: GAME_MANAGER.state.roleState.forgesRemaining,
                })
            }
        };  
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }
    handleSave(){
        GAME_MANAGER.sendSetForgerWill(this.state.localRole, this.state.localWill);
    }
    handleSend(){
        GAME_MANAGER.sendSendMessagePacket('\n' + this.state.savedWill);
    }

    render(){

        let forgerRoleOptions: JSX.Element[] = [];
        forgerRoleOptions.push(
            <option key={"none"} value={"none"}>{translate("none")}</option>
        );
        for(let role of Object.keys(ROLES)){
            forgerRoleOptions.push(
                <option key={role} value={role}>{translate("role."+role+".name")}</option>
            );
        }

        return <div className="large-forger-menu">
            <div>                
                <select
                    value={this.state.localRole} 
                    onChange={(e)=>{
                        this.setState({localRole: e.target.value as Role});
                    }}>
                    {forgerRoleOptions}
                </select>
                <div>
                    <button
                        className={"material-icons-round " + (this.state.localWill !== this.state.savedWill || this.state.localRole !== this.state.savedRole ? "highlighted" : "")}
                        onClick={() => this.handleSave()}
                    >
                        save
                    </button>
                    <button
                        className={"material-icons-round"}
                        onClick={() => this.handleSend()}
                    >
                        send
                    </button>
                </div>
            </div>
            <textarea
                value={this.state.localWill}
                onChange={(e) => {
                    this.setState({ localWill: e.target.value });
                }}
                onKeyDown={(e) => {
                    if (e.ctrlKey) {
                        if (e.key === 's') {
                            e.preventDefault();
                            this.handleSave();
                        } else if (e.key === "Enter") {
                            this.handleSave();
                        }
                    }
                }}>
            </textarea>
            <div>
                {translate("role.forger.menu.forgesRemaining", this.state.forgesRemaining ?? 0)}
            </div>
        </div>
    }
}