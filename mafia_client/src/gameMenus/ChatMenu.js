import React from "react";
import gameManager from "../index.js";

export class ChatMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            gameState : gameManager.gameState,
            chatFeild: "",
        };
        this.listener = ()=>{
            this.setState({
                gameState: gameManager.gameState
            });
        };
    }
    componentDidMount() {
        gameManager.addStateListener(this.listener);
    }
    componentWillUnmount() {
        gameManager.removeStateListener(this.listener);
    }

    //helper functions
    sendChatFeild() {
        let text = this.state.chatFeild.trim();
        if(text.startsWith("/w")){
            try{
                let playerIndex = Number(text[2])-1;
                text = text.substring(3);
                gameManager.sendWhisper_button(playerIndex, text);
            }catch(e){
                gameManager.sendMessage_button(text);
            }
        }else{
            gameManager.sendMessage_button(text);
        }
        this.setState({chatFeild:""});
    }

    //these next functions are old
    componentDidUpdate() {
        if(this.bottomIsInViewport(500))   //used to be 500
            this.scrollToBottom();
    }
    scrollToBottom() {
        this.buttomOfPage.scrollIntoView({ behavior: "smooth" });
    }
    bottomIsInViewport(offset = 0) {
        if (!this.buttomOfPage) return false;
        const top = this.buttomOfPage.getBoundingClientRect().top;
        //if top is between 0 and height then true
        //else false
        return (top + offset) >= 0 && (top - offset) <= window.innerHeight;
    }

    renderTextInput(){return(<div style={{position: "sticky", bottom: 10}}>
        <input value={this.state.chatFeild} onChange={(e)=>{this.setState({chatFeild: e.target.value.trimStart()})}} 
            onKeyPress={(e) => {
            if(e.code === "Enter") {
                this.sendChatFeild();
            }
        }}></input>
        <button onClick={()=>{
            this.sendChatFeild();
        }}>Send</button>
    </div>);}
    renderChatMessage(msg, i) {
        return(<div key={i} style={{textAlign:"left"}}>
            {getChatString(msg)}
        </div>);
    }
    render(){return(<div>
        {this.state.gameState.chatMessages.map((msg, i)=>{
            return this.renderChatMessage(msg, i);
        }, this)}

        <br ref={(el) => { this.buttomOfPage = el; }}/>
            {this.renderTextInput()}
        <br/>
        <br/>
        <br/>
    </div>)}
}




function getChatString(message) {
    // console.log(message);
    if(message.Normal !== undefined){
        if(message.Normal.message_sender.Player !== undefined){
            return "("+(message.Normal.message_sender.Player+1)+")"+
            gameManager.gameState.players[message.Normal.message_sender.Player].name+": "+
            message.Normal.text;
        }
    }
    if(message.Whisper !== undefined){
        return ""+
        "From ("+(message.Whisper.from_player_index+1)+")"+
        gameManager.gameState.players[message.Whisper.from_player_index].name+
        " to ("+
        (message.Whisper.to_player_index+1)+")"+
        gameManager.gameState.players[message.Whisper.to_player_index].name+": "+
        message.Whisper.text;
    }
    if(message.BroadcastWhisper!==undefined){
        return ""+
        "("+(message.BroadcastWhisper.whisperer+1)+")"+
        gameManager.gameState.players[message.BroadcastWhisper.whisperer].name+
        " is whispering to ("+
        (message.BroadcastWhisper.whisperee+1)+")"+
        gameManager.gameState.players[message.BroadcastWhisper.whisperee].name;
    }
    if(message.PhaseChange!==undefined){
        return ""+
        message.PhaseChange.phase_type+" "+
        message.PhaseChange.day_number;
    }
    if(message.TrialInformation!==undefined){
        return "You need "+
        message.TrialInformation.required_votes+
        " votes to put someone on trial. There's "+
        message.TrialInformation.trials_left+" trials left today.";
    }
    if(message.NightInformation!==undefined){
        return getNightInformationString(message.NightInformation.night_information);
    }
    if (message.MayorRevealed !== undefined) {
        return "" +
        gameManager.gameState.players[message.MayorRevealed.player_index].name+
        " has revealed as mayor!";
    }
    if (message.MayorCantWhisper !== undefined) {
        return "You can't whisper as or to a revealed mayor!";
    }
    if (message.JailorDecideExecuteYou !== undefined) {
        return "Jailor has decided to execute you!";
    }
    if (message.MediumSeanceYou !== undefined) {
        return "You are being seanced by the medium!";
    }
    
    return JSON.stringify(message);
}
function getNightInformationString(message){
    if (message.RoleBlocked !== undefined) {
        if (message.RoleBlocked.immune) {
            return "Someone tried to roleblock you but you are immune.";
        } else {
            return "You have been roleblocked.";
        }
    }
    if (message === "TargetSurvivedAttack") {
        return "Your target had defense and survived.";
    }
    if (message === "YouSurvivedAttack") {
        return "You had defense and survived an attack.";
    }
    if (message === "YouDied") {
        return "You died!";
    }
    if (message === "VeteranAttackedYou") {
        return "You were attacked by the veteran you visited.";
    }
    if (message === "VeteranAttackedVisitor") {
        return "You attacked a visitor.";
    }
    if (message === "VigilanteSuicide") {
        return "You committed suicide over the guilt of killing an innocnet person";
    }
    if (message === "DoctorHealed") {
        return "You healed your target while they got attacked.";
    }
    if (message === "DoctorHealedYou") {
        return "A doctor healed you while you got attacked.";
    }
    if (message.SheriffResult !== undefined) {
        if(message.SheriffResult.suspicious)
            return "Your target seems to be suspicious.";
        return "Your target seems to be innocent.";
    }

    return JSON.stringify(message);
}


