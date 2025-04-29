import React from "react"
import "./largeHypnotistMenu.css"
import ChatElement from "../../../../../components/ChatMessage"
import CheckBox from "../../../../../components/CheckBox"
import StyledText from "../../../../../components/StyledText"
import GAME_MANAGER from "../../../../.."
import translate from "../../../../../game/lang"

export type Hypnotist = {
    roleblock: boolean,
    
    youWereRoleblockedMessage: boolean,
    youSurvivedAttackMessage: boolean,
    youWereGuardedMessage: boolean,
    youWereTransportedMessage: boolean,
    youWerePossessedMessage: boolean,
    youWereWardblockedMessage: boolean
}

type LargeHypnotistMenuProps = {
}
type LargeHypnotistMenuState = {
    roleblock: boolean,
    
    youWereRoleblockedMessage: boolean,
    youSurvivedAttackMessage: boolean,
    youWereGuardedMessage: boolean,
    youWereTransportedMessage: boolean,
    youWerePossessedMessage: boolean,
    youWereWardblockedMessage: boolean
}
export default class LargeHypnotistMenu extends React.Component<LargeHypnotistMenuProps, LargeHypnotistMenuState> {
    listener: () => void;
    constructor(props: LargeHypnotistMenuState) {
        super(props);

        if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player" && GAME_MANAGER.state.clientState.roleState?.type === "hypnotist")
            this.state = {
                roleblock: GAME_MANAGER.state.clientState.roleState?.roleblock,
                
                youWereRoleblockedMessage: GAME_MANAGER.state.clientState.roleState?.youWereRoleblockedMessage?? false,
                youSurvivedAttackMessage: GAME_MANAGER.state.clientState.roleState?.youSurvivedAttackMessage?? false,
                youWereGuardedMessage: GAME_MANAGER.state.clientState.roleState?.youWereGuardedMessage?? false,
                youWereTransportedMessage: GAME_MANAGER.state.clientState.roleState?.youWereTransportedMessage?? false,
                youWerePossessedMessage: GAME_MANAGER.state.clientState.roleState?.youWerePossessedMessage?? false,
                youWereWardblockedMessage: GAME_MANAGER.state.clientState.roleState?.youWereWardblockedMessage?? false
            };
        this.listener = ()=>{
            if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player" && GAME_MANAGER.state.clientState.roleState?.type === "hypnotist"){
                this.setState({
                    roleblock: GAME_MANAGER.state.clientState.roleState?.roleblock,
                
                    youWereRoleblockedMessage: GAME_MANAGER.state.clientState.roleState?.youWereRoleblockedMessage?? false,
                    youSurvivedAttackMessage: GAME_MANAGER.state.clientState.roleState?.youSurvivedAttackMessage?? false,
                    youWereGuardedMessage: GAME_MANAGER.state.clientState.roleState?.youWereGuardedMessage?? false,
                    youWereTransportedMessage: GAME_MANAGER.state.clientState.roleState?.youWereTransportedMessage?? false,
                    youWerePossessedMessage: GAME_MANAGER.state.clientState.roleState?.youWerePossessedMessage?? false,
                    youWereWardblockedMessage: GAME_MANAGER.state.clientState.roleState?.youWereWardblockedMessage?? false
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

    handleRoleblockToggle(){
        GAME_MANAGER.sendSetConsortOptions(
            !this.state.roleblock, 
            this.state.youWereRoleblockedMessage, 
            this.state.youSurvivedAttackMessage, 
            this.state.youWereGuardedMessage, 
            this.state.youWereTransportedMessage, 
            this.state.youWerePossessedMessage, 
            this.state.youWereWardblockedMessage
        );
    }
    handleYouWereRoleblockedMessageToggle(){
        GAME_MANAGER.sendSetConsortOptions(
            this.state.roleblock, 
            !this.state.youWereRoleblockedMessage, 
            this.state.youSurvivedAttackMessage, 
            this.state.youWereGuardedMessage, 
            this.state.youWereTransportedMessage, 
            this.state.youWerePossessedMessage, 
            this.state.youWereWardblockedMessage
        );
    }
    handleYouSurvivedAttackMessageToggle(){
        GAME_MANAGER.sendSetConsortOptions(
            this.state.roleblock, 
            this.state.youWereRoleblockedMessage, 
            !this.state.youSurvivedAttackMessage, 
            this.state.youWereGuardedMessage, 
            this.state.youWereTransportedMessage, 
            this.state.youWerePossessedMessage, 
            this.state.youWereWardblockedMessage
        );
    }
    handleYouWereGuardedMessageToggle(){
        GAME_MANAGER.sendSetConsortOptions(
            this.state.roleblock, 
            this.state.youWereRoleblockedMessage, 
            this.state.youSurvivedAttackMessage, 
            !this.state.youWereGuardedMessage, 
            this.state.youWereTransportedMessage, 
            this.state.youWerePossessedMessage, 
            this.state.youWereWardblockedMessage
        );
    }
    handleYouWereTransportedMessageToggle(){
        GAME_MANAGER.sendSetConsortOptions(
            this.state.roleblock, 
            this.state.youWereRoleblockedMessage, 
            this.state.youSurvivedAttackMessage, 
            this.state.youWereGuardedMessage, 
            !this.state.youWereTransportedMessage, 
            this.state.youWerePossessedMessage, 
            this.state.youWereWardblockedMessage
        );
    }
    handleYouWerePossessedMessageToggle(){
        GAME_MANAGER.sendSetConsortOptions(
            this.state.roleblock, 
            this.state.youWereRoleblockedMessage, 
            this.state.youSurvivedAttackMessage, 
            this.state.youWereGuardedMessage, 
            this.state.youWereTransportedMessage, 
            !this.state.youWerePossessedMessage, 
            this.state.youWereWardblockedMessage
        );
    }
    handleYourTargetWasJailedMessageToggle(){
        GAME_MANAGER.sendSetConsortOptions(
            this.state.roleblock, 
            this.state.youWereRoleblockedMessage, 
            this.state.youSurvivedAttackMessage, 
            this.state.youWereGuardedMessage, 
            this.state.youWereTransportedMessage, 
            this.state.youWerePossessedMessage, 
            !this.state.youWereWardblockedMessage
        );
    }


    render(){
        return <div className="large-hypnotist-menu">
            <div>
                <StyledText>
                    {translate("wiki.article.standard.roleblock.title")}
                </StyledText>
                <CheckBox checked={this.state.roleblock} onChange={(checked)=>this.handleRoleblockToggle()}/>
            </div>
            <div>
                <ChatElement message={{
                    variant: {
                        type: "roleBlocked",
                    },
                    chatGroup:null
                }}/>
                <CheckBox checked={this.state.youWereRoleblockedMessage} onChange={(checked)=>this.handleYouWereRoleblockedMessageToggle()}/>
            </div>
            <div>
                <ChatElement message={{
                    variant: {
                        type: "youSurvivedAttack",
                    },
                    chatGroup:null
                }}/>
                <CheckBox checked={this.state.youSurvivedAttackMessage} onChange={(checked)=>this.handleYouSurvivedAttackMessageToggle()}/>
            </div>
            <div>
                <ChatElement message={{
                    variant: {
                        type: "youWereGuarded",
                    },
                    chatGroup:null
                }}/>
                <CheckBox checked={this.state.youWereGuardedMessage} onChange={(checked)=>this.handleYouWereGuardedMessageToggle()}/>
            </div>
            <div>
                <ChatElement message={{
                    variant: {
                        type: "transported",
                    },
                    chatGroup:null
                }}/>
                <CheckBox checked={this.state.youWereTransportedMessage} onChange={(checked)=>this.handleYouWereTransportedMessageToggle()}/>
            </div>
            <div>
                <ChatElement message={{
                    variant: {
                        type: "youWerePossessed",
                        immune: false,
                    },
                    chatGroup:null
                }}/>
                <ChatElement message={{
                    variant: {
                        type: "youWerePossessed",
                        immune: true,
                    },
                    chatGroup:null
                }}/>
                <CheckBox checked={this.state.youWerePossessedMessage} onChange={(checked)=>this.handleYouWerePossessedMessageToggle()}/>
            </div>
            <div>
                <ChatElement message={{
                    variant: {
                        type: "wardblocked",
                    },
                    chatGroup:null
                }}/>
                <CheckBox checked={this.state.youWereWardblockedMessage} onChange={(checked)=>this.handleYourTargetWasJailedMessageToggle()}/>
            </div>            
        </div>
    }
}