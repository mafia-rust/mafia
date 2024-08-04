import React from "react"
import GAME_MANAGER from "../../../.."
import translate from "../../../../game/lang"
import "./largeConsortMenu.css"
import ChatElement from "../../../../components/ChatMessage"
import Icon from "../../../../components/Icon"

export type Hypnotist = {
    roleblock: boolean,
    
    youWereRoleblockedMessage: boolean,
    youSurvivedAttackMessage: boolean,
    youWereProtectedMessage: boolean,
    youWereTransportedMessage: boolean,
    youWerePossessedMessage: boolean,
    yourTargetWasJailedMessage: boolean
}

type LargeConsortMenuProps = {
}
type LargeConsortMenuState = {
    roleblock: boolean,
    
    youWereRoleblockedMessage: boolean,
    youSurvivedAttackMessage: boolean,
    youWereProtectedMessage: boolean,
    youWereTransportedMessage: boolean,
    youWerePossessedMessage: boolean,
    yourTargetWasJailedMessage: boolean
}
export default class LargeConsortMenu extends React.Component<LargeConsortMenuProps, LargeConsortMenuState> {
    listener: () => void;
    constructor(props: LargeConsortMenuState) {
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
                
                {translate("wiki.article.standard.roleblock.title")}
                <label onClick={()=>this.handleRoleblockToggle()}>
                    <Icon>{this.state.roleblock ? "check" : "close"}</Icon>
                </label>

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
                <label onClick={()=>this.handleYouWereRoleblockedMessageToggle()}>
                    <Icon>{this.state.youWereRoleblockedMessage ? "check" : "close"}</Icon>
                </label>
                
            </div>
            <div>
                <ChatElement message={{
                    variant: {
                        type: "youSurvivedAttack",
                    },
                    chatGroup:null
                }}/>
                <label onClick={()=>this.handleYouSurvivedAttackMessageToggle()}>
                    <Icon>{this.state.youSurvivedAttackMessage ? "check" : "close"}</Icon>
                </label>
                
            </div>
            <div>
                <ChatElement message={{
                    variant: {
                        type: "youWereProtected",
                    },
                    chatGroup:null
                }}/>
                <label onClick={()=>this.handleYouWereProtectedMessageToggle()}>
                    <Icon>{this.state.youWereProtectedMessage ? "check" : "close"}</Icon>
                </label>
                
            </div>
            <div>
                <ChatElement message={{
                    variant: {
                        type: "transported",
                    },
                    chatGroup:null
                }}/>
                <label onClick={()=>this.handleYouWereTransportedMessageToggle()}>
                    <Icon>{this.state.youWereTransportedMessage ? "check" : "close"}</Icon>
                </label>
                
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
                <label onClick={()=>this.handleYouWerePossessedMessageToggle()}>
                    <Icon>{this.state.youWerePossessedMessage ? "check" : "close"}</Icon>
                </label>
                
            </div>
            <div>
                <ChatElement message={{
                    variant: {
                        type: "wardblocked",
                    },
                    chatGroup:null
                }}/>
                <label onClick={()=>this.handleYourTargetWasJailedMessageToggle()}>
                    <Icon>{this.state.yourTargetWasJailedMessage ? "check" : "close"}</Icon>
                </label>
            </div>            
        </div>
    }
}