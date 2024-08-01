import React from "react"
import GAME_MANAGER from "../../../.."
import { Role } from "../../../../game/roleState.d"
import translate from "../../../../game/lang"
import ROLES from "../../../../resources/roles.json";
import "./largeForgerMenu.css"
import { Button } from "../../../../components/Button";
import Icon from "../../../../components/Icon";

type LargeForgerMenuProps = {
}
type LargeForgerMenuState = {
    localRole: Role | null,
    savedRole: Role | null,
    localWill: string,
    savedWill: string,
    forgesRemaining: number,
}
export default class LargeForgerMenu extends React.Component<LargeForgerMenuProps, LargeForgerMenuState> {
    listener: () => void;
    constructor(props: LargeForgerMenuState) {
        super(props);

        if(
            GAME_MANAGER.state.stateType === "game" && 
            GAME_MANAGER.state.clientState.type === "player" && 
            GAME_MANAGER.state.clientState.roleState?.type === "forger"
        )
            this.state = {
                localRole: GAME_MANAGER.state.clientState.roleState?.fakeRole,
                savedRole: GAME_MANAGER.state.clientState.roleState?.fakeRole,
                localWill: GAME_MANAGER.state.clientState.roleState?.fakeWill,
                savedWill: GAME_MANAGER.state.clientState.roleState?.fakeWill,
                forgesRemaining: GAME_MANAGER.state.clientState.roleState?.forgesRemaining,
            };
        this.listener = ()=>{
            if(
                GAME_MANAGER.state.stateType === "game" &&
                GAME_MANAGER.state.clientState.type === "player" &&
                GAME_MANAGER.state.clientState.roleState?.type === "forger"
            ){
                this.setState({
                    savedWill: GAME_MANAGER.state.clientState.roleState.fakeWill,
                    savedRole: GAME_MANAGER.state.clientState.roleState.fakeRole,
                    forgesRemaining: GAME_MANAGER.state.clientState.roleState.forgesRemaining,
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
    handleSave(){
        let role = this.state.localRole;
        let will = this.state.localWill;

        if(will === ""){
            will = "ROLE\nNight 1: \nNight 2:";
        }

        this.setState({
            localWill: will,
        });

        GAME_MANAGER.sendSetForgerWill(role, will);
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
                    value={this.state.localRole?this.state.localRole:"none"} 
                    onChange={(e)=>{
                        this.setState({localRole: e.target.value as Role});
                    }}>
                    {forgerRoleOptions}
                </select>
                <div>
                    <Button
                        highlighted={this.state.localWill !== this.state.savedWill || this.state.localRole !== this.state.savedRole}
                        onClick={() => {
                            this.handleSave();
                            return true;
                        }}
                        pressedChildren={() => <Icon>done</Icon>}
                    >
                        <Icon>save</Icon>
                    </Button>
                    <Button
                        onClick={() => {
                            this.handleSend()
                            return true;
                        }}
                        pressedChildren={() => <Icon>done</Icon>}
                    >
                        <Icon>send</Icon>
                    </Button>
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