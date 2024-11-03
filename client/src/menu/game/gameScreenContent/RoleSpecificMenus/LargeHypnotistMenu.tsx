import React from "react"
import GAME_MANAGER from "../../../.."
import translate from "../../../../game/lang"
import "./largeHypnotistMenu.css"
import ChatElement from "../../../../components/ChatMessage"
import StyledText from "../../../../components/StyledText"
import CheckBox from "../../../../components/CheckBox"

export type Hypnotist = {
    roleblock: boolean,
    
    youWereRoleblockedMessage: boolean,
    youSurvivedAttackMessage: boolean,
    youWereProtectedMessage: boolean,
    youWereTransportedMessage: boolean,
    youWerePossessedMessage: boolean,
    yourTargetWasJailedMessage: boolean
}

type LargeHypnotistMenuProps = {
}
type LargeHypnotistMenuState = {
    roleblock: boolean,
    
    youWereRoleblockedMessage: boolean,
    youSurvivedAttackMessage: boolean,
    youWereProtectedMessage: boolean,
    youWereTransportedMessage: boolean,
    youWerePossessedMessage: boolean,
    yourTargetWasJailedMessage: boolean
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
                youWereProtectedMessage: GAME_MANAGER.state.clientState.roleState?.youWereProtectedMessage?? false,
                youWereTransportedMessage: GAME_MANAGER.state.clientState.roleState?.youWereTransportedMessage?? false,
                youWerePossessedMessage: GAME_MANAGER.state.clientState.roleState?.youWerePossessedMessage?? false,
                yourTargetWasJailedMessage: GAME_MANAGER.state.clientState.roleState?.yourTargetWasJailedMessage?? false
            };
        this.listener = ()=>{
            if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player" && GAME_MANAGER.state.clientState.roleState?.type === "hypnotist"){
                this.setState({
                    roleblock: GAME_MANAGER.state.clientState.roleState?.roleblock,
                
                    youWereRoleblockedMessage: GAME_MANAGER.state.clientState.roleState?.youWereRoleblockedMessage?? false,
                    youSurvivedAttackMessage: GAME_MANAGER.state.clientState.roleState?.youSurvivedAttackMessage?? false,
                    youWereProtectedMessage: GAME_MANAGER.state.clientState.roleState?.youWereProtectedMessage?? false,
                    youWereTransportedMessage: GAME_MANAGER.state.clientState.roleState?.youWereTransportedMessage?? false,
                    youWerePossessedMessage: GAME_MANAGER.state.clientState.roleState?.youWerePossessedMessage?? false,
                    yourTargetWasJailedMessage: GAME_MANAGER.state.clientState.roleState?.yourTargetWasJailedMessage?? false
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
            this.state.youWereProtectedMessage, 
            this.state.youWereTransportedMessage, 
            this.state.youWerePossessedMessage, 
            this.state.yourTargetWasJailedMessage
        );
    }
    handleYouWereRoleblockedMessageToggle(){
        GAME_MANAGER.sendSetConsortOptions(
            this.state.roleblock, 
            !this.state.youWereRoleblockedMessage, 
            this.state.youSurvivedAttackMessage, 
            this.state.youWereProtectedMessage, 
            this.state.youWereTransportedMessage, 
            this.state.youWerePossessedMessage, 
            this.state.yourTargetWasJailedMessage
        );
    }
    handleYouSurvivedAttackMessageToggle(){
        GAME_MANAGER.sendSetConsortOptions(
            this.state.roleblock, 
            this.state.youWereRoleblockedMessage, 
            !this.state.youSurvivedAttackMessage, 
            this.state.youWereProtectedMessage, 
            this.state.youWereTransportedMessage, 
            this.state.youWerePossessedMessage, 
            this.state.yourTargetWasJailedMessage
        );
    }
    handleYouWereProtectedMessageToggle(){
        GAME_MANAGER.sendSetConsortOptions(
            this.state.roleblock, 
            this.state.youWereRoleblockedMessage, 
            this.state.youSurvivedAttackMessage, 
            !this.state.youWereProtectedMessage, 
            this.state.youWereTransportedMessage, 
            this.state.youWerePossessedMessage, 
            this.state.yourTargetWasJailedMessage
        );
    }
    handleYouWereTransportedMessageToggle(){
        GAME_MANAGER.sendSetConsortOptions(
            this.state.roleblock, 
            this.state.youWereRoleblockedMessage, 
            this.state.youSurvivedAttackMessage, 
            this.state.youWereProtectedMessage, 
            !this.state.youWereTransportedMessage, 
            this.state.youWerePossessedMessage, 
            this.state.yourTargetWasJailedMessage
        );
    }
    handleYouWerePossessedMessageToggle(){
        GAME_MANAGER.sendSetConsortOptions(
            this.state.roleblock, 
            this.state.youWereRoleblockedMessage, 
            this.state.youSurvivedAttackMessage, 
            this.state.youWereProtectedMessage, 
            this.state.youWereTransportedMessage, 
            !this.state.youWerePossessedMessage, 
            this.state.yourTargetWasJailedMessage
        );
    }
    handleYourTargetWasJailedMessageToggle(){
        GAME_MANAGER.sendSetConsortOptions(
            this.state.roleblock, 
            this.state.youWereRoleblockedMessage, 
            this.state.youSurvivedAttackMessage, 
            this.state.youWereProtectedMessage, 
            this.state.youWereTransportedMessage, 
            this.state.youWerePossessedMessage, 
            !this.state.yourTargetWasJailedMessage
        );
    }


    render(){
        return <div className="large-hypnotist-menu">
            <div>
                <StyledText>
                    {translate("wiki.article.standard.roleblock.title")}
                </StyledText>
                <CheckBox checked={this.state.roleblock} onChange={(_)=>this.handleRoleblockToggle()}/>
            </div>
            <div>
                <ChatElement message={{
                    variant: {
                        type: "roleBlocked",
                        immune: false,
                    },
                    chatGroup:null
                }}/>
                <ChatElement message={{
                    variant: {
                        type: "roleBlocked",
                        immune: true,
                    },
                    chatGroup:null
                }}/>
                <CheckBox checked={this.state.youWereRoleblockedMessage} onChange={(_)=>this.handleYouWereRoleblockedMessageToggle()}/>
            </div>
            <div>
                <ChatElement message={{
                    variant: {
                        type: "youSurvivedAttack",
                    },
                    chatGroup:null
                }}/>
                <CheckBox checked={this.state.youSurvivedAttackMessage} onChange={(_)=>this.handleYouSurvivedAttackMessageToggle()}/>
            </div>
            <div>
                <ChatElement message={{
                    variant: {
                        type: "youWereProtected",
                    },
                    chatGroup:null
                }}/>
                <CheckBox checked={this.state.youWereProtectedMessage} onChange={(_)=>this.handleYouWereProtectedMessageToggle()}/>
            </div>
            <div>
                <ChatElement message={{
                    variant: {
                        type: "transported",
                    },
                    chatGroup:null
                }}/>
                <CheckBox checked={this.state.youWereTransportedMessage} onChange={(_)=>this.handleYouWereTransportedMessageToggle()}/>
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
                <CheckBox checked={this.state.youWerePossessedMessage} onChange={(_)=>this.handleYouWerePossessedMessageToggle()}/>
            </div>
            <div>
                <ChatElement message={{
                    variant: {
                        type: "wardblocked",
                    },
                    chatGroup:null
                }}/>
                <CheckBox checked={this.state.yourTargetWasJailedMessage} onChange={(_)=>this.handleYourTargetWasJailedMessageToggle()}/>
            </div>            
        </div>
    }
}